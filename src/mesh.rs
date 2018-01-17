
use siege_math::Vec4;
use vertex::{Vertex, VertexType};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh<V: Vertex> {
    // The type of vertex.
    vertex_type: VertexType,

    /// The vertices of the mesh
    pub vertices: Vec<V>,

    /// Indices of vertices for forming triangles
    /// Meshes currently use indexed draw only
    pub indices: Vec<(u32,u32,u32)>,

    /// String values
    /// Can use to store names of textures, e.g.: ("bump_texture", "mybump.obj")
    /// Well-known keys include:
    ///    albedo, normal, tangent, diffuse, specular, roughness, gloss,
    ///    bump, displacement, ao, cavity
    pub strings: Vec<(String, String)>,

    /// Float values
    /// Can use to store floats, e.g.: ("roughness", 12.0)
    /// Well-known keys include:
    ///    roughness, gloss
    pub floats: Vec<(String, f32)>,

    /// Float-quads
    /// Can use to store colors, e.g.: ("diffuse", (0.5, 0.5, 0.5, 1.0))
    /// Well-known keys include:
    ///    diffuse, specular
    pub vec4s: Vec<(String, Vec4<f32>)>,
}

impl<V: Vertex> Default for Mesh<V> {
    fn default() -> Mesh<V> {
        Mesh {
            vertex_type: VertexType::Standard,
            vertices: Vec::new(),
            indices: Vec::new(),
            strings: Vec::new(),
            floats: Vec::new(),
            vec4s: Vec::new(),
        }
    }
}

impl<V: Vertex> Mesh<V> {
    pub fn new(vertex_type: VertexType) -> Mesh<V> {
        Mesh {
            vertex_type: vertex_type,
            ..Default::default()
        }
    }
}
