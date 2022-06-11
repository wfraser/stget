use anyhow::{bail, Context, Result};
use crate::SyncthingMessage;
use crate::syncthing_proto;
use crate::util;
use std::io;
use std::net::TcpStream;
use std::sync::Arc;

use byteorder::{ByteOrder, NetworkEndian};
use lz4_compression;
use protobuf;
use protobuf::Message as ProtobufMessage;
use ring;
use rustls;

const HELLO_MAGIC: u32 = 0x2ea7_d90b;

pub struct Session {
    tls: rustls::ClientConnection,
    stream: TcpStream,
    device_name: String,
    next_request_id: i32,
}

impl Session {
    pub fn write_hello(&mut self) -> Result<()> {
        let mut tls_writer = self.tls.writer();
        let mut output = protobuf::CodedOutputStream::new(&mut tls_writer);

        let mut magic = [0u8;4];
        NetworkEndian::write_u32(&mut magic, HELLO_MAGIC);
        output.write_raw_bytes(&magic)?;

        let mut hello = syncthing_proto::Hello::new();
        hello.device_name = self.device_name.clone();
        hello.client_name = env!("CARGO_PKG_NAME").to_owned();
        hello.client_version = env!("CARGO_PKG_VERSION").to_owned();

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
        hello.merge_from(&mut input).context("error reading Hello")?;

        Ok((input.pos() as usize, hello))
    }

    pub fn read_message(buf: &[u8])
            -> Result<(usize, syncthing_proto::MessageType, Box<dyn SyncthingMessage>)> {
        let mut input = protobuf::CodedInputStream::from_bytes(buf);

        let header_length = NetworkEndian::read_u16(&input.read_raw_bytes(2)?);
        let mut header = syncthing_proto::Header::new();
        let old_limit = input.push_limit(u64::from(header_length))?;
        header.merge_from(&mut input).context("error reading message header")?;
        input.pop_limit(old_limit);

        debug!("header: {:?}, compression: {:?}", header.type_, header.compression);

        let body_length = NetworkEndian::read_u32(&input.read_raw_bytes(4)?);
        debug!("body length = {} / {:#x}", body_length, body_length);

        let body_protobuf = match header.compression.unwrap() {
            syncthing_proto::MessageCompression::LZ4 => {
                let uncompressed_length = NetworkEndian::read_u32(&input.read_raw_bytes(4)?);
                debug!("uncompressed length = {} / {:#x}", uncompressed_length, uncompressed_length);
                let lz4_slice = &buf[input.pos() as usize
                    .. input.pos() as usize + body_length as usize - 4];
                let body_protobuf = lz4_compression::decompress::decompress(lz4_slice)
                    .map_err(|e| anyhow::anyhow!("LZ4 decrompression error: {:?}", e))?;
                debug!("{} / {:#x} LZ4 bytes processed", body_protobuf.len(), body_protobuf.len());
                if body_protobuf.len() as u32 != uncompressed_length {
                    bail!("uncompressed LZ4 data ({} bytes) doesn't match expected length ({} bytes)",
                            body_protobuf.len(), uncompressed_length);
                }
                input.skip_raw_bytes(body_length - 4)?;
                body_protobuf
            },
            syncthing_proto::MessageCompression::NONE => {
                input.read_raw_bytes(body_length)?
            }
        };

        let mut body_input = protobuf::CodedInputStream::from_bytes(&body_protobuf);
        let mut body: Box<dyn SyncthingMessage> = match header.type_.unwrap() {
            syncthing_proto::MessageType::CLUSTER_CONFIG    => Box::new(syncthing_proto::ClusterConfig::new()),
            syncthing_proto::MessageType::INDEX             => Box::new(syncthing_proto::Index::new()),
            syncthing_proto::MessageType::INDEX_UPDATE      => Box::new(syncthing_proto::IndexUpdate::new()),
            syncthing_proto::MessageType::REQUEST           => unimplemented!("message type REQUEST"),
            syncthing_proto::MessageType::RESPONSE          => Box::new(syncthing_proto::Response::new()),
            syncthing_proto::MessageType::DOWNLOAD_PROGRESS => unimplemented!("message type DOWNLOAD_PROGRESS"),
            syncthing_proto::MessageType::PING              => Box::new(syncthing_proto::Ping::new()),
            syncthing_proto::MessageType::CLOSE             => Box::new(syncthing_proto::Close::new()),
        };
        body.as_mut().as_protobuf_message().merge_from_dyn(&mut body_input)?;

        debug!("body_input pos: {}", body_input.pos());

        Ok((input.pos() as usize, header.type_.unwrap(), body))
    }

    pub fn write_message<T: ProtobufMessage + protobuf::Message>(
        &mut self,
        message: &T,
        message_type: syncthing_proto::MessageType,
        ) -> Result<()>
    {
        let mut tls_writer = self.tls.writer();
        let mut output = protobuf::CodedOutputStream::new(&mut tls_writer);

        let mut header = syncthing_proto::Header::new();
        header.compression = syncthing_proto::MessageCompression::NONE.into();
        header.type_ = message_type.into();

        let mut header_len = [0u8; 2];
        NetworkEndian::write_u16(&mut header_len, header.compute_size() as u16);
        output.write_raw_bytes(&header_len)?;
        header.write_to_with_cached_sizes(&mut output)?;

        let mut body_len = [0u8; 4];
        NetworkEndian::write_u32(&mut body_len, message.compute_size() as u32);
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

        debug!("sending block request {}:", request_id);
        debug!("    folder: {:?}", folder);
        debug!("    path: {:?}", path);
        debug!("    offset: {:?}", offset);
        debug!("    size: {:?}", size);

        let mut req = syncthing_proto::Request::new();
        req.id = request_id;
        req.folder = folder;
        req.name = path;
        req.offset = offset;
        req.size = size;
        req.hash = hash;
        req.from_temporary = false;

        self.write_message(&req, syncthing_proto::MessageType::REQUEST)?;
        Ok(request_id)
    }

    // FIXME(wfraser) only for testing
    pub fn read_to_end(&mut self, data: &mut Vec<u8>) -> Result<usize> {
        use std::io::Read;
        use std::mem::transmute;

        let mut nread = 0;
        loop {
            if data.len() == data.capacity() {
                data.reserve(32);
            }

            match self.tls.reader().read(unsafe { transmute(data.spare_capacity_mut()) }) {
                Ok(n) => {
                    unsafe { data.set_len(data.len() + n); }
                    nread += n;
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No more plaintext available.
                    break;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        Ok(nread)
    }

    // FIXME(wfraser) only for testing
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        use std::io::Write;
        self.tls.writer().write(data).map_err(|e| e.into())
    }

    // This is basically rustls::ClientSession::complete_io() with extra logging.
    // FIXME(wfraser) only public for testing
    pub fn complete_io(&mut self) -> Result<(usize, usize)> {
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

impl ::std::fmt::Debug for Session {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut s = fmt.debug_struct("stget::session::Session");
        s.field("device_name", &self.device_name);
        s.finish()
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
                    return Err(e).context("failed to get the system hostname");
                }
            }
        };
        info!("our device name is {:?}", device_name);

        let mut config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(
                Arc::new(SyncthingCertVerifier::new(self.remote_device_id)))
            .with_single_cert(vec![self.client_cert], self.private_key)?;
        config.alpn_protocols.push(b"bep/1.0".to_vec());

        let host_and_port = &self.remote_host_and_port;
        let stream = TcpStream::connect(host_and_port).map_err(|e| {
            error!("failed to connect to {}: {}", host_and_port, e);
            e
        })?;

        let dnsname = rustls::ServerName::try_from("syncthing")?;

        Ok(Session {
            tls: rustls::ClientConnection::new(Arc::new(config), dnsname)?,
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

impl rustls::client::ServerCertVerifier for SyncthingCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> ::std::result::Result<rustls::client::ServerCertVerified, rustls::Error>
    {
        use rustls::internal::msgs::codec::Codec;
        debug!("Checking device ID");
        let cert_bytes = end_entity.get_encoding();
        let mut hash_ctx = ring::digest::Context::new(&ring::digest::SHA256);
        hash_ctx.update(&cert_bytes[3..]);
        let digest = hash_ctx.finish();
        debug!("cert hash is {:?}", digest);
        let device_id = util::device_id_from_hash(digest.as_ref());
        debug!("device ID {}", device_id);
        if device_id == self.device_id {
            debug!("device ID matches");
            return Ok(rustls::client::ServerCertVerified::assertion());
        }
        error!("none of the presented server certificates have the expected Device ID");
        Err(rustls::Error::General("Syncthing device ID mismatch".to_owned()))
    }
}
