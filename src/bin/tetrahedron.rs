
extern crate siege_mesh;
extern crate bincode;

use std::f32;
use siege_mesh::{Mesh, ColoredVertex, VertexType};

pub fn main() {
    use std::fs::File;

    let mesh = define_mesh();

    let mut f = File::create("tetrahedron.mesh").unwrap();

    ::bincode::serialize_into(&mut f, &mesh, ::bincode::Infinite).unwrap();

    println!("Serialized mesh.");
}

const ISQRT3: f32 = 0.5773502691896258;

fn define_mesh() -> Mesh<ColoredVertex> {

    let vertices = vec![
        ColoredVertex { pos: [ 1.0, 1.0,-1.0], color: [1.0, 0.0, 0.0], normal: [ISQRT3, -ISQRT3, -ISQRT3] },
        ColoredVertex { pos: [ 1.0,-1.0, 1.0], color: [1.0, 0.0, 0.0], normal: [ISQRT3, -ISQRT3, -ISQRT3] },
        ColoredVertex { pos: [-1.0,-1.0,-1.0], color: [1.0, 0.0, 0.0], normal: [ISQRT3, -ISQRT3, -ISQRT3] },

        ColoredVertex { pos: [-1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0], normal: [-ISQRT3, ISQRT3, -ISQRT3] },
        ColoredVertex { pos: [ 1.0, 1.0,-1.0], color: [1.0, 1.0, 1.0], normal: [-ISQRT3, ISQRT3, -ISQRT3] },
        ColoredVertex { pos: [-1.0,-1.0,-1.0], color: [1.0, 1.0, 1.0], normal: [-ISQRT3, ISQRT3, -ISQRT3] },

        ColoredVertex { pos: [-1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0], normal: [ISQRT3, ISQRT3, ISQRT3] },
        ColoredVertex { pos: [ 1.0,-1.0, 1.0], color: [0.0, 1.0, 0.0], normal: [ISQRT3, ISQRT3, ISQRT3] },
        ColoredVertex { pos: [ 1.0, 1.0,-1.0], color: [0.0, 1.0, 0.0], normal: [ISQRT3, ISQRT3, ISQRT3] },

        ColoredVertex { pos: [-1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0], normal: [-ISQRT3, -ISQRT3, ISQRT3] },
        ColoredVertex { pos: [-1.0,-1.0,-1.0], color: [0.0, 0.0, 1.0], normal: [-ISQRT3, -ISQRT3, ISQRT3] },
        ColoredVertex { pos: [ 1.0,-1.0, 1.0], color: [0.0, 0.0, 1.0], normal: [-ISQRT3, -ISQRT3, ISQRT3] },
    ];

    let indices = vec![
        (0,  1,  2), (3, 4, 5),
        (6,  7,  8), (9, 10, 11_u16)];

    let mut mesh = Mesh::new(VertexType::Colored);
    mesh.vertices = vertices;
    mesh.indices = indices;

    mesh

}
