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
        //("127.0.0.1:22000", "syncthing", "JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH");
        ("odin.codewise.org:443", "odin.codewise.org", "ODHDPS7-OMUJMIS-XS3ZKPX-SM6YAU5-KN2WEXK-F65DPTB-DCN4KMU-MACHMQS");

    let mut session = stget::session::SessionBuilder::new(host_and_port.to_owned(), device_id.to_owned())
        .remote_tls_hostname(cn.to_owned())
        .connect().unwrap();

    session.write(b"GET / HTTP/1.0\r\n\
                  Host: odin.codewise.org\r\n\
                  Connection: close\r\n\
                  Accept-Encoding: identity\r\n\
                  \r\n")
            .unwrap();

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

    println!("{}", String::from_utf8_lossy(&data));
}
