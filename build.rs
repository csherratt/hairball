
extern crate capnpc;

fn main() {
    ::capnpc::compile("hairball", &["schema/hairball.capnp"]).unwrap();
}