use std::collections::HashMap;

use gl;
use sdl2::keyboard::Scancode;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;

use super::texture_crud::TextureCrud;

pub struct Editor {
    camera_coord_label: Label2D,
    texture_crud: TextureCrud,
    mode_index: u32,
    max_mode_index: u32,
}

pub enum EditorModes {
    TextureCrud
}

impl Editor {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let label: Label2D = Label2D::new( sdl_payload, camera, camera.coord_str(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 0.2, 0.05, 64);
        return Editor {
            camera_coord_label: label,
            mode_index: 0,
            max_mode_index: 0,
            texture_crud: TextureCrud::new(sdl_payload, camera, model_map)
        };
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        self.camera_coord_label.change_text(sdl_context, camera.coord_str());
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Left)) {
        }
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Right)) {
        }
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
        }
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
        }
        match self.mode_index {
            0 => self.texture_crud.run(sdl_context, camera, shader_container, model_map),
            _ => {}
        }
        self.draw(camera, shader_container, model_map);
    }

    pub fn draw(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        unsafe { gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id); }
        camera.set_projection_ortho(shader_container);
        self.camera_coord_label.draw(&mut shader_container.get_shader("fragment".to_string()));
        camera.set_projection(shader_container);
    }


}