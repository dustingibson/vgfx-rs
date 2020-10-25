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

pub struct Demo {
    pub model: Model,
    pub plane: Plane
}

impl Demo {
    pub fn new() -> Self {
        let mut model: Model = Model::new(glm::vec3(0.0,0.0,0.0));
        let mut cuboids: Vec<Cuboid> = vec![];
        let light_pos = glm::vec3(2.5, 5.0, -0.5);
        for i in 0..20 {
            for j in 0..20 {
                for k in 0..20 {
                    cuboids.push(Cuboid::new(glm::vec3(0.05 * i as f32,0.05 * j as f32, 0.05 * k as f32), glm::vec3(0.05 * i as f32, 0.05 * j as f32, 0.05 * k as f32), 0.05, 0.05, 0.05));
                }
            }
        }
        //cuboids.push(Cuboid::new(glm::vec3(3.0,0.0,2.0), glm::vec3(1.0, 0.5, 0.31), 1.0, 1.0, 2.0));
        //cuboids.push(Cuboid::new(light_pos, glm::vec3(5.0, 7.0, 7.0), 1.0, 1.0, 1.0));

        model.insert_submodel(glm::vec3(0.0,0.0, 0.0), glm::vec3(30.0,30.0,30.0), &mut cuboids);
        
        //cuboids.push(Cuboid::new(glm::vec3(3.0,0.0,2.0), glm::vec3(1.0, 0.5, 0.31), 1.0, 1.0, 2.0));
        //cuboids.push(Cuboid::new(light_pos, glm::vec3(5.0, 7.0, 7.0), 1.0, 1.0, 1.0));
        return Demo {
            model: model,
            plane: Plane::new( glm::vec3(0.0,0.0,0.0), glm::vec3(0.0,1.0,0.0), 10.0, 10.0)
        };
    }

    pub fn insert_cuboid(&mut self, position: glm::Vec3, size: glm::Vec3, color: glm::Vec3) {
        //self.cuboids.push(Cuboid::new(position, color, size.x, size.y, size.z));
    }

    pub fn draw_cuboids(&mut self,  shader_container: &mut ShaderContainer) {
        unsafe { gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id); }
        self.plane.draw(&mut shader_container.get_shader("fragment".to_string()));
        self.model.draw(&mut shader_container.get_shader("fragment".to_string()));
    }

    pub fn clean_up_cuboids(&mut self) {
        self.plane.clean_up();
    }

    pub fn run(&mut self, shader: &mut ShaderContainer) {
        self.draw_cuboids(shader);
    }


}