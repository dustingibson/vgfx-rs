use gl;
use crate::BFile;
extern crate nalgebra_glm as glm;

use crate::TexturePolygon;
use crate::SubModel;
use crate::Shader;
use crate::gfx::texture::Texture;
use crate::world::world_model::WorldFacePartition;
#[derive(Clone)]

pub struct ModelInstance {
    pub model_name: String,
    pub position: Vec<f32>
}

pub struct AreaInstance {
    pub model_instances: Vec<ModelInstance>
}

pub struct Model {
    pub name: String,
    pub face_partitions: Vec<WorldFacePartition>,
    pub textures: Vec<Texture>
}

impl Model {
    pub fn new(name: String) -> Self {
        return Model {
            name: name.to_string(),
            face_partitions: vec![],
            textures: vec![]
        };
    }
}