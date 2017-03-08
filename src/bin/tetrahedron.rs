
extern crate siege_mesh;
extern crate bincode;

use std::f32;
use siege_mesh::{Mesh, VertexPCN, V_PCN};

pub fn main() {
    use std::fs::File;
    use std::io::Write;

    let mesh = define_mesh();

    let mut f = File::create("/tmp/tetrahedron.mesh").unwrap();

    let vertex_type: [u8; 1] = [V_PCN; 1];
    f.write(&vertex_type).unwrap();

    ::bincode::serialize_into(&mut f, &mesh, ::bincode::SizeLimit::Infinite).unwrap();

    println!("Serialized mesh.");
}

const ISQRT3: f32 = 0.5773502691896258;

fn define_mesh() -> Mesh<VertexPCN> {

    let vertices = vec![
        VertexPCN { pos: [ 1.0, 1.0,-1.0], color: [1.0, 0.0, 0.0], normal: [ISQRT3, -ISQRT3, -ISQRT3] },
        VertexPCN { pos: [ 1.0,-1.0, 1.0], color: [1.0, 0.0, 0.0], normal: [ISQRT3, -ISQRT3, -ISQRT3] },
        VertexPCN { pos: [-1.0,-1.0,-1.0], color: [1.0, 0.0, 0.0], normal: [ISQRT3, -ISQRT3, -ISQRT3] },

        VertexPCN { pos: [-1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0], normal: [-ISQRT3, ISQRT3, -ISQRT3] },
        VertexPCN { pos: [ 1.0, 1.0,-1.0], color: [1.0, 1.0, 1.0], normal: [-ISQRT3, ISQRT3, -ISQRT3] },
        VertexPCN { pos: [-1.0,-1.0,-1.0], color: [1.0, 1.0, 1.0], normal: [-ISQRT3, ISQRT3, -ISQRT3] },

        VertexPCN { pos: [-1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0], normal: [ISQRT3, ISQRT3, ISQRT3] },
        VertexPCN { pos: [ 1.0,-1.0, 1.0], color: [0.0, 1.0, 0.0], normal: [ISQRT3, ISQRT3, ISQRT3] },
        VertexPCN { pos: [ 1.0, 1.0,-1.0], color: [0.0, 1.0, 0.0], normal: [ISQRT3, ISQRT3, ISQRT3] },

        VertexPCN { pos: [-1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0], normal: [-ISQRT3, -ISQRT3, ISQRT3] },
        VertexPCN { pos: [-1.0,-1.0,-1.0], color: [0.0, 0.0, 1.0], normal: [-ISQRT3, -ISQRT3, ISQRT3] },
        VertexPCN { pos: [ 1.0,-1.0, 1.0], color: [0.0, 0.0, 1.0], normal: [-ISQRT3, -ISQRT3, ISQRT3] },
    ];

    let indices = vec![
        (0,  1,  2), (3, 4, 5),
        (6,  7,  8), (9, 10, 11_u32)];

    Mesh {
        vertices: vertices,
        indices: indices,
        ..Default::default()
    }
}
