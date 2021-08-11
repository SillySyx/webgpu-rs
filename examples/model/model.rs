use std::error::Error;

#[derive(Clone)]
struct FaceIndices {
    id: String,
    position: usize,
    normal: usize,
    uv: usize,
}

impl FaceIndices {
    fn parse(face_str: &str) -> Self {
        let values: Vec<usize> = face_str
            .split("/")
            .map(|value| value.parse::<usize>().unwrap())
            .collect();

        Self {
            id: face_str.to_owned(),
            position: values[0] - 1,
            uv: values[1] - 1,
            normal: values[2] - 1,
        }
    }

    fn to_vertex_raw(&self, positions: &Vec<cgmath::Vector3<f32>>, normals: &Vec<cgmath::Vector3<f32>>, uvs: &Vec<cgmath::Vector2<f32>>) -> VertexRaw {
        let position = positions[self.position];
        let normal = normals[self.normal];
        let uv = uvs[self.uv];

        VertexRaw {
            position: [position.x, position.y, position.z],
            normals: [normal.x, normal.y, normal.z],
            uvs: [uv.x, uv.y],
        }
    }
}

enum Face {
    Triangle(FaceIndices, FaceIndices, FaceIndices),
    Quad(FaceIndices, FaceIndices, FaceIndices, FaceIndices),
}

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub instances: Vec<Instance>,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub offset: u32,
    pub len: u32,
}

#[derive(Debug)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Vector3<f32>,
    pub scale: cgmath::Vector3<f32>,
    pub material: Material,
}

impl Instance {
    pub fn to_instance_raw(&self) -> InstanceRaw {
        let position_matrix = cgmath::Matrix4::from_translation(self.position);
        let rotation_matrix = cgmath::Matrix4::from_angle_x(cgmath::Deg(self.rotation.x)) * cgmath::Matrix4::from_angle_y(cgmath::Deg(self.rotation.y)) * cgmath::Matrix4::from_angle_z(cgmath::Deg(self.rotation.z));
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        let model_matrix = position_matrix * rotation_matrix * scale_matrix;

        InstanceRaw {
            model_matrix: model_matrix.into(),
            ambient_color: self.material.ambient.into(),
            diffuse_color: self.material.diffuse.into(),
            specular_color: self.material.specular.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Material {
    pub ambient: cgmath::Vector3<f32>,
    pub diffuse: cgmath::Vector3<f32>,
    pub specular: cgmath::Vector3<f32>,
}

pub trait VertexBufferLayout {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub position: [f32; 3],
    pub uvs: [f32; 2],
    pub normals: [f32; 3],
}

impl VertexBufferLayout for VertexRaw {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexRaw>() as wgpu::BufferAddress, 
            step_mode: wgpu::InputStepMode::Vertex, 
            attributes: &[ 
                wgpu::VertexAttribute {
                    offset: 0, 
                    shader_location: 0, 
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model_matrix: [[f32; 4]; 4],
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
}

impl VertexBufferLayout for InstanceRaw {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress, 
            step_mode: wgpu::InputStepMode::Instance, 
            attributes: &[ 
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub fn parse_wavefront_object(data: String) -> Result<(Model, Vec<VertexRaw>, Vec<u32>), Box<dyn Error>> {
    let mut positions = vec![];
    let mut uvs = vec![];
    let mut normals = vec![];

    let mut verticies: Vec<(String, VertexRaw)> = vec![];
    let mut indices = vec![];

    let instances = vec![];
    let mut meshes = vec![];

    for text in data.split("\n") {
        if text.starts_with("o ") {
            let indices_len = indices.len() as u32;
            if indices_len > 0 {
                let offset = calc_mesh_offset(&meshes);
                meshes.push(Mesh {
                    offset,
                    len: indices_len - offset,
                });
            }
        }

        if text.starts_with("v ") {
            positions.push(parse_into_vec3f32(&text[2..])?);
        }

        if text.starts_with("vt ") {
            uvs.push(parse_into_vec2f32(&text[3..])?);
        }

        if text.starts_with("vn ") {
            normals.push(parse_into_vec3f32(&text[3..])?);
        }

        if text.starts_with("f ") {
            let face = parse_face(&text[2..]);
            if let Face::Triangle(a, b, c) = &face {
                add_verticies_and_indices(a, &mut verticies, &mut indices, &positions, &normals, &uvs);
                add_verticies_and_indices(b, &mut verticies, &mut indices, &positions, &normals, &uvs);
                add_verticies_and_indices(c, &mut verticies, &mut indices, &positions, &normals, &uvs);
            }
            if let Face::Quad(a, b, c, d) = &face {
                add_verticies_and_indices(a, &mut verticies, &mut indices, &positions, &normals, &uvs);
                add_verticies_and_indices(b, &mut verticies, &mut indices, &positions, &normals, &uvs);
                add_verticies_and_indices(c, &mut verticies, &mut indices, &positions, &normals, &uvs);

                add_verticies_and_indices(a, &mut verticies, &mut indices, &positions, &normals, &uvs);
                add_verticies_and_indices(c, &mut verticies, &mut indices, &positions, &normals, &uvs);
                add_verticies_and_indices(d, &mut verticies, &mut indices, &positions, &normals, &uvs);
            }
        }
    }

    let offset = calc_mesh_offset(&meshes);
    meshes.push(Mesh {
        offset,
        len: indices.len() as u32 - offset,
    });

    Ok((
        Model {
            instances,
            meshes,
        }, 
        verticies.iter().map(|(_, vertex)| *vertex).collect(), 
        indices
    ))
}

pub fn parse_wavefront_material(data: String) -> Result<Material, Box<dyn Error>> {
    let mut ambient = cgmath::vec3(0.0f32, 0.0, 0.0);
    let mut diffuse = cgmath::vec3(1.0f32, 0.0, 0.0);
    let mut specular = cgmath::vec3(1.0f32, 1.0, 1.0);
    // let mut emissive = cgmath::vec3(0.0f32, 0.0, 0.0);

    for text in data.split("\n") {
        if text.starts_with("Ka ") {
            ambient = parse_into_vec3f32(&text[3..])?;
        }

        if text.starts_with("Kd ") {
            diffuse = parse_into_vec3f32(&text[3..])?;
        }

        if text.starts_with("Ks ") {
            specular = parse_into_vec3f32(&text[3..])?;
        }

        // if text.starts_with("Ke ") {
        //     emissive = parse_into_vec3f32(&text[3..])?;
        // }
    }

    Ok(Material {
        ambient,
        diffuse,
        specular,
    })
}

fn parse_into_vec3f32(text: &str) -> Result<cgmath::Vector3<f32>, Box<dyn Error>> {
    let values: Vec<f32> = text
        .split(" ")
        .map(|value| value.parse::<f32>().unwrap())
        .collect();

    Ok(cgmath::vec3(values[0], values[1], values[2]))
}

fn parse_into_vec2f32(text: &str) -> Result<cgmath::Vector2<f32>, Box<dyn Error>> {
    let values: Vec<f32> = text
        .split(" ")
        .map(|value| value.parse::<f32>().unwrap())
        .collect();

    Ok(cgmath::vec2(values[0], values[1]))
}

fn parse_face(faces_str: &str) -> Face {
    let faces: Vec<FaceIndices> = faces_str
        .split(" ")
        .map(|face| FaceIndices::parse(face))
        .collect();

    if faces.len() == 3 {
        return Face::Triangle(faces[0].clone(), faces[1].clone(), faces[2].clone());
    }

    Face::Quad(faces[0].clone(), faces[1].clone(), faces[2].clone(), faces[3].clone())
}

fn find_index_of_face(verticies: &Vec<(String, VertexRaw)>, face_id: &str) -> Option<usize> {
    verticies
        .iter()
        .enumerate()
        .find(|(_, (id ,_))| id == face_id)
        .map(|(index, _)| index)
}

fn add_verticies_and_indices(face: &FaceIndices, verticies: &mut Vec<(String, VertexRaw)>, indices: &mut Vec<u32>, positions: &Vec<cgmath::Vector3<f32>>, normals: &Vec<cgmath::Vector3<f32>>, uvs: &Vec<cgmath::Vector2<f32>>) {
    match find_index_of_face(&verticies, &face.id) {
        Some(index) => {
            indices.push(index as u32);
        },
        None => {
            let vertex = face.to_vertex_raw(&positions, &normals, &uvs);
            verticies.push((face.id.clone(), vertex));

            let index = find_previous_index(&indices);
            if indices.len() > 0 {
                indices.push(index + 1);
            }
            else {
                indices.push(index);
            }
        },
    };
}

fn find_previous_index(indices: &Vec<u32>) -> u32 {
    let largest_index =  indices.iter().fold(0u32, |largest, index| largest.max(*index));

    largest_index
}

fn calc_mesh_offset(meshes: &Vec<Mesh>) -> u32 {
    meshes.iter().fold(0u32, |offset, mesh| offset + mesh.offset + mesh.len)
}