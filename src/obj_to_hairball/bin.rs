extern crate obj;
extern crate genmesh;
extern crate gfx_mesh;
extern crate hairball;
extern crate hairball_mesh;
extern crate hairball_mesh_index;
extern crate hairball_material;
extern crate hairball_geometry;
extern crate hairball_draw_binding;

use std::fs::File;
use std::io::BufReader;
use std::env::args;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::{Path, PathBuf};

use genmesh::{
    Triangulate,
    MapToVertices,
    Vertices,
    LruIndexer,
    Indexer
};

use gfx_mesh::{Attribute, BuildInterlaced, Interlaced};

use hairball::LocalEntity;
use hairball_material::{Component, Value};

pub const POSITION: &'static str = "a_Position";
pub const NORMAL: &'static str = "a_Normal";
pub const TEX0: &'static str = "a_Tex0";

fn load_material<P>(p: P,
                    materials: u32,
                    builder: &mut hairball::Builder,
                    names: &mut HashMap<String, u32>,
                    bindings: &mut Vec<(u32, Component, Value)>) -> Result<(), std::io::Error>
    where P: AsRef<Path>
{
    let file = try!(File::open(p));
    let mat = obj::Mtl::load(&mut BufReader::new(file));
    for m in mat.materials {
        let mid = builder.add_entity(
            LocalEntity::named(m.name.clone())
                .parent(materials)
        );
        names.insert(m.name.clone(), mid);

        m.ka.map(|v| {
            bindings.push(
                (mid, Component::Ambient, Value::Color([v[0], v[1], v[2], 1.]))
            );
        });
        m.kd.map(|v| {
            bindings.push(
                (mid, Component::Diffuse, Value::Color([v[0], v[1], v[2], 1.]))
            );
        });
        m.ks.map(|v| {
            bindings.push(
                (mid, Component::Specular, Value::Color([v[0], v[1], v[2], 1.]))
            );
        });
    }
    Ok(())
}

fn main() {
    let mut args = args(); args.next();
    let obj_path = args.next().expect("Please supply a path for an obj");
    let hb_path = args.next().expect("please supply to write into");

    let object = Rc::new(File::open(&obj_path[..]).map(|f| {
        let mut f = BufReader::new(f);
        obj::Obj::load(&mut f)
    }).unwrap());


    let mut builder = hairball::Builder::new(hb_path).unwrap();

    let materials = builder.add_entity(LocalEntity::named("material".to_owned()));
    let geometry = builder.add_entity(LocalEntity::named("geometry".to_owned()));

    let mut material_names = HashMap::new();
    let mut material_binding = Vec::new();
    for m in object.materials().iter() {
        let mut p = PathBuf::from(&obj_path[..]);
        p.pop();
        p.push(&m[..]);
        load_material(p,
            materials,
            &mut builder,
            &mut material_names,
            &mut material_binding
        ).unwrap();
    }
    hairball_material::write(&mut builder, &material_binding[..]);

    let mut mesh = HashMap::new();
    for o in object.object_iter() {
        for g in o.group_iter() {
            let name = format!("{}.{}.{}", o.name, g.name, g.index);

            let mut vertices = Vec::new();
            let indices: Vec<u32> = {
                let object = object.clone();
                let mut indexer = LruIndexer::new(64, |_, v| {
                    let (p, t, n): (usize, Option<usize>, Option<usize>) = v;
                    let p = object.position()[p];
                    let t = t.map(|t| object.texture()[t]).unwrap_or([0., 0.]);
                    let n = n.map(|n| object.normal()[n]).unwrap_or([1., 0., 0.]);
                    vertices.push((p, n, t))
                });

                g.indices.iter()
                   .map(|x| *x)
                   .triangulate()
                   .vertex(|v| indexer.index(v) as u32)
                   .vertices()
                   .collect()
            };

            let vertices = [Attribute::f32(POSITION, 3), Attribute::f32(NORMAL, 3), Attribute::f32(TEX0, 2)]
                .build(vertices.into_iter())
                .unwrap()
                .owned_attributes();
            mesh.insert(name, (indices, vec![vertices], g));
        }
    }

    let mut name_to_id = HashMap::new();
    for name in mesh.keys() {
        name_to_id.insert(
            &name[..],
            builder.add_entity(
                LocalEntity::named(name.clone())
                    .parent(geometry)
            )
        );
    }

    let x: Vec<(u32, &Vec<u32>)> =
        mesh.iter().map(|(name, &(ref indices, _, _))| {
            (*name_to_id.get(&name[..]).unwrap(), indices)
        }).collect();
    hairball_mesh_index::write(&mut builder, &x[..]);

    let x: Vec<(u32, &Vec<Interlaced<Vec<Attribute<String>>, String, Vec<u8>>>)> =
        mesh.iter().map(|(name, &(_, ref mesh, _))| {
            (*name_to_id.get(&name[..]).unwrap(), mesh)
        }).collect();
    hairball_mesh::write(&mut builder, &x[..]);

    let x: Vec<(u32, hairball_geometry::Geometry<u32>)> =
        mesh.iter().map(|(name, &(ref idx, _, _))| {
            let name = *name_to_id.get(&name[..]).unwrap();
            (
                name,
                hairball_geometry::Geometry{
                    mesh: name,
                    start: 0,
                    length: idx.len() as u32
                }
            )
        }).collect();
    hairball_geometry::write(&mut builder, &x[..]);

    let x: Vec<(u32, hairball_draw_binding::DrawBinding<u32>)> =
        mesh.iter()
            .filter(|&(_, &(_, _, ref o))| o.material.is_some())
            .map(|(name, &(_, _, ref o))| {
                let name = *name_to_id.get(&name[..]).unwrap();
                let material = *material_names.get(o.material.as_ref().unwrap()).unwrap();
                (
                    name,
                    hairball_draw_binding::DrawBinding{
                        geometry: name,
                        material: material
                    }
                )
        }).collect();
    hairball_draw_binding::write(&mut builder, &x[..]);

    builder.close();
}