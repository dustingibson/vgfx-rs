use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::TexturePolygon;
use crate::Plane;
use crate::ShaderContainer;
use crate::Model;
use crate::Camera;
use crate::Label2D;
use crate::SDLContext;
use crate::World;

pub struct Demo {
    //pub plane: Plane,
    pub label: Label2D,
    pub world: World
}

impl Demo {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera) -> Self {
        let mut label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 0.5, 0.5);
        return Demo {
            world: World::new_load(sdl_payload),
            label: label
        };
    }

    pub fn draw(&mut self, camera: &mut Camera,  shader_container: &mut ShaderContainer) {
        unsafe { gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id); }
        self.world.draw(&mut shader_container.get_shader("fragment".to_string()), camera);
        self.draw_hud(camera, shader_container);
    }

    pub fn clean_up(&mut self) {
        self.label.clean_up();
        self.world.clean_up();
    }

    pub fn run(&mut self, camera: &mut Camera, shader: &mut ShaderContainer) {
        self.draw(camera, shader);
    }

    pub fn draw_hud(&mut self, camera: &mut Camera, shader: &mut ShaderContainer) {
        camera.set_projection_ortho(shader);
        self.label.draw(camera, &mut shader.get_shader("fragment".to_string()));
        camera.set_projection(shader);
    }

    pub fn draw_debug(&mut self, sdl_payload: &mut SDLContext) {
        
    }

}