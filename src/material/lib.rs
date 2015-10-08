extern crate capnp;
extern crate hairball;

#[cfg(feature="build-schema")]
pub mod material_capnp {
    include!(concat!(env!("OUT_DIR"), "/material_capnp.rs"));
}

const COLUMN_NAME: &'static str = "material";

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Texture(u32),
    Color([f32; 4])
}

#[derive(Copy, Clone, Debug)]
pub enum Component {
    Ambient,
    Diffuse,
    Specular
}

pub fn write(hb: &mut hairball::Builder, i: &[(u32, Component, Value)]) {
    use material_capnp::Component::*;

    let column: material_capnp::column::Builder = hb.column(COLUMN_NAME)
        .and_then(|c| c.get_as())
        .unwrap();

    let mut rows = column.init_bindings(i.len() as u32);
    for (i, &(name, c, v)) in i.iter().enumerate() {
        let mut row = rows.borrow().get(i as u32);
        row.set_id(name);
        row.set_component(match c {
            Component::Ambient => Ambient,
            Component::Diffuse => Diffuse,
            Component::Specular => Specular,
        });
        match v {
            Value::Texture(t) => {
                row.set_texture(t)
            }
            Value::Color(c) => {
                let mut color = row.init_color();
                color.set_red(c[0]);
                color.set_green(c[1]);
                color.set_blue(c[2]);
                color.set_alpha(c[3]);
            }
        }
    }
}

impl<'a, E> Iterator for Reader<'a, E> {
    type Item = (&'a E, Component, Value);

    fn next(&mut self) -> Option<Self::Item> {
        use material_capnp::Component::*;
        use material_capnp::binding::Which;

        if let Ok(meshes) = self.column.get_bindings() {
            while self.index < meshes.len() {
                let idx = self.index;
                self.index += 1;

                let m = meshes.get(idx);
                let id = match self.reader.entity(m.get_id() as usize) {
                    Some(id) => id,
                    None => continue
                };

                let comp = match m.get_component() {
                    Ok(Ambient) => Component::Ambient,
                    Ok(Diffuse) => Component::Diffuse,
                    Ok(Specular) => Component::Specular,
                    Err(_) => continue
                };

                let value = match m.which() {
                    Ok(Which::Texture(t)) => {
                        Value::Texture(t)
                    },
                    Ok(Which::Color(Ok(c))) => {
                        Value::Color([
                            c.get_red(),
                            c.get_green(),
                            c.get_blue(),
                            c.get_alpha()
                        ])
                    }
                    Ok(Which::Color(_)) | Err(_) => continue
                };
                return Some((id, comp, value));
            }
        }
        None
    }
}

pub struct Reader<'a, E:'a> {
    reader: &'a hairball::ReaderMapping<'a, E>,
    column: material_capnp::column::Reader<'a>,
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
