use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ModelInstance {
    pub model_name: String,
    pub position: Vec<f32>
}

#[derive(Serialize, Deserialize)]
pub struct AreaInstance {
    pub model_instances: Vec<ModelInstance>
}

pub struct TextureInfo {
    pub name: String,
    pub ambient_color: Vec<f32>,
    pub diffuse_color: Vec<f32>,
    pub specular_color: Vec<f32>,
    pub emissive_coeficient: Vec<f32>,
    pub transmission_filter: Vec<f32>,
    pub optical_density: f32,
    pub dissolve: f32,
    pub specular_highlights: f32,
    pub illum: i32,
    pub img: Vec<u8>,
}

pub struct Model {
    pub name: String,
    pub faces: Vec<Face>,
    pub vertices: Vec<f32>,
    pub texture_mappings: Vec<f32>,
    pub normals: Vec<f32>,
    pub texture_info: Vec<TextureInfo>
}

pub struct Face {
    pub texture_info_index: usize,
    pub vertex_index: usize,
    pub texture_map_index: usize,
    pub normals_index: usize
}