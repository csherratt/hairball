@0xea94daeee2a1719a;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("mesh_index");

struct Index {
    id @0 :UInt32;
    index @1 :List(UInt32);
}

struct Column {
    meshes @0 :List(Index);
}
