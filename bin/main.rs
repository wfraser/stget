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
extern crate webpki;

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
        DeviceIdMismatch {
            description("Device ID mismatch")
            display("Device ID mismatch")
        }
        WebPki(inner: webpki::Error) {
            description("WebPKI error")
            display("WebPKI Error: {:?}", inner)
        }
    }
}

fn get_certificate(host_and_port: &str, cn: &str) -> Result<rustls::Certificate> {
    use rustls::{Session, TLSError};

    let config = Arc::new(rustls::ClientConfig::new());
    let mut stream = TcpStream::connect(host_and_port)?;
    let mut client = rustls::ClientSession::new(&config, cn);
    loop {
        if client.wants_write() {
            debug!("TLS writing");
            client.write_tls(&mut stream)?;
        }

        if client.wants_read() {
            debug!("TLS reading");
            client.read_tls(&mut stream)?;
            let result = client.process_new_packets();
            if let Err(TLSError::WebPKIError(webpki::Error::UnknownIssuer)) = result {
                println!("got unknown issuer error; grabbing cert and reconnecting");
                if let Some(certs) = client.get_peer_certificates() {
                    return Ok(certs[0].clone());
                }
            } else if let Err(e) = result {
                return Err(e.into());
            }
        }

        if !client.is_handshaking() {
            // This shouldn't happen, as the initial config has an empty set of root certs.
            bail!("finished handshaking without certificate error!");
        }
    }
}

fn write_hello(client: &mut rustls::ClientSession) -> Result<()> {
    use protobuf::{CodedOutputStream, Message};
    let mut output = CodedOutputStream::new(client);

    /*
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
    */

    output.write_raw_bytes(b"GET / HTTP/1.0\r\n\
                             Host: www.codewise.org\r\n\
                             Connection: close\r\n\
                             Accept-Encoding: identity\r\n\
                             \r\n")?;
    output.flush()?;

    Ok(())
}

/// `host_and_port`: string with the hostname or IP address, a colon, and the port to connect to.
/// `cn`: the expected Common Name of the certificate the host on the other side must present
/// `device_id`: the SHA-256 of the certificate the host on the other end must present
fn connect(host_and_port: &str, cn: &str, device_id: &str) -> Result<rustls::ClientSession>{
    use rustls::Session;
    use rustls::internal::msgs::codec::Codec;

    let cert = get_certificate(host_and_port, cn)?;

    let cert_bytes = cert.get_encoding();
    let mut hash_ctx = ring::digest::Context::new(&ring::digest::SHA256);
    hash_ctx.update(&cert_bytes[3..]);
    let digest = hash_ctx.finish();
    println!("remote certificate hash is {:?}", digest);
    let actual_device_id = stget::util::device_id_from_hash(digest.as_ref());
    println!("remote has device ID {}", actual_device_id);

    if actual_device_id != device_id {
        bail!(ErrorKind::DeviceIdMismatch);
    }

    let mut config = rustls::ClientConfig::new();
    config.root_store.add(&cert).map_err(ErrorKind::WebPki)?;

    let mut client = rustls::ClientSession::new(&Arc::new(config), cn);

    write_hello(&mut client)?;

    loop {
        let mut stream = TcpStream::connect(host_and_port)?;
        stream.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;

        if client.wants_write() {
            println!("writing");
            client.write_tls(&mut stream)?;
        }
        if client.wants_read() {
            println!("reading");
            match client.read_tls(&mut stream) {
                Ok(nread) => {
                    println!("read {} bytes of TLS data", nread);
                },
                Err(e) => {
                    println!("TLS read error: {}", e);
                    return Err(e.into());
                }
            }
            if let Err(e) = client.process_new_packets() {
                println!("got an error processing TLS packets: {}", e);
                println!("peer certs: {:?}", client.get_peer_certificates());
                return Err(e.into());
            }
            let mut plaintext = Vec::new();
            match client.read_to_end(&mut plaintext) {
                Ok(nread) => {
                    if nread != 0 {
                        println!("read plaintext: {}", String::from_utf8_lossy(&plaintext));
                    } else {
                        println!("read no plaintext");
                    }
                },
                Err(e) => {
                    println!("plaintext read error: {}", e);
                    return Err(e.into());
                }
            }
        }

        if !client.is_handshaking() {
            println!("done handshaking");
            return Ok(client);
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    //connect("10.0.0.1:22000", "syncthing", "JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH").unwrap();
    connect("www.codewise.org:443", "www.codewise.org", "O3PT6VP-QEQ4YBZ-OTMICG5-MX2G4KX-HK7ZTWP-OMANANC-WXR4TCI-PA3QWAA").unwrap();

    println!("Hello, World!");
}
