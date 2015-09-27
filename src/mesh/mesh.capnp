@0xe5428d3d69b8dfa0;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("hairball");

enum Type {
    f32 @0;
    f64 @1;

    u8 @2;
    u16 @3;
    u32 @4;

    i8 @5;
    i16 @6;
    i32 @7;

    normalizedU8 @8;
    normalizedU16 @9;
    normalizedU32 @10;

    normalizedI8 @11;
    normalizedI16 @12;
    normalizedI32 @13;
}

struct Attribute {
    name @0 :Text;
    elementCount @1 :UInt8;
    elementType @2 :Type;
}

struct VertexBuffer {
    attributes @0 :List(Attribute);
    data @1 :Data;
}

struct Mesh {
    id @0 :UInt32;
    buffers @1 :List(VertexBuffer);
}

struct Column {
    meshes @0 :List(Mesh);
}
