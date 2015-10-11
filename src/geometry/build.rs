extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("geometry", &["geometry.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}
