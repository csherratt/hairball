@0xfba3396bec085320;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("draw_binding");

struct DrawBinding {
    id @0 :UInt32;
    geometry @1 :UInt32;
    material @2 :UInt32;
}

struct Column {
    bindings @0 :List(DrawBinding);
}
