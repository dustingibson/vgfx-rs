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
    pub faces: Vec<Vec<FaceRender>>,
    pub texture_index: usize
}

impl FacePartitionRender {

    pub fn new(sdl_context: &mut SDLContext, texture_index: usize) -> Self {
        return FacePartitionRender {
            faces: vec![],
            texture_index: texture_index
        }
    }

    pub fn clean_up(&mut self) {
        unsafe {
        }
    }

    pub fn draw(&mut self) {
        
    }
}