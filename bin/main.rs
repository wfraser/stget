#![allow(unknown_lints)]

extern crate base32;
extern crate bufstream;
extern crate env_logger;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate protobuf;
extern crate ring;
extern crate rustls;
extern crate stget;

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

error_chain! {
    foreign_links {
        Io(io::Error);
        ProtoBuf(protobuf::ProtobufError);
        Tls(rustls::TLSError);
    }

    errors {
    }
}

fn write_hello(client: &mut rustls::ClientSession) -> Result<()> {
    use protobuf::{CodedOutputStream, Message};
    let mut output = CodedOutputStream::new(client);

    output.write_uint32_no_tag(stget::HELLO_MAGIC)?;

    let mut hello = stget::syncthing_proto::Hello::new();
    hello.set_device_name("test-device".into());
    hello.set_client_name("stget".into());
    hello.set_client_version("v0.0.1".into());
    hello.compute_size();

    output.write_raw_bytes(&[
        ((hello.get_cached_size() & 0xFF00) >> 8) as u8,
        (hello.get_cached_size() & 0xFF) as u8])?;

    hello.write_to_with_cached_sizes(&mut output)?;

    output.flush()?;

    Ok(())
}

struct SyncthingCertVerifier {
    device_id: String,
}

impl SyncthingCertVerifier {
    pub fn new<S: AsRef<str>>(device_id: S) -> SyncthingCertVerifier {
        SyncthingCertVerifier {
            device_id: device_id.as_ref().to_owned(),
        }
    }
}

impl rustls::ServerCertVerifier for SyncthingCertVerifier {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        presented_certs: &[rustls::Certificate],
        _dns_name: &str)
        -> std::result::Result<(), rustls::TLSError>
    {
        use rustls::internal::msgs::codec::Codec;
        debug!("Checking device ID. Server presented {} certificates", presented_certs.len());
        for (i, cert) in presented_certs.iter().enumerate() {
            let cert_bytes = cert.get_encoding();
            let mut hash_ctx = ring::digest::Context::new(&ring::digest::SHA256);
            hash_ctx.update(&cert_bytes[3..]);
            let digest = hash_ctx.finish();
            debug!("{}: cert hash is {:?}", i, digest);
            let device_id = stget::util::device_id_from_hash(digest.as_ref());
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

/// `host_and_port`: string with the hostname or IP address, a colon, and the port to connect to.
/// `cn`: the expected Common Name of the certificate the host on the other side must present
/// `device_id`: the SHA-256 of the certificate the host on the other end must present
fn connect(host_and_port: &str, cn: &str, device_id: &str) -> Result<rustls::ClientSession>{
    use rustls::Session;

    let mut config = rustls::ClientConfig::new();

    rustls::DangerousClientConfig { cfg: &mut config }.set_certificate_verifier(
        Box::new(SyncthingCertVerifier::new(device_id)));

    let mut client = rustls::ClientSession::new(&Arc::new(config), cn);

    //write_hello(&mut client)?;
    client.write(b"GET / HTTP/1.0\r\n
                   Host: www.codewise.org\r\n
                   Connection: close\r\n
                   Accept-Encoding: identity\r\n
                   \r\n")?;

    let mut stream = TcpStream::connect(host_and_port)?;
    //stream.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;

    // This is basically client.complete_io() with extra logging.
    loop {
        let handshaking = client.is_handshaking();
        let mut wrlen = 0;
        let mut rdlen = 0;

        loop {
            while client.wants_write() {
                debug!("writing");
                match client.write_tls(&mut stream) {
                    Ok(n) => {
                        debug!("wrote {} bytes of TLS data", n);
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
                break;
            }

            debug!("reading");
            match client.read_tls(&mut stream) {
                Ok(n) => {
                    debug!("read {} bytes of TLS data", n);
                    rdlen += n;
                },
                Err(e) => {
                    error!("read error: {}", e);
                    return Err(e.into());
                }
            }

            if let Err(e) = client.process_new_packets() {
                error!("error processing TLS packets: {}", e);
                return Err(e.into());
            }

            match (handshaking, client.is_handshaking()) {
                (true, false) => {
                    debug!("done handshaking");
                    break;
                },
                (false, _) => {
                    debug!("read completed");
                    break;
                },
                (_, _) => {
                    debug!("looping again");
                }
            }
        }

        debug!("rl = {}, wl = {}", rdlen, wrlen);
        if rdlen == 0 && wrlen == 0 {
            debug!("EOF");
            break;
        }
        let mut plaintext = vec![];
        client.read_to_end(&mut plaintext)?;
        println!("got plaintext of {} bytes", plaintext.len());
        println!("got plaintext: {}", String::from_utf8_lossy(&plaintext));
    }

    Ok(client)
}

fn main() {
    env_logger::init().unwrap();

    //connect("127.0.0.1:22000", "syncthing", "JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH").unwrap();
    connect("odin.codewise.org:443", "odin.codewise.org", "F7PLLUS-HP3DMOU-K7XCC3I-3AGZLZQ-6R4XT5D-FS7NZ7J-4Q3OZNJ-OJ2SSA7").unwrap();

    println!("Hello, World!");
}
