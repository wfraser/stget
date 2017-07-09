#![allow(unknown_lints)]

extern crate base32;
extern crate byteorder;
extern crate compress;
#[macro_use] extern crate error_chain;
extern crate libc;
#[macro_use] extern crate log;
extern crate protobuf;
extern crate ring;
extern crate rustls;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ProtoBuf(protobuf::ProtobufError);
        Tls(rustls::TLSError);
    }

    errors {
    }
}

pub mod certificate;
pub mod session;
pub mod syncthing_proto;
pub mod util;

pub use certificate::{Certificate, PrivateKey};
