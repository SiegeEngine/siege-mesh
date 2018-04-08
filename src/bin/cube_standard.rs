
extern crate siege_mesh;
extern crate siege_math;
extern crate bincode;

use siege_math::Point3;
use siege_mesh::{Mesh, StandardVertex, VertexType};

pub fn main() {
    let cube_mesh = define_mesh();
    cube_mesh.save(VertexType::Standard, "cube_standard.mesh").unwrap();
    println!("Saved.");
}

/*
Cubes have 8 vertices.

We need 24, however, to get the normals right (and we would need 14 to UV map
anyways)

Texture layout puts the same texture on all 6 faces.
 */

fn define_mesh() -> Mesh<StandardVertex> {
    let vertices = vec![
        // Front
        StandardVertex { pos: [-1.0, -1.0, -1.0], // left upper front  0
                         normal: [0.0, 0.0, -1.0],
                         uv: [0.0, 0.0],
        },
        StandardVertex { pos: [-1.0,  1.0, -1.0], // left lower front  1
                         normal: [0.0, 0.0, -1.0],
                         uv: [0.0, 1.0],
        },
        StandardVertex { pos: [ 1.0,  1.0, -1.0], // right lower front 2
                         normal: [0.0, 0.0, -1.0],
                         uv: [1.0, 1.0],
        },
        StandardVertex { pos: [ 1.0, -1.0, -1.0], // right upper front 3
                         normal: [0.0, 0.0, -1.0],
                         uv: [1.0, 0.0],
        },
        // Top
        StandardVertex { pos: [-1.0, -1.0, -1.0], // left upper front  4
                         normal: [0.0, -1.0, 0.0],
                         uv: [0.0, 1.0],
        },
        StandardVertex { pos: [-1.0, -1.0, 1.0], // left upper rear    5
                         normal: [0.0, -1.0, 0.0],
                         uv: [0.0, 0.0],
        },
        StandardVertex { pos: [ 1.0, -1.0, 1.0], // right upper rear    6
                         normal: [0.0, -1.0, 0.0],
                         uv: [1.0, 0.0],
        },
        StandardVertex { pos: [1.0, -1.0,  -1.0], // right upper front  7
                         normal: [0.0, -1.0, 0.0],
                         uv: [1.0, 1.1],
        },
        // Back
        StandardVertex { pos: [-1.0,  -1.0, 1.0], // left upper rear  8
                         normal: [0.0, 0.0, 1.0],
                         uv: [1.0, 0.0],
        },
        StandardVertex { pos: [-1.0,  1.0,  1.0], // left lower rear   9
                         normal: [0.0, 0.0, 1.0],
                         uv: [1.0, 1.0],
        },
        StandardVertex { pos: [ 1.0,  1.0,  1.0], // right lower rear  10
                         normal: [0.0, 0.0, 1.0],
                         uv: [0.0, 1.0],
        },
        StandardVertex { pos: [ 1.0, -1.0,  1.0], // right upper rear  11
                         normal: [0.0, 0.0, 1.0],
                         uv: [0.0, 0.0,],
        },
        // Bottom
        StandardVertex { pos: [-1.0,  1.0,  -1.0], // left lower front  12
                         normal: [0.0, 1.0, 0.0],
                         uv: [0.0, 0.0],
        },
        StandardVertex { pos: [-1.0,  1.0, 1.0], // left lower rear   13
                         normal: [0.0, 1.0, 0.0],
                         uv: [0.0, 1.0],
        },
        StandardVertex { pos: [ 1.0,  1.0, 1.0], // right lower rear  14
                         normal: [0.0, 1.0, 0.0],
                         uv: [1.0, 1.0],
        },
        StandardVertex { pos: [ 1.0,  1.0,  -1.0], // right lower front 15
                         normal: [0.0, 1.0, 0.0],
                         uv: [1.0, 0.0],
        },
        // Left
        StandardVertex { pos: [-1.0, -1.0,  -1.0], // left upper front  16
                         normal: [-1.0, 0.0, 0.0],
                         uv: [1.0, 0.0],
        },
        StandardVertex { pos: [-1.0, -1.0, 1.0], // left upper rear   17
                         normal: [-1.0, 0.0, 0.0],
                         uv: [0.0, 0.0],
        },
        StandardVertex { pos: [-1.0,  1.0, 1.0], // left lower rear   18
                         normal: [-1.0, 0.0, 0.0],
                         uv: [0.0, 1.0],
        },
        StandardVertex { pos: [-1.0,  1.0,  -1.0],  // left lower front 19
                         normal: [-1.0, 0.0, 0.0],
                         uv: [1.0, 1.0],
        },
        // Right
        StandardVertex { pos: [ 1.0, -1.0,  -1.0], // right upper front 20
                         normal: [ 1.0, 0.0, 0.0],
                         uv: [0.0, 0.0],
        },
        StandardVertex { pos: [ 1.0, -1.0, 1.0], // right upper rear  21
                         normal: [ 1.0, 0.0, 0.0],
                         uv: [1.0, 0.0],
        },
        StandardVertex { pos: [ 1.0,  1.0, 1.0], // right lower rear  22
                         normal: [ 1.0, 0.0, 0.0],
                         uv: [1.0, 1.0],
        },
        StandardVertex { pos: [ 1.0,  1.0,  -1.0], // right lower front 23
                         normal: [ 1.0, 0.0, 0.0],
                         uv: [0.0, 1.0],
        },
    ];

    let indices = vec![
        (0,  1,  2),  (0,  2,  3),
        (5,  4,  7),  (5,  7,  6),
        (12, 13, 14), (12, 14, 15),
        (9,  8,  11), (9,  11, 10),
        (17, 18, 19), (17, 19, 16),
        (20, 23, 22), (20, 22, 21u16)
    ];

    let bounding_cuboid = [
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, -1.0),
        Point3::new(1.0, -1.0, 1.0),
        Point3::new(1.0, -1.0, -1.0),
        Point3::new(-1.0, 1.0, 1.0),
        Point3::new(-1.0, 1.0, -1.0),
        Point3::new(-1.0, -1.0, 1.0),
        Point3::new(-1.0, -1.0, -1.0)];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.bounding_cuboid = Some(bounding_cuboid);
    mesh.strings = vec![
        ("pipeline".to_owned(), "standard".to_owned()),
        ("albedo".to_owned(), "cube_standard_albedomap.bc7.dds.zst".to_owned()),
        ("normal".to_owned(), "cube_standard_normalmap.bc7.dds.zst".to_owned())
    ];

    mesh
}
