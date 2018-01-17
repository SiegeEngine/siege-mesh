
extern crate siege_mesh;
extern crate bincode;

use siege_mesh::{Mesh, StandardVertex};

pub fn main() {
    use std::fs::File;

    let cube_mesh = define_mesh();

    let mut f = File::create("cube_standard.mesh").unwrap();

    ::bincode::serialize_into(&mut f, &cube_mesh, ::bincode::Infinite).unwrap();

    println!("Serialized mesh.");
}

/*
Cubes have 8 vertices.
We need 14 in order to UV map properly
       7---6
       |tp |
   8---0---1---C---D
   | l |fr | r |bak|
   9---3---2---B---A
       |bt |
       4---5
 */

fn define_mesh() -> Mesh<StandardVertex> {
    let vertices = vec![
        StandardVertex { pos: [-1.0, -1.0, -1.0], // 0
                         uv: [0.25, 1.0/3.0] },
        StandardVertex { pos: [ 1.0, -1.0, -1.0], // 1
                         uv: [0.50, 1.0/3.0] },
        StandardVertex { pos: [ 1.0,  1.0, -1.0], // 2
                         uv: [0.50, 2.0/3.0] },
        StandardVertex { pos: [-1.0,  1.0, -1.0], // 3
                         uv: [0.25, 2.0/3.0] },
        StandardVertex { pos: [-1.0,  1.0,  1.0], // 4
                         uv: [0.25, 1.0] },
        StandardVertex { pos: [ 1.0,  1.0,  1.0], // 5
                         uv: [0.50, 1.0] },
        StandardVertex { pos: [ 1.0, -1.0,  1.0], // 6
                         uv: [0.50, 0.0] },
        StandardVertex { pos: [-1.0, -1.0,  1.0], // 7
                         uv: [0.25, 0.0] },
        StandardVertex { pos: [-1.0, -1.0,  1.0], // 8     coincident with 7
                         uv: [0.0, 1.0/3.0] },
        StandardVertex { pos: [-1.0,  1.0,  1.0], // 9     coincident with 4
                         uv: [0.0, 2.0/3.0] },
        StandardVertex { pos: [-1.0,  1.0,  1.0], // 10, A coincident with 4
                         uv: [1.0, 2.0/3.0] },
        StandardVertex { pos: [ 1.0,  1.0,  1.0], // 11, B coincidnet with 5
                         uv: [0.75, 2.0/3.0] },
        StandardVertex { pos: [ 1.0, -1.0,  1.0], // 12, C coincident with 6
                         uv: [0.75, 1.0/3.0] },
        StandardVertex { pos: [-1.0, -1.0,  1.0], // 13, D coincident with 7
                         uv: [1.0, 1.0/3.0] },
    ];

    let indices = vec![
        (0, 3, 2), (0, 2, 1), // front
        (3, 4, 5), (3, 5, 2), // bottom
        (7, 0, 1), (7, 1, 6), // top
        (8, 9, 3), (8, 3, 0), // left
        (1, 2, 11), (1, 11, 12), // right
        (12, 11, 10), (12, 10, 13_u16) // back
    ];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.strings = vec![
        ("pipeline".to_owned(), "standard".to_owned()),
        ("albedo".to_owned(), "cube_standard_albedomap.png".to_owned()),
        ("normal".to_owned(), "cube_standard_normalmap.png".to_owned())
    ];

    mesh
}
