
// Serialization:
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate siege_math;

pub mod vertex;
pub use self::vertex::*;

use siege_math::Vec4;

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh<V: Vertex> {
    /// The vertices of the mesh
    pub vertices: Vec<V>,

    /// Indices of vertices for forming triangles
    /// Meshes currently use indexed draw only
    pub indices: Vec<(u32,u32,u32)>,

    /// String values
    /// Can use to store names of textures, e.g.: ("bump_texture", "mybump.obj")
    /// Well-known keys include:
    ///    diffuse_texture, specular_texture, roughness_texture, gloss_texture,
    ///    normal_texture, bump_texture, displacement_texture, ao_texture,
    ///    cavity_texture
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
            vertices: Vec::new(),
            indices: Vec::new(),
            strings: Vec::new(),
            floats: Vec::new(),
            vec4s: Vec::new(),
        }
    }
}
