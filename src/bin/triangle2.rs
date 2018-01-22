
extern crate siege_mesh;
extern crate bincode;

use siege_mesh::{Mesh, ColoredVertex, VertexType};

pub fn main() {
    let mesh = define_mesh();
    mesh.save(VertexType::Colored, "triangle2.mesh").unwrap();
    println!("Saved.");
}

fn define_mesh() -> Mesh<ColoredVertex> {

    let vertices = vec![
        ColoredVertex { pos: [-0.97, 0.93, 1.0], color: [1.0, 0.0, 0.0],
                        normal: [ 0.0, 0.0, -1.0 ]},
        ColoredVertex { pos: [-0.95, 0.95, 0.0], color: [1.0, 0.0, 0.0],
                        normal: [ 0.0, 0.0, -1.0 ]},
        ColoredVertex { pos: [-0.93, 0.93, 1.0], color: [1.0, 0.0, 0.0],
                        normal: [ 0.0, 0.0, -1.0 ]},
    ];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;

    mesh
}