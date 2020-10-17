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

pub struct Demo {
    pub cuboids: Vec<Cuboid>,
    pub plane: Plane
}

impl Demo {
    pub fn new() -> Self {
        let mut cuboids: Vec<Cuboid> = vec![];
        let light_pos = glm::vec3(2.5, 3.0, -0.5);
        cuboids.push(Cuboid::new(glm::vec3(3.0,0.0,2.0), glm::vec3(1.0, 0.5, 0.31), 1.0, 1.0, 2.0));
        cuboids.push(Cuboid::new(light_pos, glm::vec3(5.0, 7.0, 7.0), 1.0, 1.0, 1.0));
        return Demo {
            cuboids: cuboids,
            plane: Plane::new( glm::vec3(0.0,0.0,0.0), glm::vec3(0.0,1.0,0.0), 10.0, 10.0)
        };
    }

    pub fn insert_cuboid(&mut self, position: glm::Vec3, size: glm::Vec3, color: glm::Vec3) {
        self.cuboids.push(Cuboid::new(position, color, size.x, size.y, size.z));
    }

    pub fn draw_cuboids(&mut self,  shader: &mut Shader) {
        for cuboid in self.cuboids.iter_mut() {
            cuboid.draw(shader);
        }
        self.plane.draw(shader);
    }

    pub fn clean_up_cuboids(&mut self) {
        for cuboid in self.cuboids.iter_mut() {
            cuboid.clean_up();
        }
        self.plane.clean_up();
    }

    pub fn run(&mut self, shader: &mut Shader) {
        self.draw_cuboids(shader);
    }


}