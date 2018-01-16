
extern crate siege_mesh;
extern crate bincode;

use siege_mesh::{Mesh, VertexPU, V_PU};

pub fn main() {
    use std::fs::File;
    use std::io::Write;

    let cube_mesh = define_mesh();

    let mut f = File::create("/tmp/cube.pu.mesh").unwrap();

    let vertex_type: [u8; 1] = [V_PU; 1];
    f.write(&vertex_type).unwrap();

    ::bincode::serialize_into(&mut f, &cube_mesh, ::bincode::Infinite).unwrap();

    println!("Serialized mesh.");
}

// UV MAPPING IS SLOPPY, come back to fix that.

fn define_mesh() -> Mesh<VertexPU> {
    let vertices = vec![
        // Front (orange)
        VertexPU { pos: [-1.0, -1.0, -1.0], // left upper front  0
                   uv: [0.0, 0.0] },
        VertexPU { pos: [-1.0,  1.0, -1.0], // left lower front  1
                   uv: [0.0, 1.0] },
        VertexPU { pos: [ 1.0,  1.0, -1.0], // right lower front 2
                   uv: [1.0, 1.0] },
        VertexPU { pos: [ 1.0, -1.0, -1.0], // right upper front 3
                   uv: [1.0, 0.0] },
        // Top (yellow)
        VertexPU { pos: [-1.0, -1.0, -1.0], // left upper front  4
                    uv: [0.0, 1.0] },
        VertexPU { pos: [-1.0, -1.0, 1.0], // left upper rear    5
                   uv: [0.0, 0.0] },
        VertexPU { pos: [1.0, -1.0, 1.0], // right upper rear    6
                   uv: [1.0, 0.0] },
        VertexPU { pos: [1.0, -1.0, -1.0], // right upper front  7
                   uv: [1.0, 1.0] },
        // Back (red)
        VertexPU { pos: [-1.0,  -1.0,  1.0], // left upper rear  8
                   uv: [1.0, 0.0] },
        VertexPU { pos: [-1.0,  1.0,  1.0], // left lower rear   9
                   uv: [1.0, 1.0] },
        VertexPU { pos: [ 1.0,  1.0,  1.0], // right lower rear  10
                   uv: [0.0, 1.0] },
        VertexPU { pos: [ 1.0, -1.0,  1.0], // right upper rear  11
                   uv: [0.0, 0.0] },
        // Bottom (green)
        VertexPU { pos: [-1.0,  1.0, -1.0], // left lower front  12
                   uv: [1.0, 0.0] },
        VertexPU { pos: [-1.0,  1.0,  1.0], // left lower rear   13
                   uv: [1.0, 1.0] },
        VertexPU { pos: [ 1.0,  1.0,  1.0], // right lower rear  14
                   uv: [0.0, 1.0] },
        VertexPU { pos: [ 1.0,  1.0, -1.0], // right lower front 15
                   uv: [0.0, 0.0] },
        // Left (blue)
        VertexPU { pos: [-1.0, -1.0, -1.0], // left upper front  16
                   uv: [1.0, 0.0] },
        VertexPU { pos: [-1.0, -1.0,  1.0], // left upper rear   17
                   uv: [1.0, 1.0] },
        VertexPU { pos: [-1.0,  1.0,  1.0], // left lower rear   18
                   uv: [0.0, 1.0] },
        VertexPU { pos: [-1.0,  1.0, -1.0],  // left lower front 19
                   uv: [0.0, 0.0] },
        // Right (purple)
        VertexPU { pos: [ 1.0, -1.0, -1.0], // right upper front 20
                   uv: [1.0, 0.0] },
        VertexPU { pos: [ 1.0, -1.0,  1.0], // right upper rear  21
                   uv: [1.0, 1.0] },
        VertexPU { pos: [ 1.0,  1.0,  1.0], // right lower rear  22
                   uv: [0.0, 1.0] },
        VertexPU { pos: [ 1.0,  1.0, -1.0], // right lower front 23
                   uv: [0.0, 0.0] },
    ];

    let indices = vec![
        (0,  1,  2), (0,  2,  3),
        (5,  4,  7), (5,  7,  6),
        (12, 13, 14), (12, 14, 15),
        (9,  8,  11), (9,  11, 10),
        (17, 18, 19), (17, 19, 16),
        (20, 23, 22), (20, 22, 21u32)];

    Mesh {
        vertices: vertices,
        indices: indices,
        ..Default::default()
    }
}
