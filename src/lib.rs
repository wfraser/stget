extern crate base32;
extern crate protobuf;

pub const HELLO_MAGIC: u32 = 0x2ea7d90b;

pub mod syncthing_proto;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
