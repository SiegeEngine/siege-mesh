
// Notes:  tangent maps are no longer best precomputed; Better to compute them
//         in the fragment shader.

use errors::*;
use serde::Serialize;

/// The type of vertex
#[repr(u8)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
pub enum VertexType {

    // PC = 0, // retired

    /// A vertex with color and normal, for demonstration shapes (e.g. cube, tetrahedron)
    /// (formerly called PCN)
    Colored = 1,

    /// For standard mesh type, where everything is done with UV mapped textures, so it only
    /// needs position and UV (formerly called PU)
    Standard = 2,

    // PNTU = 3, // retired
    // PNTUS = 4, // retired
    // PNTUR = 5, // retired
    // PNTUSR = 6 // retired

    /// For a rectangular area on the screen for GUI usage (2d position only, uv implicit)
    GuiRectangle = 7,

    /// For grayboxed meshes with no textures involved, using physically-improved
    /// Blinn-Phong shading.  Per-vertex position and normal.  Color (diffuse
    /// and specular) and shininess is across the entire model, not per-vertex.
    Graybox = 9,

    /// Cheaper than Standard, we provide shading data per-vertex
    CheapV1 = 11,

    /// Even Cheaper than Cheap, we provide normal/specular/roughness per-vertex
    CheapV2 = 12,

    /// Star
    Star = 13,

    /// Cubemap (position and uvw)
    Cubemap = 14,
}

impl VertexType {
    pub fn try_from_u8(u: u8) -> Result<VertexType> {
        match u {
            1 => Ok(VertexType::Colored),
            2 => Ok(VertexType::Standard),
            7 => Ok(VertexType::GuiRectangle),
            9 => Ok(VertexType::Graybox),
            11 => Ok(VertexType::CheapV1),
            12 => Ok(VertexType::CheapV2),
            13 => Ok(VertexType::Star),
            14 => Ok(VertexType::Cubemap),
            _ => Err(ErrorKind::UnknownVertexType.into()),
        }
    }
}

pub trait Vertex: Copy + Serialize
{
    fn get_type() -> VertexType;
}

/// A vertex with color and normal, for demonstration shapes (e.g. cube, tetrahedron)
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ColoredVertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}
impl Vertex for ColoredVertex {
    fn get_type() -> VertexType {
        VertexType::Colored
    }
}

/// For standard mesh type, where everything is done with UV mapped textures, so it only
/// needs position and UV.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct StandardVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}
impl Vertex for StandardVertex {
    fn get_type() -> VertexType {
        VertexType::Standard
    }
}

/// For a rectangular area on the screen for GUI usage (2d position only, uv implicit)
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct GuiRectangleVertex {
    pub pos: [f32; 2],
}
impl Vertex for GuiRectangleVertex {
    fn get_type() -> VertexType {
        VertexType::GuiRectangle
    }
}

/// For grayboxed meshes with no textures involved. Normal, specular, and roughness are
/// per vertex.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct GrayboxVertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
}
impl Vertex for GrayboxVertex {
    fn get_type() -> VertexType {
        VertexType::Graybox
    }
}

/// Cheaper than Standard, we provide shininess per-vertex instead of with maps
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct CheapV1Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub shininess: f32,
}
impl Vertex for CheapV1Vertex {
    fn get_type() -> VertexType {
        VertexType::CheapV1
    }
}

/// Even Cheaper than Cheap, we provide normal/shininess per-vertex
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct CheapV2Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub shininess: f32,
}
impl Vertex for CheapV2Vertex {
    fn get_type() -> VertexType {
        VertexType::CheapV2
    }
}

/// Used for stars
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StarVertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    // From 0.0 to 1.0 (percent of max star brightness)
    pub brightness: f32,
}
impl Vertex for StarVertex {
    fn get_type() -> VertexType {
        VertexType::Star
    }
}

/// For cubemaps. Position.
#[repr(C)]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct CubemapVertex {
    pub pos: [f32; 3],
}
impl Vertex for CubemapVertex {
    fn get_type() -> VertexType {
        VertexType::Cubemap
    }
}
