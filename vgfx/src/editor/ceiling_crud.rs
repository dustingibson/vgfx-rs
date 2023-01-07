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

pub struct CeilingCrud {
    pub main_label: Label2D,
    pub texture_cursor: Option<ModelInstance>,
    pub model_index: i32,
    pub prev_model_id: String,
    pub height: f32
}

impl CeilingCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 128);
        let mut texture_crud = CeilingCrud {
            main_label: label,
            texture_cursor: None,
            model_index: 0,
            prev_model_id: "no_selected_ceiling".to_string(),
            height: 16.0
        };
        return texture_crud;
    }

    fn new_model_instance(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, index: u32) -> ModelInstance {
        return ModelInstance {
            model_name: self.model_map_to_index(model_map, index), 
            position: glm::vec3(0.0, self.height, 0.0),
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

    pub fn set_new_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Option<ModelInstance> {
        let mut model_instance = self.new_model_instance(camera, model_map, self.model_index as u32);
        return Some(model_instance);
    }

    pub fn contains_ceiling_texture(&mut self, model_map: &HashMap<String, Model>) -> bool {
        let mut has_ceiling = false;
        model_map.keys().for_each( |k| {
            if k.contains("ceiling") {
                has_ceiling = true;
            }
        });
        return has_ceiling;
    }

    pub fn next_or_prev_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, direction: i32) {
        while self.contains_ceiling_texture(model_map) {
            if direction > 0 {
                if (self.model_index + 1 >= model_map.len() as i32) { self.model_index = 0; } else { self.model_index += direction; }
            } else {
                if (self.model_index < 0) { self.model_index = model_map.len() as i32 - 1; } else { self.model_index += direction; }
            }
            let index_name = self.model_map_to_index(model_map, self.model_index as u32);
            if index_name.contains("ceiling") {
                if !self.texture_cursor.as_ref().is_none() {
                    self.prev_model_id = self.texture_cursor.as_ref().unwrap().name.to_string();
                }
                self.texture_cursor = self.set_new_texture(camera, model_map);
                return;
            }
        }
        return;
    }

    pub fn prev_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>) {
        if ( (self.model_index - 1) < 0) { self.model_index = model_map.len() as i32 - 1; } else { self.model_index -= 1; }
        self.set_new_texture(camera, model_map);
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, world: &mut World) {
        if (sdl_context.check_pressed(";".to_string())) {
            self.next_or_prev_texture(camera, &world.model_map, 1);
            world.oct_tree.remove_item_by_name(self.prev_model_id.to_string());
            world.oct_tree.insert_item(Box::new(self.texture_cursor.clone().unwrap()), 0.0, 0.0, 0.0);
        }
        if (sdl_context.check_pressed("'".to_string())) {
            self.next_or_prev_texture(camera, &world.model_map, -1);
            world.oct_tree.remove_item_by_name(self.prev_model_id.to_string());
            world.oct_tree.insert_item(Box::new(self.texture_cursor.clone().unwrap()), 0.0, 0.0, 0.0);
        }
    }


}