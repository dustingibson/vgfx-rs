use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::SubModel;
use crate::Shader;
#[derive(Clone)]

pub struct Model {
    pub sub_models: Vec<SubModel>,
    pub size: glm::Vec3
}

impl Model {
    pub fn new(size: glm::Vec3) -> Self {
        return Model {
            sub_models: vec![],
            size: size
        };
    }

    pub fn insert_submodel(&mut self, position: glm::Vec3, size: glm::Vec3, cuboids: &mut Vec<Cuboid>) {
        self.sub_models.push(SubModel::new(position, size, cuboids));
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        for sub_model in self.sub_models.iter_mut() {
            sub_model.draw(shader);
        }
    }
}