use gl;
use gl::types::*;
use std::mem;
use crate::Shader;
use crate::Texture;
use crate::SDLContext;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct FaceRender {
    pub vertex_buffer: Vec<GLfloat>,
    pub normal_buffer: Vec<GLfloat>,
    pub texture_buffer: Vec<GLfloat>
}

#[derive(Clone)]
pub struct FacePartitionRender {
    pub faces: Vec<FaceRender>,
    pub texture: Texture
}

impl FacePartitionRender {

    pub fn new(sdl_context: &mut SDLContext, texture: Texture) -> Self {
        return FacePartitionRender {
            faces: vec![],
            texture: texture
        }
    }

    pub fn clean_up(&mut self) {
        unsafe {
        }
    }

    pub fn draw(&mut self) {
        
    }
}