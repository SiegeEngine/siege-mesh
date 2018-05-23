extern crate bincode;
extern crate siege_mesh;

use siege_mesh::{CubemapVertex, Mesh, Point3, VertexType};

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
        CubemapVertex {
            pos: [1.0, -1.0, -1.0],
        }, // right upper front
        CubemapVertex {
            pos: [1.0, -1.0, 1.0],
        }, // right upper rear
        CubemapVertex {
            pos: [1.0, 1.0, -1.0],
        }, // right lower front
        CubemapVertex {
            pos: [1.0, 1.0, 1.0],
        }, // right lower rear
        // Left (-X)
        CubemapVertex {
            pos: [-1.0, -1.0, 1.0],
        }, // left upper rear
        CubemapVertex {
            pos: [-1.0, -1.0, -1.0],
        }, // left upper front
        CubemapVertex {
            pos: [-1.0, 1.0, 1.0],
        }, // left lower rear
        CubemapVertex {
            pos: [-1.0, 1.0, -1.0],
        }, // left lower front
        // Bottom (+Y)
        CubemapVertex {
            pos: [-1.0, 1.0, -1.0],
        }, // left lower front
        CubemapVertex {
            pos: [1.0, 1.0, -1.0],
        }, // right lower front
        CubemapVertex {
            pos: [-1.0, 1.0, 1.0],
        }, // left lower rear
        CubemapVertex {
            pos: [1.0, 1.0, 1.0],
        }, // right lower rear
        // Top (-Y)
        CubemapVertex {
            pos: [-1.0, -1.0, 1.0],
        }, // left upper rear
        CubemapVertex {
            pos: [1.0, -1.0, 1.0],
        }, // right upper rear
        CubemapVertex {
            pos: [-1.0, -1.0, -1.0],
        }, // left upper front
        CubemapVertex {
            pos: [1.0, -1.0, -1.0],
        }, // right upper front
        // Back (+Z)
        CubemapVertex {
            pos: [1.0, -1.0, 1.0],
        }, // right upper rear
        CubemapVertex {
            pos: [-1.0, -1.0, 1.0],
        }, // left upper rear
        CubemapVertex {
            pos: [1.0, 1.0, 1.0],
        }, // right lower rear
        CubemapVertex {
            pos: [-1.0, 1.0, 1.0],
        }, // left lower rear
        // Front (-Z)
        CubemapVertex {
            pos: [-1.0, -1.0, -1.0],
        }, // left upper front
        CubemapVertex {
            pos: [1.0, -1.0, -1.0],
        }, // right upper front
        CubemapVertex {
            pos: [-1.0, 1.0, -1.0],
        }, // left lower front
        CubemapVertex {
            pos: [1.0, 1.0, -1.0],
        }, // right lower front
    ];

    let indices = vec![
        (2, 1, 0),
        (1, 2, 3),
        (6, 5, 4),
        (5, 6, 7),
        (10, 9, 8),
        (9, 10, 11),
        (14, 13, 12),
        (13, 14, 15),
        (18, 17, 16),
        (17, 18, 19),
        (22, 21, 20),
        (21, 22, 23_u16),
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
    mesh.strings = vec![];

    mesh
}
