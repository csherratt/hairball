extern crate capnp;
extern crate hairball;

#[cfg(feature="build-schema")]
pub mod geometry_capnp {
    include!(concat!(env!("OUT_DIR"), "/geometry_capnp.rs"));
}

#[cfg(not(feature="build-schema"))]
pub mod geometry_capnp;

const COLUMN_NAME: &'static str = "geometry";

#[derive(Copy, Debug, Clone)]
pub struct Geometry<E> {
    pub mesh: E,
    pub start: u32,
    pub length: u32
}

/// Write out a column into a hairball
pub fn write(hb: &mut hairball::Builder, i: &[(u32, Geometry<u32>)]) {
    let column: geometry_capnp::column::Builder = hb.column(COLUMN_NAME)
        .and_then(|c| c.get_as())
        .unwrap();

    let mut rows = column.init_geometries(i.len() as u32);
    for (i, &(name, geo)) in i.iter().enumerate() {
        let mut row = rows.borrow().get(i as u32);
        row.set_id(name);
        row.set_start(geo.start);
        row.set_length(geo.length);
        row.set_mesh(geo.mesh);
    }
}


impl<'a, E> Iterator for Reader<'a, E> {
    type Item = (&'a E, Geometry<&'a E>);

    fn next(&mut self) -> Option<Self::Item> {
        for i in &mut self.index {
            let row = self.column.get(i);

            let id = match self.reader.entity(row.get_id() as usize) {
                Some(id) => id,
                None => continue
            };

            let mesh = match self.reader.entity(row.get_mesh() as usize) {
                Some(mesh) => mesh,
                None => continue
            };

            return Some((
                id,
                Geometry{
                    mesh: mesh,
                    start: row.get_start(),
                    length: row.get_length()
                }
            ));
        }
        None
    }
}

/// Used to read geometry colimn from the hairball
pub struct Reader<'a, E:'a> {
    reader: &'a hairball::ReaderMapping<'a, E>,
    column: capnp::struct_list::Reader<'a, geometry_capnp::geometry::Owned>,
    index: std::ops::Range<u32>
}

/// Create a column reader for iff the hairball has a valid geometry
/// column set
pub fn read<'a, E>(read: &'a hairball::ReaderMapping<'a, E>) -> Option<Reader<'a, E>>
    where E: 'a
{
    read.column(COLUMN_NAME)
        .and_then(|c| c.get_as().ok() )
        .and_then(|c: geometry_capnp::column::Reader<'a>| c.get_geometries().ok( ))
        .map(|c| {
            let len = c.len();
            Reader{
                reader: read,
                column: c,
                index: (0..len)
            }
        })
}