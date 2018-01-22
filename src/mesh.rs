
use errors::*;
use vertex::*;
use std::path::Path;
use siege_math::Vec4;

pub const MAGIC1: u8 = 238;
pub const MAGIC2: u8 = 91;
pub const MAGIC3: u8 = 130;

// In order to deserialize a Mesh, we first need to know its vertex type.
// So we have a header, which helps us deserialize the rest.
// Since we have a header, we might as well have a magic number too.
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshHeader {
    magic1: u8,
    magic2: u8,
    magic3: u8,
    pub vertex_type: VertexType,
}
impl MeshHeader {
    pub fn new(vertex_type: VertexType) -> MeshHeader {
        MeshHeader {
            magic1: MAGIC1,
            magic2: MAGIC2,
            magic3: MAGIC3,
            vertex_type: vertex_type,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh<V: Vertex> {
    /// The vertices of the mesh
    pub vertices: Vec<V>,

    /// Indices of vertices for forming triangles
    /// Meshes currently use indexed draw only
    pub indices: Vec<(u16,u16,u16)>,

    /// String values
    /// Can use to store names of textures, e.g.: ("bump_texture", "mybump.obj")
    /// Well-known keys include:
    ///    'pipeline': which siege-client pipeline is this mesh compatibile with
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
            vertices: Vec::new(),
            indices: Vec::new(),
            strings: Vec::new(),
            floats: Vec::new(),
            vec4s: Vec::new(),
        }
    }
}

impl<V: Vertex> Mesh<V> {
    pub fn new() -> Mesh<V> {
        Mesh {
            ..Default::default()
        }
    }

    pub fn save(&self, vertex_type: VertexType, pathstr: &str) -> Result<()>
    {
        use std::fs::File;
        use std::io::Write;

        let mut f = try!(File::create(pathstr));
        f.write(&[MAGIC1, MAGIC2, MAGIC3, vertex_type as u8])?;
        ::bincode::serialize_into(&mut f, self, ::bincode::Infinite)?;
        Ok(())
    }
}

pub fn load_header(path: &Path) -> Result<(VertexType, Vec<u8>)>
{
    use std::fs::File;
    use std::io::Read;

    let mut f = try!(File::open(path));
    let mut bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes)?;
    if bytes.len() < 4 {
        return Err(ErrorKind::ShortFile.into());
    }

    if bytes[0]!=MAGIC1 || bytes[1]!=MAGIC2 || bytes[2]!=MAGIC3
    {
        return Err(ErrorKind::BadMagicNumber.into());
    }

    let vertex_type = VertexType::try_from_u8(bytes[3])?;

    Ok((vertex_type, bytes))
}

pub fn deserialize_colored(bytes: &[u8]) -> Result<Mesh<ColoredVertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
pub fn deserialize_standard(bytes: &[u8]) -> Result<Mesh<StandardVertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
pub fn deserialize_gui_rectangle(bytes: &[u8]) -> Result<Mesh<GuiRectangleVertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
pub fn deserialize_graybox(bytes: &[u8]) -> Result<Mesh<GrayboxVertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
pub fn deserialize_cheapv1(bytes: &[u8]) -> Result<Mesh<CheapV1Vertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
pub fn deserialize_cheapv2(bytes: &[u8]) -> Result<Mesh<CheapV2Vertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
pub fn deserialize_star(bytes: &[u8]) -> Result<Mesh<StarVertex>> {
    Ok(::bincode::deserialize(bytes)?)
}
