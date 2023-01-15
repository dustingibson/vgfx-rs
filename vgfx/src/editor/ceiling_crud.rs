use std::collections::HashMap;
use gl;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;
use crate::model::model::ModelInstance;
use crate::model::ceiling::Ceiling;
use crate::World;
use uuid::Uuid;

pub struct CeilingCrud {
    pub main_label: Label2D,
    pub texture_cursor: Option<ModelInstance>,
    pub model_index: i32,
    pub prev_model_id: String,
    pub height: f32,
    pub ceiling: Ceiling
}

impl CeilingCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 128);
        let mut texture_crud = CeilingCrud {
            main_label: label,
            texture_cursor: None,
            model_index: 0,
            prev_model_id: "no_selected_ceiling".to_string(),
            height: 25.0,
            ceiling: Ceiling::new()
        };
        return texture_crud;
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

    pub fn contains_ceiling_texture(&mut self, model_map: &HashMap<String, Model>) -> bool {
        let mut has_ceiling = false;
        model_map.keys().for_each( |k| {
            if k.contains("ceiling") {
                has_ceiling = true;
            }
        });
        return has_ceiling;
    }

    pub fn next_or_prev_texture_name(&mut self, model_map: &HashMap<String, Model>, direction: i32) -> String {
        while self.contains_ceiling_texture(model_map) {
            if direction > 0 {
                if (self.model_index + 1 >= model_map.len() as i32) { self.model_index = 0; } else { self.model_index += direction; }
            } else {
                if (self.model_index < 0) { self.model_index = model_map.len() as i32 - 1; } else { self.model_index += direction; }
            }
            let index_name = self.model_map_to_index(model_map, self.model_index as u32);
            if index_name.contains("ceiling") {
                return index_name;
            }
        }
        return "".to_string();
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, world: &mut World) {
        // TODO: Change size, we may need this when I work on collision
        if (sdl_context.check_pressed("]".to_string())) {
            let model_name = self.next_or_prev_texture_name(&world.model_map, 1);
            self.ceiling.remove_textures(world);
            self.ceiling.insert_texture(glm::vec3(0.0, 1.0*self.height, 0.0), model_name, 1.0, self.height, camera, &mut world.oct_tree);
        }
        if (sdl_context.check_pressed("[".to_string())) {
            let model_name = self.next_or_prev_texture_name(&world.model_map, -1);
            self.ceiling.remove_textures(world);
            self.ceiling.insert_texture(glm::vec3(0.0, 1.0*self.height, 0.0), model_name, 1.0, self.height, camera, &mut world.oct_tree);
        }
    }


}