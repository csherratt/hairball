extern crate obj;
extern crate genmesh;
extern crate gfx_mesh;
extern crate hairball;
extern crate hairball_mesh;
extern crate hairball_mesh_index;

use std::fs::File;
use std::io::BufReader;
use std::env::args;
use std::collections::HashMap;
use std::rc::Rc;

use genmesh::{
    Triangulate,
    MapToVertices,
    Vertices,
    LruIndexer,
    Indexer
};

use gfx_mesh::{Attribute, BuildInterlaced, Interlaced};

use hairball::LocalEntity;

pub const POSITION: &'static str = "a_Position";
pub const NORMAL: &'static str = "a_Normal";
pub const TEX0: &'static str = "a_Tex0";

fn main() {
    let mut args = args(); args.next();
    let obj_path = args.next().expect("Please supply a path for an obj");
    let hb_path = args.next().expect("please supply to write into");

    let object = Rc::new(File::open(obj_path).map(|f| {
        let mut f = BufReader::new(f);
        obj::Obj::load(&mut f)
    }).unwrap());

    let mut builder = hairball::Builder::new(hb_path).unwrap();
    let mut hm = HashMap::new();

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

            hm.insert(name, (indices, vec![vertices]));
        }
    }

    let mut name_to_id = HashMap::new();
    for name in hm.keys() {
        name_to_id.insert(
            &name[..],
            builder.add_entity(LocalEntity::named(name.clone()))
        );
    }

    let x: Vec<(u32, &Vec<u32>)> = hm.iter().map(|(name, &(ref indices, _))|{
        (*name_to_id.get(&name[..]).unwrap(), indices)
    }).collect();
    hairball_mesh_index::write(&mut builder, &x[..]);


    let x: Vec<(u32, &Vec<Interlaced<Vec<Attribute<String>>, String, Vec<u8>>>)> = hm.iter().map(|(name, &(_, ref mesh))|{
        (*name_to_id.get(&name[..]).unwrap(), mesh)
    }).collect();
    hairball_mesh::write(&mut builder, &x[..]);
    builder.close();
}