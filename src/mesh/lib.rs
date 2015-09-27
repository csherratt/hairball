extern crate capnp;
extern crate hairball;
extern crate gfx_mesh;

use gfx_mesh::{Attribute, Interlaced};

#[cfg(feature="build-schema")]
pub mod mesh_capnp {
    include!(concat!(env!("OUT_DIR"), "/mesh_capnp.rs"));
}

const COLUMN_NAME: &'static str = "mesh";

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