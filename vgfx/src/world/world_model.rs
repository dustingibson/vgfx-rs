pub struct WorldModelInstance {
    pub model_name: String,
    pub position: Vec<f32>
}

pub struct WorldAreaInstance {
    pub model_instances: Vec<WorldModelInstance>
}

pub struct WorldTextureInfo {
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

pub struct WorldModel {
    pub name: String,
    pub faces: Vec<WorldFacePartition>,
    pub vertices: Vec<Vec<f32>>,
    pub texture_mappings: Vec<Vec<f32>>,
    pub normals: Vec<Vec<f32>>,
    pub texture_info: Vec<WorldTextureInfo>
}

pub struct WorldFacePartition {
    pub texture_info_index: usize,
    pub faces: Vec<Vec<WorldFace>>
}

pub struct WorldFace {
    pub vertex_index: usize,
    pub texture_map_index: usize,
    pub normals_index: usize
}