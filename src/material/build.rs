extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("material", &["material.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}
