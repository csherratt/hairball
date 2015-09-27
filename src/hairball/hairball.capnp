@0xcd2e2c45e62d6958;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("hairball");

struct Version {
    major @0 :UInt16;
    minor @1 :UInt16;
    patch @2 :UInt16;
}

struct Column {
    next @0 :Column;
    name @1 :Text;
    version @2 :Version;
    data @3 :AnyPointer;
}

# Used to lookup an entry
struct LocalEntry {
    name @0 :Text;
    parent @1 :UInt32;
}

struct ExternalEntry {
    file @0 :UInt32;
    path @1 :Text;
}

struct Entity {
    union {
        local @0 :LocalEntry;
        external @1 :ExternalEntry;
    }
}

struct Hairball {
    version @0 :Version;
    entities @1 :List(Entity);
    columns @2 :Column;
    external @3 :List(Data);
}
