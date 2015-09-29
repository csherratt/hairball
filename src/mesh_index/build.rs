extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("index", &["index.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}
