
extern crate siege_mesh;
extern crate bincode;

use siege_mesh::{Mesh, CubemapVertex, VertexType};

pub fn main() {
    let cube_mesh = define_mesh();
    cube_mesh.save(VertexType::Cubemap, "cubemap.mesh").unwrap();
    println!("Saved.");
}

fn define_mesh() -> Mesh<CubemapVertex> {
    // We define these in the same order that vulkan CUBE ARRAY layers are
    // ordered:  +X, -X, +Y, -Y, +Z, -Z.
    // Not yet sure about the corner ordering though.

    let vertices = vec![
        // Right (+X)
        CubemapVertex { pos: [ 1.0, -1.0,  9.0], // right upper front
                        uvw: [ 0.0, 0.0, 0.0],
        },
        CubemapVertex { pos: [ 1.0, -1.0, 11.0], // right upper rear
                        uvw: [ 1.0, 0.0, 0.0],
        },
        CubemapVertex { pos: [ 1.0,  1.0,  9.0], // right lower front
                        uvw: [ 0.0, 1.0, 0.0],
        },
        CubemapVertex { pos: [ 1.0,  1.0, 11.0], // right lower rear
                        uvw: [ 1.0, 1.0, 0.0],
        },
        // Left (-X)
        CubemapVertex { pos: [-1.0, -1.0, 11.0], // left upper rear
                        uvw: [ 0.0, 0.0, 1.0],
        },
        CubemapVertex { pos: [-1.0, -1.0,  9.0], // left upper front
                        uvw: [ 1.0, 0.0, 1.0],
        },
        CubemapVertex { pos: [-1.0,  1.0, 11.0], // left lower rear
                        uvw: [ 0.0, 1.0, 1.0],
        },
        CubemapVertex { pos: [-1.0,  1.0,  9.0],  // left lower front
                        uvw: [ 1.0, 1.0, 1.0],
        },
        // Bottom (+Y)
        CubemapVertex { pos: [-1.0,  1.0,  9.0], // left lower front
                        uvw: [ 0.0, 0.0, 2.0],
        },
        CubemapVertex { pos: [ 1.0,  1.0,  9.0], // right lower front
                        uvw: [ 1.0, 0.0, 2.0],
        },
        CubemapVertex { pos: [-1.0,  1.0, 11.0], // left lower rear
                        uvw: [ 0.0, 1.0, 2.0],
        },
        CubemapVertex { pos: [ 1.0,  1.0, 11.0], // right lower rear
                        uvw: [ 1.0, 1.0, 2.0],
        },
        // Top (-Y)
        CubemapVertex { pos: [-1.0, -1.0, 11.0], // left upper rear
                        uvw: [ 0.0, 0.0, 3.0],
        },
        CubemapVertex { pos: [ 1.0, -1.0, 11.0], // right upper rear
                        uvw: [ 1.0, 0.0, 3.0],
        },
        CubemapVertex { pos: [-1.0, -1.0, 9.0], // left upper front
                        uvw: [ 0.0, 1.0, 3.0],
        },
        CubemapVertex { pos: [1.0, -1.0,  9.0], // right upper front
                        uvw: [ 1.0, 1.0, 3.0],
        },
        // Back (+Z)
        CubemapVertex { pos: [ 1.0, -1.0,  11.0], // right upper rear
                        uvw: [ 0.0, 0.0, 4.0],
        },
        CubemapVertex { pos: [-1.0,  -1.0, 11.0], // left upper rear
                        uvw: [ 1.0, 0.0, 4.0],
        },
        CubemapVertex { pos: [ 1.0,  1.0,  11.0], // right lower rear
                        uvw: [ 0.0, 1.0, 4.0],
        },
        CubemapVertex { pos: [-1.0,  1.0,  11.0], // left lower rear
                        uvw: [ 1.0, 1.0, 4.0],
        },
        // Front (-Z)
        CubemapVertex { pos: [-1.0, -1.0, 9.0], // left upper front
                        uvw: [ 0.0, 0.0, 5.0],
        },
        CubemapVertex { pos: [ 1.0, -1.0, 9.0], // right upper front
                        uvw: [ 1.0, 0.0, 5.0],
        },
        CubemapVertex { pos: [-1.0,  1.0, 9.0], // left lower front
                        uvw: [ 0.0, 1.0, 5.0],
        },
        CubemapVertex { pos: [ 1.0,  1.0, 9.0], // right lower front
                        uvw: [ 1.0, 1.0, 5.0],
        },
    ];

    let indices = vec![
        (2,   1,  0),  ( 1,  2,  3),
        (6,   5,  4),  ( 5,  6,  7),
        (10,  9,  8),  ( 9, 10, 11),
        (14, 13, 12),  (13, 14, 15),
        (18, 17, 16),  (17, 18, 19),
        (22, 21, 20),  (21, 22, 23_u16),
    ];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.strings = vec![];

    mesh
}
