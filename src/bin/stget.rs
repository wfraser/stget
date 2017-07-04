#![allow(unknown_lints)]

extern crate clap;
extern crate env_logger;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
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

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("experimental Syncthing file retrieval program")
            .arg(clap::Arg::with_name("address")
                    .help("Address of the remote host. Port 22000 is used if unspecified.")
                    .required(true)
                    .index(1))
            .arg(clap::Arg::with_name("device_id")
                    .help("Device ID of the remote host.")
                    .required(true)
                    .index(2))
            .arg(clap::Arg::with_name("path")
                    .help("File path to fetch.")
                    .index(3))
            .arg(clap::Arg::with_name("list")
                    .short("l")
                    .long("list")
                    .takes_value(false)
                    .help("List all files on the remote end."))
            .group(clap::ArgGroup::with_name("path_or_list")
                    .args(&["path", "list"])
                    .required(true))
            .get_matches();

    let host_and_port = match matches.value_of("address").unwrap() {
        host if host.contains(':') => host.to_owned(),
        host => {
            debug!("no port specified; assuming 22000");
            format!("{}:22000", host)
        }
    };

    let device_id = matches.value_of("device_id").unwrap();

    /*
    // FIXME(wfraser) remove this
    let (host_and_port, device_id) =
        ("127.0.0.1:22000", "JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH");
    */

    // TODO(wfraser) base path should be in $XDG_CONFIG_DIRS or something
    let base_path = std::env::current_dir().unwrap_or_else(|e| {
        panic!("unable to get working directory: {}", e);
    });

    // TODO(wfraser) make this configurable
    let cert_path = base_path.join("cert").join("cert.pem");
    let key_path = base_path.join("cert").join("private.pem");

    let cert = stget::certificate::read_cert_file_pem(&cert_path).unwrap_or_else(|e| {
        println!("Unable to load certificate {:?}: {}", cert_path, e);
        println!("Did you remember to generate a client certificate?");
        std::process::exit(1);
    });
    let key = stget::certificate::read_key_file_pem(&key_path).unwrap_or_else(|e| {
        println!("Unable to load private key {:?}: {}", key_path, e);
        println!("Did you remember to generate a client certificate?");
        std::process::exit(1);
    });

    let mut session = stget::session::SessionBuilder {
        remote_host_and_port: host_and_port.to_string(),
        remote_device_id: device_id.to_string(),
        local_device_name: None,
        client_cert: cert,
        private_key: key,
    }.connect().expect("Failed to create TLS session");

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

    let (len, remote_hello): (usize, stget::syncthing_proto::Hello) =
        stget::session::Session::read_hello(&data).unwrap_or_else(|e| {
            println!("error reading remote hello: {}", e);
            panic!(e);
        });
    data.drain(0..len);

    println!("Remote is {}, running {} {}",
        remote_hello.device_name,
        remote_hello.client_name,
        remote_hello.client_version);
}
