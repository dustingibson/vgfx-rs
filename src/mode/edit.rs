use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::Plane;
use crate::Shader;
use crate::ShaderContainer;
use crate::Model;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::BFile;

pub struct ModelEditor {
    pub models: Vec<Model>,
    pub main_label: Label2D
}

impl ModelEditor {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera) -> Self {
        let mut label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), 0.5, 0.5);

        let mut models: Vec<Model> = Vec::new();
        models.push(Model::fromFile(glm::vec3(0.0,0.0, 0.0)));

        return ModelEditor {
            models: models,
            main_label: label
        };
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader: &mut ShaderContainer) {
        // Next Submodel
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Left)) {
        }
        // Prev SUbmodel
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Right)) {
        }
        // New Submodel
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
            self.main_label.change_text(sdl_context, "NEW".to_string());
        }
        // Edit Model
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
        }
        self.draw(camera, shader);
    }

    pub fn draw(&mut self, camera: &mut Camera, shader: &mut ShaderContainer) {
        unsafe { gl::UseProgram(shader.get_shader("fragment".to_string()).program_id); }
        for model in self.models.iter_mut() {
            model.draw(&mut shader.get_shader("fragment".to_string()));
        }
        camera.set_projection_ortho(shader);
        self.main_label.draw(camera, &mut shader.get_shader("fragment".to_string()));
        camera.set_projection(shader);
    }


}