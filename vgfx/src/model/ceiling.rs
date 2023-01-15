use gl;
extern crate nalgebra_glm as glm;
use crate::Camera;
use crate::model::model::ModelInstance;
use crate::World;
use uuid::Uuid;
use crate::utils::octo::OctTree;


pub struct Ceiling {
    pub size: f32,
    pub height: f32,
    pub model_name: String,
    pub model_id: String
}

impl Ceiling {
    pub fn new() -> Self {
        return Ceiling {
            size: 0.0,
            height: 0.0,
            model_name: "".to_string(),
            model_id: "".to_string()
        }
    }

    fn new_model_instance(&mut self) -> ModelInstance {
        return ModelInstance {
            model_name: self.model_name.to_string(), 
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
            rotate: glm::Vec3::new(0.0, 0.0, 0.0),
            name: Uuid::new_v4().to_string()
        }
    }

    pub fn remove_textures(&mut self, world: &mut World) {
        world.oct_tree.remove_item_by_name(self.model_id.to_string());
    }

    pub fn insert_texture(&mut self, position: glm::Vec3, model_name: String, size: f32, height: f32, camera: &mut Camera, oct_tree: &mut OctTree<ModelInstance>) {
        self.model_name = model_name;
        self.size = size;
        self.height = height;
        let mut model_instance = self.new_model_instance();
        //model_instance.position = glm::vec3(0.0, 1.0*self.height, 0.0);
        model_instance.position = position;
        self.model_id = model_instance.name.to_string();
        oct_tree.insert_item_vec3(Box::new(model_instance.clone()), model_instance.position);
    }

}