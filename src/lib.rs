#[macro_use] extern crate log;

pub mod certificate;
pub mod session;
pub mod syncthing_proto;
pub mod util;

pub use certificate::{Certificate, PrivateKey};

pub trait SyncthingMessage {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn as_protobuf_message(&mut self) -> &mut dyn protobuf::MessageDyn;
}

macro_rules! impl_syncthing_message {
    ($type:path) => {
        impl SyncthingMessage for $type {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
            fn as_protobuf_message(&mut self) -> &mut dyn protobuf::MessageDyn { self }
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
