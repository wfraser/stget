#![allow(unknown_lints)]

extern crate base32;
extern crate byteorder;
#[macro_use] extern crate error_chain;
extern crate libc;
extern crate lz4_compress;
#[macro_use] extern crate log;
extern crate protobuf;
extern crate ring;
extern crate rustls;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ProtoBuf(protobuf::ProtobufError);
        //LZ4(lz4_compress::Error); // see https://github.com/redox-os/tfs/pull/85
        Tls(rustls::TLSError);
    }

    errors {
        LZ4 {
            description("LZ4 decompression error")
            display("LZ4 decompression error")
        }
    }
}

pub mod certificate;
pub mod session;
pub mod syncthing_proto;
pub mod util;

pub use certificate::{Certificate, PrivateKey};

pub trait SyncthingMessage {
    fn as_any(&self) -> &std::any::Any;
    fn as_any_mut(&mut self) -> &mut std::any::Any;
    fn as_protobuf_message(&mut self) -> &mut protobuf::Message;
}

macro_rules! impl_syncthing_message {
    ($type:path) => {
        impl SyncthingMessage for $type {
            fn as_any(&self) -> &std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut std::any::Any { self }
            fn as_protobuf_message(&mut self) -> &mut protobuf::Message { self }
        }
    }
}

impl_syncthing_message!(syncthing_proto::ClusterConfig);
impl_syncthing_message!(syncthing_proto::Index);
impl_syncthing_message!(syncthing_proto::IndexUpdate);
impl_syncthing_message!(syncthing_proto::Request);
impl_syncthing_message!(syncthing_proto::Response);
impl_syncthing_message!(syncthing_proto::DownloadProgress);
impl_syncthing_message!(syncthing_proto::Ping);
impl_syncthing_message!(syncthing_proto::Close);
