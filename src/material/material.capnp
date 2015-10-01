@0x817b30d64a691bb5;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("material");

enum Component {
    ambient @0;
    diffuse @1;
    specular @2;
}

struct Color {
    red @0: Float32;
    green @1: Float32;
    blue @2: Float32;
    alpha @3: Float32;
}

struct Binding {
    id @0: UInt32;
    component @1: Component;
    union {
        texture @2: UInt32;
        color @3: Color;
    }
}

struct Column {
    bindings @0: List(Binding);
}