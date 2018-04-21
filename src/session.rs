use super::{Result, ResultExt};
use super::SyncthingMessage;
use super::syncthing_proto;
use super::util;
use std::io;
use std::net::TcpStream;
use std::sync::Arc;

use byteorder::{ByteOrder, NetworkEndian};
use compress;
use protobuf;
use protobuf::Message as ProtobufMessage;
use ring;
use rustls;

const HELLO_MAGIC: u32 = 0x2ea7_d90b;

pub struct Session {
    tls: rustls::ClientSession,
    stream: TcpStream,
    device_name: String,
    next_request_id: i32,
}

impl Session {
    pub fn write_hello(&mut self) -> Result<()> {
        let mut output = protobuf::CodedOutputStream::new(&mut self.tls);

        let mut magic = [0u8;4];
        NetworkEndian::write_u32(&mut magic, HELLO_MAGIC);
        output.write_raw_bytes(&magic)?;

        let mut hello = syncthing_proto::Hello::new();
        hello.set_device_name(self.device_name.clone());
        hello.set_client_name(env!("CARGO_PKG_NAME").to_owned());
        hello.set_client_version(env!("CARGO_PKG_VERSION").to_owned());

        let mut len = [0u8;2];
        NetworkEndian::write_u16(&mut len, hello.compute_size() as u16);
        output.write_raw_bytes(&len)?;

        hello.write_to_with_cached_sizes(&mut output)?;
        output.flush()?;

        Ok(())
    }

    pub fn read_hello(buf: &[u8]) -> Result<(usize, syncthing_proto::Hello)> {
        let mut input = protobuf::CodedInputStream::from_bytes(buf);

        let magic = NetworkEndian::read_u32(&input.read_raw_bytes(4)?);
        if magic != HELLO_MAGIC {
            bail!("incorrect magic number: {:#x} (expected {:#x})", magic, HELLO_MAGIC);
        }

        let len = NetworkEndian::read_u16(&input.read_raw_bytes(2)?);
        input.push_limit(u64::from(len))?;
        debug!("hello message length specified as {:#x}; we have {:#x} bytes",
                len, buf.len() as u64 - input.pos());

        let mut hello = syncthing_proto::Hello::new();
        hello.merge_from(&mut input).chain_err(|| "error reading Hello")?;

        Ok((input.pos() as usize, hello))
    }

    pub fn read_message(buf: &[u8])
            -> Result<(usize, syncthing_proto::MessageType, Box<SyncthingMessage>)> {
        let mut input = protobuf::CodedInputStream::from_bytes(buf);

        let header_length = NetworkEndian::read_u16(&input.read_raw_bytes(2)?);
        let mut header = syncthing_proto::Header::new();
        let old_limit = input.push_limit(u64::from(header_length))?;
        header.merge_from(&mut input).chain_err(|| "error reading message header")?;
        input.pop_limit(old_limit);

        debug!("header: {:?}, compression: {:?}", header.get_field_type(), header.get_compression());

        let body_length = NetworkEndian::read_u32(&input.read_raw_bytes(4)?);
        debug!("body length = {} / {:#x}", body_length, body_length);

        let mut body_protobuf = vec![];
        match header.get_compression() {
            syncthing_proto::MessageCompression::LZ4 => {
                let uncompressed_length = NetworkEndian::read_u32(&input.read_raw_bytes(4)?);
                debug!("uncompressed length = {} / {:#x}", uncompressed_length, uncompressed_length);
                let n = compress::lz4::decode_block(&buf[input.pos() as usize ..], &mut body_protobuf);
                debug!("{} / {:#x} LZ4 bytes processed", n, n);
                if body_protobuf.len() as u32 != uncompressed_length {
                    bail!("uncompressed LZ4 data ({} bytes) doesn't match expected length ({} bytes)",
                            body_protobuf.len(), uncompressed_length);
                }
                input.skip_raw_bytes(body_length - 4)?;
            },
            syncthing_proto::MessageCompression::NONE => {
                input.read_raw_bytes_into(body_length, &mut body_protobuf)?;
            }
        }

        let mut body_input = protobuf::CodedInputStream::from_bytes(&body_protobuf);
        let mut body: Box<SyncthingMessage> = match header.get_field_type() {
            syncthing_proto::MessageType::CLUSTER_CONFIG    => Box::new(syncthing_proto::ClusterConfig::new()),
            syncthing_proto::MessageType::INDEX             => Box::new(syncthing_proto::Index::new()),
            syncthing_proto::MessageType::INDEX_UPDATE      => Box::new(syncthing_proto::IndexUpdate::new()),
            syncthing_proto::MessageType::REQUEST           => unimplemented!("message type REQUEST"),
            syncthing_proto::MessageType::RESPONSE          => Box::new(syncthing_proto::Response::new()),
            syncthing_proto::MessageType::DOWNLOAD_PROGRESS => unimplemented!("message type DOWNLOAD_PROGRESS"),
            syncthing_proto::MessageType::PING              => Box::new(syncthing_proto::Ping::new()),
            syncthing_proto::MessageType::CLOSE             => Box::new(syncthing_proto::Close::new()),
        };
        body.as_mut().as_protobuf_message().merge_from(&mut body_input)?;

        debug!("body_input pos: {}", body_input.pos());

        Ok((input.pos() as usize, header.get_field_type(), body))
    }

    pub fn write_message<T: ProtobufMessage + protobuf::MessageStatic>(
        &mut self,
        message: &T,
        message_type: syncthing_proto::MessageType,
        ) -> Result<()>
    {
        let mut output = protobuf::CodedOutputStream::new(&mut self.tls);

        let mut header = syncthing_proto::Header::new();
        header.set_compression(syncthing_proto::MessageCompression::NONE);
        header.set_field_type(message_type);

        let mut header_len = [0u8; 2];
        NetworkEndian::write_u16(&mut header_len, header.compute_size() as u16);
        output.write_raw_bytes(&header_len)?;
        header.write_to_with_cached_sizes(&mut output)?;

        let mut body_len = [0u8; 4];
        NetworkEndian::write_u32(&mut body_len, message.compute_size());
        output.write_raw_bytes(&body_len)?;
        message.write_to_with_cached_sizes(&mut output)?;

        output.flush()?;
        Ok(())
    }

    pub fn write_block_request(
        &mut self,
        folder: String,
        path: String,
        offset: i64,
        size: i32,
        hash: Vec<u8>,
        ) -> Result<i32> // returns the request ID
    {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let mut req = syncthing_proto::Request::new();
        req.set_id(request_id);
        req.set_folder(folder);
        req.set_name(path);
        req.set_offset(offset);
        req.set_size(size);
        req.set_hash(hash);
        req.set_from_temporary(false);

        self.write_message(&req, syncthing_proto::MessageType::REQUEST)?;
        Ok(request_id)
    }

    // FIXME(wfraser) only for testing
    pub fn read_to_end(&mut self, data: &mut Vec<u8>) -> Result<usize> {
        use std::io::Read;
        self.tls.read_to_end(data).map_err(|e| e.into())
    }

    // FIXME(wfraser) only for testing
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        use std::io::Write;
        self.tls.write(data).map_err(|e| e.into())
    }

    // This is basically rustls::ClientSession::complete_io() with extra logging.
    // FIXME(wfraser) only public for testing
    pub fn complete_io(&mut self) -> Result<(usize, usize)> {
        use rustls::Session;

        let handshaking = self.tls.is_handshaking();
        let mut eof = false;
        let mut wrlen = 0;
        let mut rdlen = 0;

        loop {
            while self.tls.wants_write() {
                debug!("writing");
                match self.tls.write_tls(&mut self.stream) {
                    Ok(n) => {
                        debug!("wrote {} bytes of TLS", n);
                        wrlen += n;
                    },
                    Err(e) => {
                        error!("write error: {}", e);
                        return Err(e.into());
                    }
                }
            }

            if !handshaking && wrlen > 0 {
                debug!("write completed");
                return Ok((rdlen, wrlen));
            }

            if !eof && self.tls.wants_read() {
                debug!("reading");
                match self.tls.read_tls(&mut self.stream) {
                    Ok(n) => {
                        debug!("read {} bytes of TLS", n);
                        if n == 0 {
                            eof = true;
                        }
                        rdlen += n;
                    },
                    Err(e) => {
                        error!("read error: {}", e);
                        return Err(e.into());
                    }
                }
            }

            if let Err(e) = self.tls.process_new_packets() {
                error!("error processing TLS packets: {}", e);
                return Err(e.into());
            }

            match (eof, handshaking, self.tls.is_handshaking()) {
                (_, true, false) => {
                    debug!("done handshaking");
                    return Ok((rdlen, wrlen));
                },
                (_, false, _) => {
                    debug!("read completed");
                    return Ok((rdlen, wrlen));
                },
                (true, true, true) => {
                    debug!("unexpected EOF");
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof).into());
                },
                (..) => {
                    debug!("looping again");
                }
            }
        }
    }
}

pub struct SessionBuilder {
    pub remote_host_and_port: String,
    pub remote_device_id: String,
    pub local_device_name: Option<String>,
    pub client_cert: super::Certificate,
    pub private_key: super::PrivateKey,
}

impl SessionBuilder {
    pub fn connect(self) -> Result<Session> {
        let device_name = match self.local_device_name {
            Some(name) => name,
            None => match util::get_hostname() {
                Ok(name) => {
                    debug!("no device name specified; using hostname {}", name);
                    name
                },
                Err(e) => {
                    error!("failed to get the system hostname: {}", e);
                    return Err(e).chain_err(|| "failed to get the system hostname");
                }
            }
        };
        info!("our device name is {:?}", device_name);

        let mut config = rustls::ClientConfig::new();
        config.set_single_client_cert(vec![self.client_cert], self.private_key);
        config.alpn_protocols.push("bep/1.0".to_owned());

        rustls::DangerousClientConfig { cfg: &mut config }
            .set_certificate_verifier(
                Arc::new(SyncthingCertVerifier::new(self.remote_device_id)));

        let host_and_port = &self.remote_host_and_port;
        let stream = TcpStream::connect(host_and_port).map_err(|e| {
            error!("failed to connect to {}: {}", host_and_port, e);
            e
        })?;

        Ok(Session {
            tls: rustls::ClientSession::new(&Arc::new(config), "syncthing"),
            stream,
            device_name,
            next_request_id: 0,
        })
    }
}

struct SyncthingCertVerifier {
    device_id: String,
}

impl SyncthingCertVerifier {
    pub fn new(device_id: String) -> SyncthingCertVerifier {
        SyncthingCertVerifier {
            device_id,
        }
    }
}

impl rustls::ServerCertVerifier for SyncthingCertVerifier {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        presented_certs: &[rustls::Certificate],
        _dns_name: &str)
        -> ::std::result::Result<(), rustls::TLSError>
    {
        use rustls::internal::msgs::codec::Codec;
        debug!("Checking device ID. Server presented {} certificates", presented_certs.len());
        for (i, cert) in presented_certs.iter().enumerate() {
            let cert_bytes = cert.get_encoding();
            let mut hash_ctx = ring::digest::Context::new(&ring::digest::SHA256);
            hash_ctx.update(&cert_bytes[3..]);
            let digest = hash_ctx.finish();
            debug!("{}: cert hash is {:?}", i, digest);
            let device_id = util::device_id_from_hash(digest.as_ref());
            debug!("{}: device ID {}", i, device_id);
            if device_id == self.device_id {
                debug!("{}: matches", i);
                return Ok(());
            } else {
                warn!("{}: mismatch", i);
            }
        }
        error!("none of the presented server certificates have the expected Device ID");
        Err(rustls::TLSError::General("Syncthing device ID mismatch".to_owned()))
    }
}
