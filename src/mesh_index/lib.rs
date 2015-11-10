extern crate capnp;
extern crate hairball;

#[cfg(feature="build-schema")]
pub mod index_capnp {
    include!(concat!(env!("OUT_DIR"), "/index_capnp.rs"));
}

#[cfg(not(feature="build-schema"))]
pub mod index_capnp;

const COLUMN_NAME: &'static str = "mesh_index";

pub fn write<'a, R>(hb: &mut hairball::Builder, i: &[(u32, &'a R)])
    where R: AsRef<[u32]>
{
    let column: index_capnp::column::Builder = hb.column(COLUMN_NAME)
        .and_then(|c| c.get_as())
        .unwrap();

    let mut rows = column.init_meshes(i.len() as u32);
    for (i, &(name, ref index)) in i.iter().enumerate() {
        let mut row = rows.borrow().get(i as u32);
        row.set_id(name);
        let src = index.as_ref();
        let mut dst = row.init_index(src.len() as u32);
        for (i, &s) in src.iter().enumerate() {
            dst.set(i as u32, s);
        }
    }
}

impl<'a, E> Iterator for Reader<'a, E> {
    type Item = (&'a E, Vec<u32>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(meshes) = self.column.get_meshes() {
            while self.index < meshes.len() {
                let idx = self.index;
                self.index += 1;

                let m = meshes.get(idx);
                let id = match self.reader.entity(m.get_id() as usize) {
                    Some(id) => id,
                    None => continue
                };

                if let Ok(idx) = m.get_index() {
                    let vec: Vec<u32> = (0..idx.len()).map(|i| idx.get(i)).collect();
                    return Some((id, vec));
                }
            }
        }
        None
    }
}

pub struct Reader<'a, E:'a> {
    reader: &'a hairball::ReaderMapping<'a, E>,
    column: index_capnp::column::Reader<'a>,
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
