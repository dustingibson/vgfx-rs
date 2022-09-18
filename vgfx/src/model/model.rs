

extern crate nalgebra_glm as glm;
use std::collections::HashMap;

use crate::Shader;
use crate::gfx::face::FacePartitionRender;
use crate::gfx::texture::Texture;

#[derive(Clone)]
pub struct ModelInstance {
    pub model_name: String,
    pub position: glm::Vec3,
    pub scale: f32
}

pub struct AreaInstance {
    pub model_instances: Vec<ModelInstance>
}

// TODO: If needed optimize face partitions by model instead of instance
// Tradeoff cost of memory vs cost of processing streaming VBOs
pub struct Model {
    pub name: String,
    pub textures: Vec<Texture>,
    pub face_partitions: Vec<FacePartitionRender>
}

impl Model {
    pub fn new(name: String) -> Self {
        return Model {
            name: name.to_string(),
            textures: vec![],
            face_partitions: vec![]
        };
    }

    pub fn draw(& self, shader: &mut Shader, position: &mut glm::Vec3) {
        for face_partition in self.face_partitions.iter() {
            face_partition.draw(shader, position, &self.textures[face_partition.texture_index]);
        }
    }

    pub fn clean_up(&mut self) {
        for face_partition in self.face_partitions.iter_mut() {
            face_partition.clean_up();
        }
    }
}

impl ModelInstance {
    pub fn new(name: String, position: glm::Vec3, scale: f32) -> Self {
        return ModelInstance {
            model_name: name.to_string(),
            position: position,
            scale: scale
        };
    }

    pub fn draw(&mut self, shader: &mut Shader, model_map: &HashMap<String, Model>) {
        model_map.get(&self.model_name).unwrap().draw(shader, &mut self.position);
    }
}