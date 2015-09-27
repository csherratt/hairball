
extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("hairball", &["hairball.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}
