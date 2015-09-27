extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("mesh", &["mesh.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}



