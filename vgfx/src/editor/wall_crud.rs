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

pub struct WallCrud {
    pub texture_cursors: Vec<Option<ModelInstance>>,
    pub model_index: i32,
    pub prev_model_ids: Vec<String>
}

impl WallCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let mut texture_crud = WallCrud {
            texture_cursors: vec![None, None, None, None],
            model_index: 0,
            prev_model_ids: vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]
        };
        return texture_crud;
    }

    fn new_model_instance(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, index: u32) -> ModelInstance {
        return ModelInstance {
            model_name: self.model_map_to_index(model_map, index), 
            position: glm::vec3(0.0, -50.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
            rotate: glm::Vec3::new(0.0, 0.0, 0.0),
            name: Uuid::new_v4().to_string()
        }
    }

    fn model_map_to_index(&mut self, model_map: &HashMap<String, Model>, index: u32) -> String {
        let mut cur_index: u32 = 0;
        for (key, model) in model_map.into_iter() {
            if index == cur_index {
                return model.name.to_string();
            }
            cur_index = cur_index + 1;
        }
        return "".to_string();
    }

    pub fn set_new_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, direction: u32) -> Option<ModelInstance> {
        let mut model_instance = self.new_model_instance(camera, model_map, self.model_index as u32);
        //Top
        if (direction == 0) {
            model_instance.position = glm::vec3(1000.0, -50.0, 0.0);
        }
        // Right
        if (direction == 1) {
            model_instance.position = glm::vec3(0.0, -50.0, 1000.0);
            model_instance.rotate = glm::vec3(0.0, 1.57, 0.0);
        }
        // Bottom
        else if (direction == 2) {
            model_instance.position = glm::vec3(-1000.0, -50.0, 0.0);
            model_instance.rotate = glm::vec3(3.14, 0.0, 0.0);
        }
        // Left
        else if (direction == 3) {
            model_instance.position = glm::vec3(0.0, -50.0, -1000.0);
            model_instance.rotate = glm::vec3(0.0, 1.57, 0.0);
        }
        model_instance.scale = glm::Vec3::new(10.0, 10.0, 10.0);
        return Some(model_instance);
    }

    pub fn contains_wall_texture(&mut self, model_map: &HashMap<String, Model>) -> bool {
        let mut has_wall = false;
        model_map.keys().for_each( |k| {
            if k.contains("wall") {
                has_wall = true;
            }
        });
        return has_wall;
    }

    pub fn next_or_prev_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, direction: i32) {
        while self.contains_wall_texture(model_map) {
            if direction > 0 {
                if (self.model_index + 1 >= model_map.len() as i32) { self.model_index = 0; } else { self.model_index += direction; }
            } else {
                if (self.model_index < 0) { self.model_index = model_map.len() as i32 - 1; } else { self.model_index += direction; }
            }
            let index_name = self.model_map_to_index(model_map, self.model_index as u32);
            if index_name.contains("wall") {
                for i in 0..4 {
                    if !self.texture_cursors[i].as_ref().is_none() {
                        self.prev_model_ids[i] = self.texture_cursors[i].as_ref().unwrap().name.to_string();
                    }
                    self.texture_cursors[i] = self.set_new_texture(camera, model_map, i as u32);
                }
                return;
            }
        }
        return;
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, world: &mut World) {
        if (sdl_context.check_pressed("O".to_string())) {
            self.next_or_prev_texture(camera, &world.model_map, 1);
            for i in 0..4 {
                world.oct_tree.remove_item_by_name(self.prev_model_ids.get(i).unwrap().to_string());
                world.oct_tree.insert_item(Box::new(self.texture_cursors.get(i).unwrap().clone().unwrap()), 0.0, 0.0, 0.0);
            }
        }
        if (sdl_context.check_pressed("P".to_string())) {
            self.next_or_prev_texture(camera, &world.model_map, -1);
            for i in 0..4 {
                world.oct_tree.remove_item_by_name(self.prev_model_ids.get(i).unwrap().to_string());
                world.oct_tree.insert_item(Box::new(self.texture_cursors.get(i).unwrap().clone().unwrap()), 0.0, 0.0, 0.0);
            }
        }
    }


}