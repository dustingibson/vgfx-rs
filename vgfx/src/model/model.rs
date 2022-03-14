use gl;
use crate::BFile;
extern crate nalgebra_glm as glm;

use crate::TexturePolygon;
use crate::SubModel;
use crate::Shader;
use crate::dep::events::SDLContext;
use crate::gfx::face::FacePartitionRender;
use crate::gfx::texture::Texture;
#[derive(Clone)]

pub struct ModelInstance {
    pub model_name: String,
    pub position: glm::Vec3,
    pub face_partitions: Vec<FacePartitionRender>
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

    pub fn draw(&mut self, shader: &mut Shader, position: &mut glm::Vec3) {
        for face_partition in self.face_partitions.iter_mut() {
            face_partition.draw(shader, position);
        }
    }
}