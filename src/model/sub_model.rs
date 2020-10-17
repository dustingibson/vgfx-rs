use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::Cuboid;

pub struct SubModel {
    pub cuboids: Vec<Cuboid>,
    pub position: glm::Vec3,
    pub size: glm::Vec3
}

impl SubModel {
    pub fn new(position: glm::Vec3, size: glm::Vec3) -> Self {
        return SubModel {
            position: position,
            cuboids: vec![],
            size: size
        };
    }

    pub fn insert_cuboid(&mut self, position: glm::Vec3) {

    }
}