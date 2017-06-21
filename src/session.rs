use super::{Result, ResultExt};
use super::syncthing_proto;
use super::util;
use std::io;
use std::net::TcpStream;
use std::sync::Arc;

use ring;
use rustls;

const HELLO_MAGIC: u32 = 0x2ea7d90b;

pub struct Session {
    tls: rustls::ClientSession,
    stream: TcpStream,
    device_name: String,
}

impl Session {
    pub fn write_hello(&mut self) -> Result<()> {
        use protobuf::{CodedOutputStream, Message};
        let mut output = CodedOutputStream::new(&mut self.tls);

        output.write_uint32_no_tag(HELLO_MAGIC)?;

        let mut hello = syncthing_proto::Hello::new();
        hello.set_device_name(self.device_name.clone());
        hello.set_client_name(env!("CARGO_PKG_NAME").to_owned());
        hello.set_client_version(env!("CARGO_PKG_VERSION").to_owned());
        hello.compute_size();

        output.write_raw_bytes(&[
            ((hello.get_cached_size() & 0xFF00) >> 8) as u8,
            (hello.get_cached_size() & 0xFF) as u8])?;

        hello.write_to_with_cached_sizes(&mut output)?;
        output.flush()?;

        Ok(())
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
    host_and_port: String,
    device_id: String,
    remote_tls_hostname: Option<String>,
    device_name: Option<String>,
}

impl SessionBuilder {
    pub fn new(host_and_port: String, device_id: String) -> SessionBuilder {
        SessionBuilder {
            host_and_port: host_and_port,
            device_id: device_id,
            remote_tls_hostname: None,
            device_name: None,
        }
    }

    pub fn remote_tls_hostname(mut self, cn: String) -> Self {
        self.remote_tls_hostname = Some(cn);
        self
    }

    pub fn device_name(mut self, name: String) -> Self {
        self.device_name = Some(name);
        self
    }

    pub fn connect(self) -> Result<Session> {
        let device_name = match self.device_name {
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

        rustls::DangerousClientConfig { cfg: &mut config }
            .set_certificate_verifier(
                Box::new(SyncthingCertVerifier::new(self.device_id)));

        let host_and_port = &self.host_and_port;

        let stream = TcpStream::connect(host_and_port).map_err(|e| {
            error!("failed to connect to {}: {}", host_and_port, e);
            e
        })?;

        let cn = self.remote_tls_hostname.as_ref().map(|s| s.as_str()).unwrap_or("syncthing");

        Ok(Session {
            tls: rustls::ClientSession::new(&Arc::new(config), cn),
            stream: stream,
            device_name: device_name,
        })
    }
}

struct SyncthingCertVerifier {
    device_id: String,
}

impl SyncthingCertVerifier {
    pub fn new(device_id: String) -> SyncthingCertVerifier {
        SyncthingCertVerifier {
            device_id: device_id,
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
