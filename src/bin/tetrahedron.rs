extern crate bincode;
extern crate siege_mesh;

use siege_mesh::{ColoredVertex, Mesh, Point3, VertexType};
use std::f32;

pub fn main() {
    let mesh = define_mesh();
    mesh.save(VertexType::Colored, "tetrahedron.mesh").unwrap();
    println!("Saved.");
}

const ISQRT3: f32 = 0.5773502691896258;

fn define_mesh() -> Mesh<ColoredVertex> {
    let vertices = vec![
        ColoredVertex {
            pos: [1.0, 1.0, -1.0],
            color: [1.0, 0.0, 0.0],
            normal: [ISQRT3, -ISQRT3, -ISQRT3],
        },
        ColoredVertex {
            pos: [1.0, -1.0, 1.0],
            color: [1.0, 0.0, 0.0],
            normal: [ISQRT3, -ISQRT3, -ISQRT3],
        },
        ColoredVertex {
            pos: [-1.0, -1.0, -1.0],
            color: [1.0, 0.0, 0.0],
            normal: [ISQRT3, -ISQRT3, -ISQRT3],
        },
        ColoredVertex {
            pos: [-1.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0],
            normal: [-ISQRT3, ISQRT3, -ISQRT3],
        },
        ColoredVertex {
            pos: [1.0, 1.0, -1.0],
            color: [1.0, 1.0, 1.0],
            normal: [-ISQRT3, ISQRT3, -ISQRT3],
        },
        ColoredVertex {
            pos: [-1.0, -1.0, -1.0],
            color: [1.0, 1.0, 1.0],
            normal: [-ISQRT3, ISQRT3, -ISQRT3],
        },
        ColoredVertex {
            pos: [-1.0, 1.0, 1.0],
            color: [0.0, 1.0, 0.0],
            normal: [ISQRT3, ISQRT3, ISQRT3],
        },
        ColoredVertex {
            pos: [1.0, -1.0, 1.0],
            color: [0.0, 1.0, 0.0],
            normal: [ISQRT3, ISQRT3, ISQRT3],
        },
        ColoredVertex {
            pos: [1.0, 1.0, -1.0],
            color: [0.0, 1.0, 0.0],
            normal: [ISQRT3, ISQRT3, ISQRT3],
        },
        ColoredVertex {
            pos: [-1.0, 1.0, 1.0],
            color: [0.0, 0.0, 1.0],
            normal: [-ISQRT3, -ISQRT3, ISQRT3],
        },
        ColoredVertex {
            pos: [-1.0, -1.0, -1.0],
            color: [0.0, 0.0, 1.0],
            normal: [-ISQRT3, -ISQRT3, ISQRT3],
        },
        ColoredVertex {
            pos: [1.0, -1.0, 1.0],
            color: [0.0, 0.0, 1.0],
            normal: [-ISQRT3, -ISQRT3, ISQRT3],
        },
    ];

    let indices = vec![(0, 1, 2), (3, 4, 5), (6, 7, 8), (9, 10, 11_u16)];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.bounding_sphere = Some((Point3::new(0.0, 0.0, 0.0), (3.0_f32).sqrt()));

    mesh
}
