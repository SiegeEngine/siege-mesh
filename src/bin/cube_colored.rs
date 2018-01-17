
extern crate siege_mesh;
extern crate bincode;

use siege_mesh::{Mesh, ColoredVertex};

pub fn main() {
    use std::fs::File;

    let cube_mesh = define_mesh();

    let mut f = File::create("cube_colored.mesh").unwrap();

    ::bincode::serialize_into(&mut f, &cube_mesh, ::bincode::Infinite).unwrap();

    println!("Serialized mesh.");
}

fn define_mesh() -> Mesh<ColoredVertex> {
    let vertices = vec![
        // Front (orange)
        ColoredVertex { pos: [-1.0, -1.0, -1.0], // left upper front  0
                        color: [1.0, 0.5, 0.0],
                        normal: [0.0, 0.0, -1.0] },
        ColoredVertex { pos: [-1.0,  1.0, -1.0], // left lower front  1
                        color: [1.0, 0.5, 0.0],
                        normal: [0.0, 0.0, -1.0] },
        ColoredVertex { pos: [ 1.0,  1.0, -1.0], // right lower front 2
                        color: [1.0, 0.5, 0.0],
                        normal: [0.0, 0.0, -1.0] },
        ColoredVertex { pos: [ 1.0, -1.0, -1.0], // right upper front 3
                        color: [1.0, 0.5, 0.0],
                        normal: [0.0, 0.0, -1.0] },
        // Top (yellow)
        ColoredVertex { pos: [-1.0, -1.0, -1.0], // left upper front  4
                        color: [1.0, 1.0, 0.0],
                        normal: [0.0, -1.0, 0.0] },
        ColoredVertex { pos: [-1.0, -1.0, 1.0], // left upper rear    5
                        color: [1.0, 1.0, 0.0],
                        normal: [0.0, -1.0, 0.0] },
        ColoredVertex { pos: [1.0, -1.0, 1.0], // right upper rear    6
                        color: [1.0, 1.0, 0.0],
                        normal: [0.0, -1.0, 0.0] },
        ColoredVertex { pos: [1.0, -1.0, -1.0], // right upper front  7
                        color: [1.0, 1.0, 0.0],
                        normal: [0.0, -1.0, 0.0] },
        // Back (red)
        ColoredVertex { pos: [-1.0,  -1.0,  1.0], // left upper rear  8
                        color: [1.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0] },
        ColoredVertex { pos: [-1.0,  1.0,  1.0], // left lower rear   9
                        color: [1.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0] },
        ColoredVertex { pos: [ 1.0,  1.0,  1.0], // right lower rear  10
                        color: [1.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0] },
        ColoredVertex { pos: [ 1.0, -1.0,  1.0], // right upper rear  11
                        color: [1.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0] },
        // Bottom (green)
        ColoredVertex { pos: [-1.0,  1.0, -1.0], // left lower front  12
                        color: [0.0, 1.0, 0.0],
                        normal: [0.0, 1.0, 0.0] },
        ColoredVertex { pos: [-1.0,  1.0,  1.0], // left lower rear   13
                        color: [0.0, 1.0, 0.0],
                        normal: [0.0, 1.0, 0.0] },
        ColoredVertex { pos: [ 1.0,  1.0,  1.0], // right lower rear  14
                        color: [0.0, 1.0, 0.0],
                        normal: [0.0, 1.0, 0.0] },
        ColoredVertex { pos: [ 1.0,  1.0, -1.0], // right lower front 15
                        color: [0.0, 1.0, 0.0],
                        normal: [0.0, 1.0, 0.0] },
        // Left (blue)
        ColoredVertex { pos: [-1.0, -1.0, -1.0], // left upper front  16
                        color: [0.0, 0.0, 1.0],
                        normal: [-1.0, 0.0, 0.0] },
        ColoredVertex { pos: [-1.0, -1.0,  1.0], // left upper rear   17
                        color: [0.0, 0.0, 1.0],
                        normal: [-1.0, 0.0, 0.0] },
        ColoredVertex { pos: [-1.0,  1.0,  1.0], // left lower rear   18
                        color: [0.0, 0.0, 1.0],
                        normal: [-1.0, 0.0, 0.0] },
        ColoredVertex { pos: [-1.0,  1.0, -1.0],  // left lower front 19
                        color: [0.0, 0.0, 1.0],
                        normal: [-1.0, 0.0, 0.0] },
        // Right (purple)
        ColoredVertex { pos: [ 1.0, -1.0, -1.0], // right upper front 20
                        color: [0.56, 0.0, 1.0],
                        normal: [ 1.0, 0.0, 0.0] },
        ColoredVertex { pos: [ 1.0, -1.0,  1.0], // right upper rear  21
                        color: [0.56, 0.0, 1.0],
                        normal: [ 1.0, 0.0, 0.0] },
        ColoredVertex { pos: [ 1.0,  1.0,  1.0], // right lower rear  22
                        color: [0.56, 0.0, 1.0],
                        normal: [ 1.0, 0.0, 0.0] },
        ColoredVertex { pos: [ 1.0,  1.0, -1.0], // right lower front 23
                        color: [0.56, 0.0, 1.0],
                        normal: [ 1.0, 0.0, 0.0] },
    ];

    let indices = vec![
        (0,  1,  2), (0,  2,  3),
        (5,  4,  7), (5,  7,  6),
        (12, 13, 14), (12, 14, 15),
        (9,  8,  11), (9,  11, 10),
        (17, 18, 19), (17, 19, 16),
        (20, 23, 22), (20, 22, 21u16)];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;
    mesh.indices = indices;

    mesh
}
