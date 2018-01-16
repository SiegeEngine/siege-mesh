
// Serialization:
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate siege_math;

use siege_math::Vec4;
use serde::{Serialize,Deserialize};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh<V> {
    /// The vertices of the mesh
    pub vertices: Vec<V>,

    /// Indices of vertices for forming triangles
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

impl<V> Default for Mesh<V> {
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

pub const V_PC: u8 = 0;
pub const V_PCN: u8 = 1;
pub const V_PU: u8 = 2;
pub const V_PNTU: u8 = 3;
pub const V_PNTUS: u8 = 4;
pub const V_PNTUR: u8 = 5;
pub const V_PNTUSR: u8 = 6;

/// A very simple vertex with vertex colors, and no normals (no lighting
/// effects).  Not expected to be useful for game usage, but ok for early
/// demo usage.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPC {
    pub pos: [f32; 3],
    pub color: [f32; 4],
}

/// Vertex for cube demo.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPCN {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

/// A simple vertex useful for sprites and particle systems, where
/// shading does not apply.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPU {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}

/// This is the primary and best vertex to use for 3D objects that need
/// to be shaded.  Specular maps and Roughness maps should be used, as
/// well as A.O., cavity, and anything else; therefore, this data is not
/// per-vertex and these vertices are pretty simple.  Tangent is
/// pre-computed for performance.
///
/// This can also be used if specular and/or roughness are uniform
/// push-constants, rather than even per-vertex.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPNTU {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2],
}

/// When a specular map is not available, or too uniform to be worthwhile,
/// this vertex type can specify the specular per-vertex.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPNTUS {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2],
    pub specular: f32,
}

/// When a roughness map is not available, or too uniform to be worthwhile,
/// this vertex type can specify the roughness per-vertex.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPNTUR {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2],
    pub roughness: f32,
}

/// When specular and roughness maps are not available, or too uniform to be
/// worthwhile, this vertex type can specify them per-vertex.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct VertexPNTUSR {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2],
    pub specular: f32,
    pub roughness: f32,
}
