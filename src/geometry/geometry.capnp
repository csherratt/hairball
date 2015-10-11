@0xd7beb209768ff4ac;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("geometry");

struct Geometry {
	id @0 :UInt32;
	mesh @1 :UInt32;
	start @2 :UInt32;
	length @3 :UInt32;
}

struct Column {
	geometries @0: List(Geometry);
}