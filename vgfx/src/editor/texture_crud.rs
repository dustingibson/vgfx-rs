use std::collections::HashMap;
use std::string;

use gl;
use sdl2::keyboard::Scancode;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;
use crate::model::model::ModelInstance;

pub struct TextureCrud {
    pub main_label: Label2D,
    pub texture_cursor: Option<ModelInstance>,
    pub model_index: u32
}

impl TextureCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 0.5, 0.5, 128);

        let mut texture_crud = TextureCrud {
            main_label: label,
            texture_cursor: None,
            model_index: 0
        };
        texture_crud.texture_cursor = Some(texture_crud.new_model_instance(camera, model_map, 0));
        return texture_crud;
    }

    fn new_model_instance(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, index: u32) -> ModelInstance {
        return ModelInstance {
            model_name: self.model_map_to_index(model_map, index), 
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: 1.0 
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

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Left)) {
        }
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Right)) {
        }
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
            //self.main_label.change_text(sdl_context, "NEW".to_string());
        }
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
        }
        self.draw(camera, shader_container, model_map);
    }

    pub fn draw(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        self.texture_cursor.as_mut().unwrap().position = glm::vec3(camera.position.x, camera.position.y , camera.position.z);
        unsafe { gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id); }
        self.texture_cursor.as_mut().unwrap().draw(&mut shader_container.get_shader("fragment".to_string()), model_map);
    }


}