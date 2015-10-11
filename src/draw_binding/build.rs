extern crate capnpc;

#[cfg(feature="build-schema")]
fn main() {
    ::capnpc::compile("draw_binding", &["draw_binding.capnp"]).unwrap();
}

#[cfg(not(feature="build-schema"))]
fn main() {}
