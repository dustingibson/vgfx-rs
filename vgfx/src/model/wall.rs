use std::collections::HashMap;
use gl;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;
use crate::model::model::ModelInstance;
use crate::World;
use uuid::Uuid;
use crate::utils::octo::OctTree;


pub struct Wall {
    pub size: f32,
    pub model_name: String,
    pub model_ids: Vec<String>
}

impl Wall {
    pub fn new() -> Self {
        return Wall {
            size: 0.0,
            model_name: "".to_string(),
            model_ids: vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]
        };
    }

    fn new_model_instance(&mut self) -> ModelInstance {
        return ModelInstance {
            model_name: self.model_name.to_string(), 
            position: glm::vec3(0.0, -50.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
            rotate: glm::Vec3::new(0.0, 0.0, 0.0),
            name: Uuid::new_v4().to_string()
        }
    }

    pub fn remove_textures(&mut self, world: &mut World) {
        for cur_id in self.model_ids.iter() {
            world.oct_tree.remove_item_by_name(cur_id.to_string());
        }
    }

    pub fn insert_textures(&mut self, position: glm::Vec3, model_name: String, size: f32, camera: &mut Camera, oct_tree: &mut OctTree<ModelInstance>) {
        self.model_name = model_name;
        self.size = size;

        let mut model_instance_top = self.new_model_instance();
        let mut model_instance_right = self.new_model_instance();
        let mut model_instance_bot = self.new_model_instance();
        let mut model_instance_left = self.new_model_instance();

        let degrees_0 = 0.0;
        let degrees_90 = 1.570796;
        let degrees_180 = 3.141592;
        let degrees_270 = 4.71239;
        let degrees_360 = 6.28319;

        //Top
        model_instance_top.position = glm::vec3(self.size, 0.0, 0.0) + position;
        model_instance_top.rotate = glm::vec3(degrees_180, degrees_180, 0.0);

        // Right
        model_instance_right.position = glm::vec3(0.0, 0.0, self.size)  + position;
        model_instance_right.rotate = glm::vec3(degrees_360, degrees_270, degrees_180);

        // Bottom
        model_instance_bot.position = glm::vec3(self.size*-1.0, 0.0, 0.0)  + position;
        model_instance_bot.rotate = glm::vec3(degrees_180, 0.0, 0.0);

        // Left
        model_instance_left.position = glm::vec3(0.0, 0.0, self.size*-1.0)  + position;
        model_instance_left.rotate = glm::vec3(degrees_360, degrees_90, degrees_180);

        self.model_ids[0] = model_instance_top.name.to_string();
        self.model_ids[1] = model_instance_right.name.to_string();
        self.model_ids[2] = model_instance_bot.name.to_string();
        self.model_ids[3] = model_instance_left.name.to_string();

        oct_tree.insert_item_vec3(Box::new(model_instance_top.clone()), model_instance_top.position);
        oct_tree.insert_item_vec3(Box::new(model_instance_right.clone()), model_instance_right.position);
        oct_tree.insert_item_vec3(Box::new(model_instance_bot.clone()), model_instance_bot.position);
        oct_tree.insert_item_vec3(Box::new(model_instance_left.clone()), model_instance_left.position);
    }
}