extern crate capnp;
extern crate hairball;
extern crate gfx;
extern crate gfx_mesh;

use gfx_mesh::{Attribute, Interlaced};

#[cfg(feature="build-schema")]
pub mod mesh_capnp {
    include!(concat!(env!("OUT_DIR"), "/mesh_capnp.rs"));
}

#[cfg(not(feature="build-schema"))]
pub mod mesh_capnp;

const COLUMN_NAME: &'static str = "mesh";

pub enum Error {
    Capnp(capnp::Error),
    NotInSchema,
    Mesh(gfx_mesh::Error)
}

impl std::convert::From<capnp::Error> for Error {
    fn from(err: capnp::Error) -> Error {
        Error::Capnp(err)
    }
}

impl std::convert::From<capnp::NotInSchema> for Error {
    fn from(_: capnp::NotInSchema) -> Error {
        Error::NotInSchema
    }
}

impl std::convert::From<gfx_mesh::Error> for Error {
    fn from(err: gfx_mesh::Error) -> Error {
        Error::Mesh(err)
    }
}

pub type Mesh<'a> = Vec<Interlaced<Vec<Attribute<&'a str>>, &'a str, &'a [u8]>>;

fn decode_attribute<'a>(a: mesh_capnp::attribute::Reader<'a>) -> Result<Attribute<&'a str>, Error> {
    use mesh_capnp::Type::*;

    let name = try!(a.get_name());
    let count = a.get_element_count();

    Ok(match try!(a.get_element_type()) {
        F32 => Attribute::f32(name, count),
        F64 => Attribute::f64(name, count),
        U8 => Attribute::u8(name, count),
        U16 => Attribute::u16(name, count),
        U32 => Attribute::u32(name, count),
        I8 => Attribute::i8(name, count),
        I16 => Attribute::i16(name, count),
        I32 => Attribute::i32(name, count),
        NormalizedU8 => Attribute::normalized_u8(name, count),
        NormalizedU16=> Attribute::normalized_u16(name, count),
        NormalizedU32 => Attribute::normalized_u32(name, count),
        NormalizedI8 => Attribute::normalized_i8(name, count),
        NormalizedI16=> Attribute::normalized_i16(name, count),
        NormalizedI32 => Attribute::normalized_i32(name, count),
    })
}

fn decode_attributes<'a>(r: capnp::struct_list::Reader<'a, mesh_capnp::attribute::Owned>) -> Result<Vec<Attribute<&'a str>>, Error> {
    let mut vec = Vec::new();
    for i in 0..r.len() {
        vec.push(try!(decode_attribute(r.get(i))));
    }
    Ok(vec)
}

fn decode_vb<'a>(vb: mesh_capnp::vertex_buffer::Reader<'a>) -> Result<Interlaced<Vec<Attribute<&'a str>>, &'a str, &'a [u8]>, Error> {
    let data = try!(vb.get_data());
    let attributes = try!(vb.get_attributes());
    let attributes = try!(decode_attributes(attributes));
    Ok(try!(Interlaced::new(attributes, data)))
}

fn decode<'a>(m: mesh_capnp::mesh::Reader<'a>) -> Result<(usize, Mesh<'a>), Error> {
    let id = m.get_id();
    let mut buffers = Vec::new();
    let b = try!(m.get_buffers());

    for i in 0..b.len() {
        try!(decode_vb(b.get(i)).map(|vb| buffers.push(vb)));
    }

    Ok((id as usize, buffers))
}

impl<'a, E> Iterator for Reader<'a, E> {
    type Item = (&'a E, Mesh<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(meshes) = self.column.get_meshes() {
            while self.index < meshes.len() {
                let idx = self.index;
                self.index += 1;

                if let Ok((idx, m)) = decode(meshes.get(idx)) {
                    if let Some(e) = self.reader.entity(idx) {
                        return Some((e, m));
                    }
                }
            }            
        }
        None
    }
}

pub struct Reader<'a, E:'a> {
    reader: &'a hairball::ReaderMapping<'a, E>,
    column: mesh_capnp::column::Reader<'a>,
    index: u32
}

pub fn read<'a, E>(read: &'a hairball::ReaderMapping<'a, E>) -> Option<Reader<'a, E>>
    where E: 'a
{
    read.column(COLUMN_NAME)
        .and_then(|c| c.get_as().ok() )
        .map(|c| Reader{
            reader: read,
            column: c,
            index: 0
        })
}

fn write_attribute<S>(mut builder: mesh_capnp::attribute::Builder, mesh: &Attribute<S>)
    where S: AsRef<str>
{
    use gfx::device::attrib::{Type, FloatSize, FloatSubType, IntSize, IntSubType, SignFlag};
    use mesh_capnp::Type::*;

    builder.set_name(mesh.name.as_ref());
    builder.set_element_count(mesh.element_count);
    let t = match mesh.element_type {
        Type::Float(FloatSubType::Default, FloatSize::F32) => F32,
        Type::Float(FloatSubType::Default, FloatSize::F64) => F64,
        Type::Int(IntSubType::Raw, IntSize::U8, SignFlag::Signed) => I8,
        Type::Int(IntSubType::Raw, IntSize::U16, SignFlag::Signed) => I16,
        Type::Int(IntSubType::Raw, IntSize::U32, SignFlag::Signed) => I32,
        Type::Int(IntSubType::Raw, IntSize::U8, SignFlag::Unsigned) => U8,
        Type::Int(IntSubType::Raw, IntSize::U16, SignFlag::Unsigned) => U16,
        Type::Int(IntSubType::Raw, IntSize::U32, SignFlag::Unsigned) => U32,
        Type::Int(IntSubType::Normalized, IntSize::U8, SignFlag::Signed) => NormalizedI8,
        Type::Int(IntSubType::Normalized, IntSize::U16, SignFlag::Signed) => NormalizedI16,
        Type::Int(IntSubType::Normalized, IntSize::U32, SignFlag::Signed) => NormalizedI32,
        Type::Int(IntSubType::Normalized, IntSize::U8, SignFlag::Unsigned) => NormalizedU8,
        Type::Int(IntSubType::Normalized, IntSize::U16, SignFlag::Unsigned) => NormalizedU16,
        Type::Int(IntSubType::Normalized, IntSize::U32, SignFlag::Unsigned) => NormalizedU32,
        _ => panic!("unsupported type {:?}", mesh.element_type)
    };
    builder.set_element_type(t);
}

fn write_buffers<R, A, S, D>(mut builder: mesh_capnp::mesh::Builder, name: u32, mesh: &R)
    where R: AsRef<[Interlaced<A, S, D>]>,
          A: AsRef<[Attribute<S>]>,
          S: AsRef<str>,
          D: AsRef<[u8]>
{
    builder.set_id(name);
    let mut buffers = builder.init_buffers(mesh.as_ref().len() as u32);
    for (i, buff) in mesh.as_ref().iter().enumerate() {
        let mut m = buffers.borrow().get(i as u32);
        m.set_data(buff.data().as_ref());
        let mut attrs = m.init_attributes(buff.attributes().as_ref().len() as u32);
        for (j, att) in buff.attributes().as_ref().iter().enumerate() {
            let a = attrs.borrow().get(j as u32);
            write_attribute(a, att);
        }
    }
}

///
pub fn write<'a, R, A, S, D>(hb: &mut hairball::Builder, i: &[(u32, &'a R)])
    where R: AsRef<[Interlaced<A, S, D>]>,
          A: AsRef<[Attribute<S>]>,
          S: AsRef<str>,
          D: AsRef<[u8]>
{
    let column: mesh_capnp::column::Builder = hb.column(COLUMN_NAME)
        .and_then(|c| c.get_as())
        .unwrap();

    let mut rows = column.init_meshes(i.len() as u32);
    for (i, &(name, ref mesh)) in i.iter().enumerate() {
        let row = rows.borrow().get(i as u32);
        write_buffers(row, name, mesh);
    }
}
