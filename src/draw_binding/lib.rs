extern crate capnp;
extern crate hairball;

#[cfg(feature="build-schema")]
pub mod draw_binding_capnp {
    include!(concat!(env!("OUT_DIR"), "/draw_binding_capnp.rs"));
}

#[cfg(not(feature="build-schema"))]
pub mod draw_binding_capnp;

const COLUMN_NAME: &'static str = "draw_binding";

#[derive(Copy, Debug, Clone)]
pub struct DrawBinding<E> {
    pub geometry: E,
    pub material: E
}

/// Write out a column into a hairball
pub fn write(hb: &mut hairball::Builder, i: &[(u32, DrawBinding<u32>)]) {
    let column: draw_binding_capnp::column::Builder = hb.column(COLUMN_NAME)
        .and_then(|c| c.get_as())
        .unwrap();

    let mut rows = column.init_bindings(i.len() as u32);
    for (i, &(name, geo)) in i.iter().enumerate() {
        let mut row = rows.borrow().get(i as u32);
        row.set_id(name);
        row.set_geometry(geo.geometry);
        row.set_material(geo.material);
    }
}

impl<'a, E> Iterator for Reader<'a, E> {
    type Item = (&'a E, DrawBinding<&'a E>);

    fn next(&mut self) -> Option<Self::Item> {
        for i in &mut self.index {
            let row = self.column.get(i);

            let id = match self.reader.entity(row.get_id() as usize) {
                Some(id) => id,
                None => continue
            };

            let geometry = match self.reader.entity(row.get_geometry() as usize) {
                Some(geometry) => geometry,
                None => continue
            };

            let material = match self.reader.entity(row.get_material() as usize) {
                Some(material) => material,
                None => continue
            };

            return Some((
                id,
                DrawBinding{
                    geometry: geometry,
                    material: material
                }
            ));
        }
        None
    }
}

/// Used to read geometry colimn from the hairball
pub struct Reader<'a, E:'a> {
    reader: &'a hairball::ReaderMapping<'a, E>,
    column: capnp::struct_list::Reader<'a, draw_binding_capnp::draw_binding::Owned>,
    index: std::ops::Range<u32>
}

/// Create a column reader for iff the hairball has a valid geometry
/// column set
pub fn read<'a, E>(read: &'a hairball::ReaderMapping<'a, E>) -> Option<Reader<'a, E>>
    where E: 'a
{
    read.column(COLUMN_NAME)
        .and_then(|c| c.get_as().ok() )
        .and_then(|c: draw_binding_capnp::column::Reader<'a>| c.get_bindings().ok( ))
        .map(|c| {
            let len = c.len();
            Reader{
                reader: read,
                column: c,
                index: (0..len)
            }
        })
}
