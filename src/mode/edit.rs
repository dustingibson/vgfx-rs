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

pub struct ModelEditor {
    pub models: Vec<Model>
}

impl ModelEditor {
    pub fn new() -> Self {
        return ModelEditor {
            models: vec![]
        };
    }

    pub fn run(&self, sdl_context: & SDLContext) {
        // Next Submodel
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Left)) {
        }
        // Prev SUbmodel
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::Right)) {
        }
        // New Submodel
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
        }
        // Edit Model
        if(sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
        }
    }
}