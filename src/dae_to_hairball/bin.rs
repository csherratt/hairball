extern crate collada;
extern crate genmesh;
extern crate gfx_mesh;
extern crate hairball;
extern crate hairball_mesh;
extern crate hairball_mesh_index;
extern crate hairball_material;
extern crate hairball_geometry;
extern crate hairball_draw_binding;

use std::env::args;
use std::collections::HashMap;
use collada::document::ColladaDocument;
use gfx_mesh::{BuildInterlaced, Interlaced, Attribute};
use hairball::LocalEntity;

use genmesh::{
    Triangulate,
    MapToVertices,
    Vertices,
    LruIndexer,
    Indexer,
    Triangle
};

pub const POSITION: &'static str = "a_Position";
pub const NORMAL: &'static str = "a_Normal";
pub const TEX0: &'static str = "a_Tex0";
pub const JOINT: &'static str = "a_Joint";
pub const WEIGHTS: &'static str = "a_Weight";

type attr = Interlaced<Vec<Attribute<String>>, String, Vec<u8>>;

fn position_to_mesh(o: &collada::Object) -> Vec<[f32; 3]>
{
    o.vertices
     .iter()
     .map(|v| [v.x as f32, v.y as f32, v.z as f32])
     .collect()
}

fn normal_to_mesh(o: &collada::Object) -> Vec<[f32; 3]>
{
    o.normals
     .iter()
     .map(|v| [v.x as f32, v.y as f32, v.z as f32])
     .collect()
}

fn texture_to_mesh(o: &collada::Object) -> Vec<[f32; 2]>
{
    o.tex_vertices
     .iter()
     .map(|v| [v.x as f32, v.y as f32])
     .collect()
}

fn weights_to_mesh(o: &collada::Object) -> Vec<([u32; 4], [f32; 4])>
{
    o.joint_weights
     .iter()
     .map(|v|
        (
            [v.joints[0] as u32, v.joints[1] as u32,
             v.joints[2] as u32, v.joints[3] as u32],
            v.weights
        )
     )
     .collect()
}

fn build_vbo(o: &collada::Object) -> Vec<(usize, Vec<Interlaced<Vec<Attribute<String>>, String, Vec<u8>>>, Vec<u32>)> {
    let position = position_to_mesh(o);
    let normal = normal_to_mesh(o);
    let texture = texture_to_mesh(o);
    let weights = weights_to_mesh(o);

    o.geometry.iter().enumerate()
        .map(|(gi, o)| {
            let mut vertices = Vec::new();
            let indices: Vec<u32> = {
                let mut indexer = LruIndexer::new(64, |_, v| {
                    let (p, t, n): (usize, Option<usize>, Option<usize>) = v;
                    let pos = position[p];
                    let t = t.map(|t| texture[t]).unwrap_or([0., 0.]);
                    let n = n.map(|n| normal[n]).unwrap_or([1., 0., 0.]);
                    let j = weights[p].0;
                    let w = weights[p].1;
                    vertices.push((pos, n, t, j, w))
                });

                o.shapes.iter()
                   .filter_map(|x| {
                        if let &collada::Shape::Triangle(x, y, z) = x {
                            Some(Triangle::new(x, y, z))
                        } else {
                            None
                        }
                    })
                   .triangulate()
                   .vertex(|v| indexer.index(v) as u32)
                   .vertices()
                   .collect()
            };

            let vertices = [Attribute::f32(POSITION, 3), Attribute::f32(NORMAL, 3),
                            Attribute::f32(TEX0,     2), Attribute::u32(JOINT,  4),
                            Attribute::f32(WEIGHTS,  4)]
                .build(vertices.into_iter())
                .unwrap()
                .owned_attributes();
            (gi, vec![vertices], indices)
        })
        .collect()
}


fn main() {
    let mut args = args(); args.next();
    let obj_path = args.next().expect("Please supply a path for an obj");
    let hb_path = args.next().expect("please supply to write into");

    let doc = ColladaDocument::from_path(obj_path.as_ref()).unwrap();
    let objs = doc.get_obj_set().unwrap();
    //println!("{:?}". doc.get_obj_set());

    let mut builder = hairball::Builder::new(hb_path).unwrap();

    let mut name_to_id = HashMap::new();
    for (oi, o) in objs.objects.iter().enumerate() {
        let p = builder.add_entity(LocalEntity::named(o.name.to_owned()));
        name_to_id.insert((oi, None), p);

        for (gi, _) in o.geometry.iter().enumerate() {
            let c =  builder.add_entity(LocalEntity::named(format!("{}", gi)).parent(p));
            name_to_id.insert((oi, Some(gi)), c);
        }
    }

    let meshes = objs.objects
        .iter()
        .enumerate()
        .map(|(oi, o)| (oi, build_vbo(o)))
        .collect::<Vec<_>>();

    let mut x: Vec<(u32, &Vec<u32>)> = Vec::new();
    for &(oi, ref o) in meshes.iter() {
        for &(gi, _, ref g) in o {
            x.push((name_to_id[&(oi, Some(gi))], g));
        }
    }
    hairball_mesh_index::write(&mut builder, &x[..]);

    let mut x: Vec<(u32, &Vec<Interlaced<Vec<Attribute<String>>, String, Vec<u8>>>)> = Vec::new();
    for &(oi, ref o) in meshes.iter() {
        for &(gi, ref g, _) in o {
            x.push((name_to_id[&(oi, Some(gi))], g));
        }
    }
    hairball_mesh::write(&mut builder, &x[..]);

    let mut x: Vec<(u32, hairball_geometry::Geometry<u32>)> = Vec::new();
    for &(oi, ref o) in meshes.iter() {
        for &(gi, _, ref idx) in o {
            x.push(
                (
                    name_to_id[&(oi, Some(gi))],
                    hairball_geometry::Geometry{
                        mesh: name_to_id[&(oi, Some(gi))],
                        start: 0,
                        length: idx.len() as u32
                    }
                )
            );
        }
    }
    hairball_geometry::write(&mut builder, &x[..]);
    builder.close();
}
