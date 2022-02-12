use gl;
use crate::BFile;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::TexturePolygon;
use crate::SubModel;
use crate::Shader;
#[derive(Clone)]

pub struct ModelInstance {
    pub model_name: String,
    pub position: Vec<f32>
}

pub struct AreaInstance {
    pub texture_polygons: Vec<TexturePolygon>,
    pub model_instances: Vec<ModelInstance>
}

pub struct SubModelComponent {
    pub name: String,
    pub texture_polygons: Vec<TexturePolygon>
}

pub struct ModelComponent {
    pub name: String,
    pub sub_models: Vec<SubModelComponent>
}

pub struct Model {
    pub name: String,
    pub sub_models: Vec<SubModel>,
    pub size: glm::Vec3
}

impl Model {
    pub fn new(name: String, size: glm::Vec3) -> Self {
        return Model {
            name: name.to_string(),
            sub_models: vec![],
            size: size
        };
    }

    pub fn from_single_cuboid(&mut self, name: String, cuboid: &mut Cuboid) {
        let mut cuboid_vec: Vec<Cuboid> = Vec::new();
        cuboid_vec.push(cuboid.clone());
        self.sub_models.push(SubModel::new(name.to_string(), cuboid.position, &mut cuboid_vec, &mut Vec::new()));
    }

    pub fn pos_from_cuboid(&mut self) -> glm::Vec3
    {
        return self.sub_models[0].cuboids[0].position;
    }

    pub fn insert_submodel(&mut self, name: String, position: glm::Vec3, size: glm::Vec3, cuboids: &mut Vec<Cuboid>, color_polygons: &mut Vec<TexturePolygon>) {
        self.sub_models.push(SubModel::new(name.to_string(), position, cuboids, color_polygons));
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        for sub_model in self.sub_models.iter_mut() {
            sub_model.draw(shader);
        }
    }
}