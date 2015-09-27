
extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("hairball", &["schema/hairball.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}