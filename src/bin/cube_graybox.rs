extern crate bincode;
extern crate siege_mesh;

use siege_mesh::{GrayboxVertex, Mesh, Point3, VertexType};

pub fn main() {
    let cube_mesh = define_mesh();
    cube_mesh
        .save(VertexType::Graybox, "cube_graybox.mesh")
        .unwrap();
    println!("Saved.");
}

fn define_mesh() -> Mesh<GrayboxVertex> {
    let vertices = vec![
        // Front
        GrayboxVertex {
            pos: [-1.0, -1.0, -1.0], // left upper front  0
            normal: [0.0, 0.0, -1.0],
        },
        GrayboxVertex {
            pos: [-1.0, 1.0, -1.0], // left lower front  1
            normal: [0.0, 0.0, -1.0],
        },
        GrayboxVertex {
            pos: [1.0, 1.0, -1.0], // right lower front 2
            normal: [0.0, 0.0, -1.0],
        },
        GrayboxVertex {
            pos: [1.0, -1.0, -1.0], // right upper front 3
            normal: [0.0, 0.0, -1.0],
        },
        // Top
        GrayboxVertex {
            pos: [-1.0, -1.0, -1.0], // left upper front  4
            normal: [0.0, -1.0, 0.0],
        },
        GrayboxVertex {
            pos: [-1.0, -1.0, 1.0], // left upper rear    5
            normal: [0.0, -1.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, -1.0, 1.0], // right upper rear    6
            normal: [0.0, -1.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, -1.0, -1.0], // right upper front  7
            normal: [0.0, -1.0, 0.0],
        },
        // Back
        GrayboxVertex {
            pos: [-1.0, -1.0, 1.0], // left upper rear  8
            normal: [0.0, 0.0, 1.0],
        },
        GrayboxVertex {
            pos: [-1.0, 1.0, 1.0], // left lower rear   9
            normal: [0.0, 0.0, 1.0],
        },
        GrayboxVertex {
            pos: [1.0, 1.0, 1.0], // right lower rear  10
            normal: [0.0, 0.0, 1.0],
        },
        GrayboxVertex {
            pos: [1.0, -1.0, 1.0], // right upper rear  11
            normal: [0.0, 0.0, 1.0],
        },
        // Bottom
        GrayboxVertex {
            pos: [-1.0, 1.0, -1.0], // left lower front  12
            normal: [0.0, 1.0, 0.0],
        },
        GrayboxVertex {
            pos: [-1.0, 1.0, 1.0], // left lower rear   13
            normal: [0.0, 1.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, 1.0, 1.0], // right lower rear  14
            normal: [0.0, 1.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, 1.0, -1.0], // right lower front 15
            normal: [0.0, 1.0, 0.0],
        },
        // Left
        GrayboxVertex {
            pos: [-1.0, -1.0, -1.0], // left upper front  16
            normal: [-1.0, 0.0, 0.0],
        },
        GrayboxVertex {
            pos: [-1.0, -1.0, 1.0], // left upper rear   17
            normal: [-1.0, 0.0, 0.0],
        },
        GrayboxVertex {
            pos: [-1.0, 1.0, 1.0], // left lower rear   18
            normal: [-1.0, 0.0, 0.0],
        },
        GrayboxVertex {
            pos: [-1.0, 1.0, -1.0], // left lower front 19
            normal: [-1.0, 0.0, 0.0],
        },
        // Right
        GrayboxVertex {
            pos: [1.0, -1.0, -1.0], // right upper front 20
            normal: [1.0, 0.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, -1.0, 1.0], // right upper rear  21
            normal: [1.0, 0.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, 1.0, 1.0], // right lower rear  22
            normal: [1.0, 0.0, 0.0],
        },
        GrayboxVertex {
            pos: [1.0, 1.0, -1.0], // right lower front 23
            normal: [1.0, 0.0, 0.0],
        },
    ];

    let indices = vec![
        (0, 1, 2),
        (0, 2, 3),
        (5, 4, 7),
        (5, 7, 6),
        (12, 13, 14),
        (12, 14, 15),
        (9, 8, 11),
        (9, 11, 10),
        (17, 18, 19),
        (17, 19, 16),
        (20, 23, 22),
        (20, 22, 21u16),
    ];

    let bounding_cuboid = [
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, -1.0),
        Point3::new(1.0, -1.0, 1.0),
        Point3::new(1.0, -1.0, -1.0),
        Point3::new(-1.0, 1.0, 1.0),
        Point3::new(-1.0, 1.0, -1.0),
        Point3::new(-1.0, -1.0, 1.0),
        Point3::new(-1.0, -1.0, -1.0),
    ];

    let mut mesh = Mesh::new();
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.bounding_cuboid = Some(bounding_cuboid);
    mesh.strings = vec![("pipeline".to_owned(), "graybox".to_owned())];

    mesh
}
