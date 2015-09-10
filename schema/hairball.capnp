@0xcd2e2c45e62d6958;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("hairball");

struct Version {
    major @0 :UInt16;
    minor @1 :UInt16;
    patch @2 :UInt16;
}

enum Encoding {
    unpacked @0;
    packed @1;
}

struct Column {
    name @0 :Text;
    version @1 :Version;
    encoding @2 :Encoding;
    data @3 :Data;
}

# Used to lookup an entry
struct LocalEntry {
    name @0 :Text;
    parent @1 :UInt32;
}

struct ExternalEntry {
    fileUuid @0 :Data;
    union {
        path @1 :Text;
        index @2 :UInt32;
    }
}

struct Entity {
    union {
        local @0 :LocalEntry;
        external @1 :ExternalEntry;
    }
}

struct Hairball {
    version @0 :Version;
    uuid @1 :Data;
    entities @2 :List(Entity);
    columns @3 :List(Column);
}

