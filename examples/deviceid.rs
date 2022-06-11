extern crate clap;
extern crate ring;
extern crate rustls;
extern crate stget;

use std::path::Path;
use std::process::exit;
use rustls::internal::msgs::codec::Codec;

fn main() {
    let matches = clap::Command::new("deviceid")
            .about("Calculates the Syncthing Device ID of a given DER- or PEM-encoded certificate.")
            .arg(clap::Arg::new("der")
                .long("der")
                .takes_value(false))
            .arg(clap::Arg::new("pem")
                .long("pem")
                .takes_value(false))
            .group(clap::ArgGroup::new("format")
                .args(&["der", "pem"])
                .required(true))
            .arg(clap::Arg::new("path")
                .required(true))
            .get_matches();

    let path = Path::new(matches.value_of_os("path").unwrap());
    let mut hash_ctx = ring::digest::Context::new(&ring::digest::SHA256);

    let cert = if matches.is_present("der") {
        // NOTE: no verification of the format is done here! Garbage in, garbage out.
        stget::certificate::read_cert_file_der(path)
    } else if matches.is_present("pem") {
        stget::certificate::read_cert_file_pem(path)
    } else {
        panic!("no format selected");
    }.unwrap_or_else(|e| {
        println!("{:?}: {}", path, e);
        exit(1);
    });

    hash_ctx.update(&cert.get_encoding()[3..]); // 3 bytes indicate the length.

    let digest = hash_ctx.finish();
    println!("hash: {:?}", digest);

    let device_id = stget::util::device_id_from_hash(digest.as_ref());
    println!("device ID: {}", device_id);
}
