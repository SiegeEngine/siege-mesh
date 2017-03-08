
// Serialization:
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

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
