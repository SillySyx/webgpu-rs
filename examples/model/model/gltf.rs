pub struct Content {
    pub active_scene: Option<usize>,
    pub scenes: Vec<Scene>,
    pub nodes: Vec<Node>,
    pub cameras: Vec<Camera>,
    pub skins: Vec<Skin>,
    pub animations: Vec<Animation>,
    pub meshes: Vec<Mesh>,
    pub accessors: Vec<Accessor>,
    pub buffer_views: Vec<BufferView>,
    pub buffers: Vec<Buffer>,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>,
    pub images: Vec<Image>,
    pub samplers: Vec<Sampler>,
}

pub struct Scene {
    pub name: String,
    pub nodes: Vec<usize>,
}

pub struct Node {
    pub name: String,
    pub children: Vec<usize>,
    pub mesh: Option<usize>,
    pub skin: Option<usize>,
    pub camera: Option<usize>,

    pub translation: Option<cgmath::Vector3<f32>>,
    pub rotation: Option<cgmath::Quaternion<f32>>,
    pub scale: Option<cgmath::Vector3<f32>>,
}

pub enum CameraType {
    Perspective { aspect_ratio: f32, vertical_fov: f32, z_near: f32, z_far: Option<f32> },
    Orthographic { xmag: f32, ymag: f32, z_near: f32, z_far: Option<f32> },
}

pub struct Camera {
    pub name: String,
    pub r#type: CameraType,
}

pub struct Skin {
    pub inverse_bind_matrices: usize,
    pub joints: Vec<usize>,
}

pub struct Animation {
    pub name: String,
}

pub struct Mesh {
    pub name: String,
    pub primitives: Vec<MeshPrimitive>,
    pub weights: Option<Vec<usize>>,
}

pub struct MeshPrimitive {
    pub mode: MeshPrimitiveModes,
    pub attributes: MeshPrimitiveAttribute,
    pub targets: Vec<MeshPrimitiveAttribute>,
    pub indices: usize,
    pub material: usize,
}

pub enum MeshPrimitiveModes {
    Points,
    Lines,
    Triangles,
    Quads,
}

pub struct MeshPrimitiveAttribute {
    pub position: Option<usize>,
    pub normal: Option<usize>,
    pub tangent: Option<usize>,
    pub tex_coord_0: Option<usize>,
}

pub struct Accessor {
    pub r#type: AccessorTypes,
    pub component_type: usize,
    pub count: usize,
}

pub enum AccessorTypes {

}

pub struct BufferView {
}

pub struct Buffer {
}

pub struct Material {
    pub name: String,
}

pub struct Texture {
    pub source: usize,
    pub sampler: usize,
}

pub struct Image {
    pub name: String,
    pub uri: Option<String>,
    pub buffer_view: Option<usize>,
    pub mime_type: Option<String>,
}

pub struct Sampler {
    pub mag_filter: usize,
    pub min_filter: usize,
    pub wrap_s: Option<usize>,
    pub wrap_t: Option<usize>,
}