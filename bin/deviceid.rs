extern crate ring;
extern crate stget;

use std::env;
use std::fs::File;
use std::process::exit;

fn main() {
    let args = env::args_os().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0].to_string_lossy());
        println!("Calculates the Syncthing Device ID of a given DER-encoded certificate.");
        exit(-1);
    }

    let mut f = File::open(&args[1]).unwrap_or_else(|e| {
        println!("Error opening {:?}: {}", args[1], e);
        exit(1);
    });

    // TODO: check for PEM-encoded cert and do the conversion to DER

    let mut hash_ctx = ring::digest::Context::new(&ring::digest::SHA256);
    loop {
        use std::io::Read;
        let mut buf = [0u8; 512];
        match f.read(&mut buf) {
            Ok(n) => {
                if n != 0 {
                    hash_ctx.update(&buf[0..n]);
                }
                if n < buf.len() {
                    break;
                }
            },
            Err(e) => {
                println!("read error: {}", e);
                exit(1);
            }
        }
    }

    let digest = hash_ctx.finish();
    println!("hash: {:?}", digest);

    let device_id = stget::util::device_id_from_hash(digest.as_ref());
    println!("device ID: {}", device_id);
}
