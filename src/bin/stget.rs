#![allow(unknown_lints)]

extern crate env_logger;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate stget;

use std::path::Path;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }

    errors {
    }
}

fn main() {
    env_logger::init().unwrap();

    let (host_and_port, cn, device_id) =
        ("127.0.0.1:22000", "syncthing", "JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH");
        //("odin.codewise.org:443", "odin.codewise.org", "ODHDPS7-OMUJMIS-XS3ZKPX-SM6YAU5-KN2WEXK-F65DPTB-DCN4KMU-MACHMQS");

    let cert = stget::certificate::read_cert_file_pem(Path::new("cert/cert.pem")).unwrap();
    let key = stget::certificate::read_key_file_pem(Path::new("cert/private.pem")).unwrap();

    let mut session = stget::session::SessionBuilder::new(host_and_port.to_owned(), device_id.to_owned(), cert, key)
        .remote_tls_hostname(cn.to_owned())
        .connect().unwrap();

    /*
    session.write(b"GET / HTTP/1.0\r\n\
                  Host: odin.codewise.org\r\n\
                  Connection: close\r\n\
                  Accept-Encoding: identity\r\n\
                  \r\n")
            .unwrap();
    */

    session.write_hello().unwrap();

    let mut data = vec![];
    while let Ok((r, w)) = session.complete_io() {
        if r == 0 && w == 0 {
            break;
        }
        println!("r = {}, w = {}", r, w);
        let len = data.len();
        match session.read_to_end(&mut data) {
            Err(stget::Error(stget::ErrorKind::Io(ref e), _))
                    if e.kind() == std::io::ErrorKind::ConnectionAborted => {
                println!("len was {}, now is {}", len, data.len());
                println!("connection closed");
            },
            Err(e) => {
                println!("len was {}, now is {}", len, data.len());
                panic!("read error: {:?}", e)
            },
            Ok(n) => {
                println!("read {}", n);
                if w == 0 && n == r {
                    println!("done");
                    break;
                }
            }
        }
    }

    for i in 0 .. ((data.len() / 16) + 1) {
        print!("{:04x}  ", i);
        for h in 0 .. 16 {
            if i * 16 + h < data.len() {
                print!("{:02x} ", data[i * 16 + h]);
            } else {
                print!("   ");
            }
            if h == 8 {
                print!(" ");
            }
        }
        print!("  |");
        for h in 0 .. 16 {
            if i * 16 + h < data.len() {
                let mut c = data[i * 16 + h] as char;
                if c < ' ' {
                    c = '.'
                }
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!("|");
    }

    //println!("{}", String::from_utf8_lossy(&data));
}
