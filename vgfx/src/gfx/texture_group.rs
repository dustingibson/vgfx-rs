use gl;
use gl::types::*;
use std::mem;
use crate::Shader;
use crate::Texture;
use crate::SDLContext;
use crate::VecOps;
use crate::model::model::ModelInstance;

use super::face::FacePartitionRender;
use super::text::Text;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct TextureGroupRenderer {
    pub vertex_buffer: Vec<GLfloat>,
    pub normal_buffer: Vec<GLfloat>,
    pub texture_buffer: Vec<GLfloat>,
    pub vertex_buffer_id: GLuint,
    pub normal_buffer_id: GLuint,
    pub texture_buffer_id: GLuint,
    pub vao: GLuint,
    pub length: i32,
    pub texture_index: usize,
    pub mode: u8,
    pub texture: Texture
}

impl TextureGroupRenderer {

    pub fn new(texture: Texture) -> Self {
        let mut partition = TextureGroupRenderer {
            texture_index: texture.texture_id as usize,
            vertex_buffer: vec![],
            normal_buffer: vec![],
            texture_buffer: vec![],
            vertex_buffer_id: 0,
            texture_buffer_id: 0,
            normal_buffer_id: 0,
            vao: 0,
            length: 0,
            mode: 2,
            texture: texture
        };
        partition.initGL();
        return partition;
    }

    pub fn initGL(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vertex_buffer_id);
            gl::GenBuffers(1, &mut self.normal_buffer_id);
            gl::GenBuffers(1, &mut self.texture_buffer_id);
        }
        self.update_buffers();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(0);
            if  self.mode == 3 { gl::EnableVertexAttribArray(1) }; 
            gl::EnableVertexAttribArray(2);
        }
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn append_buffer(&mut self, vertex_buffer: &mut Vec<GLfloat>, normal_buffer: &mut Vec<GLfloat>, texture_buffer: &mut Vec<GLfloat>, length: i32) {
        self.vertex_buffer.append(&mut vertex_buffer.clone());
        self.normal_buffer.append(&mut normal_buffer.clone());
        self.texture_buffer.append(&mut texture_buffer.clone());
        self.length += length;
    }

    pub fn update_buffer_from_partition(&mut self, face_partition: &mut FacePartitionRender) {
        self.append_buffer(&mut face_partition.vertex_buffer, &mut face_partition.normal_buffer, &mut face_partition.texture_buffer, face_partition.length);
    }

    pub fn update_buffers(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.vertex_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.vertex_buffer.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        if self.mode == 3 {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer_id);
                gl::BufferData(
                    gl::ARRAY_BUFFER, 
                    (self.normal_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                    self.normal_buffer.as_ptr() as *const gl::types::GLvoid, 
                    gl::STATIC_DRAW);
                //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.texture_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.texture_buffer.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn update_subbuffers(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::BufferSubData(
                gl::ARRAY_BUFFER, 
                0,
                (self.vertex_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.vertex_buffer.as_ptr() as *const gl::types::GLvoid);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        if self.mode == 3 {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer_id);
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (self.normal_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                    self.normal_buffer.as_ptr() as *const gl::types::GLvoid);
                //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer_id);
            gl::BufferSubData(
                gl::ARRAY_BUFFER, 
                0,
                (self.texture_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.texture_buffer.as_ptr() as *const gl::types::GLvoid);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn reset_buffers(&mut self) {
        self.length = 0;
        self.normal_buffer.drain(0..self.normal_buffer.len());
        self.vertex_buffer.drain(0..self.vertex_buffer.len());
        self.texture_buffer.drain(0..self.texture_buffer.len());
    }

    pub fn clean_up(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer_id);
            gl::DeleteBuffers(1, &mut self.normal_buffer_id);
            gl::DeleteBuffers(1, &mut self.texture_buffer_id);
        }
    }

    pub fn draw(&mut self, shader: &mut Shader, position: &mut glm::Vec3) {
        if self.length > 0 {
            self.update_subbuffers();
            unsafe {
                gl::BindVertexArray(self.vao);

                gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model(position)[(0,0)]);
                gl::Uniform1i(shader.get_uniform_location("textured".to_string()), if self.texture.has_img {1} else {0} );
                gl::Uniform4f(shader.get_uniform_location("ambientColor".to_string()),  self.texture.texture_properties.ambient_color[0], self.texture.texture_properties.ambient_color[1], self.texture.texture_properties.ambient_color[2], 1.0);
                gl::Uniform1i(shader.get_uniform_location("textureSample".to_string()), 0);

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.texture.texture_id);

                gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());
                if self.mode == 3 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer_id);
                    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());
                }
                gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer_id);
                gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

                gl::DrawArrays(gl::TRIANGLES, 0, self.length*3);

                gl::EnableVertexAttribArray(0);
                gl::BindVertexArray(0);
            }
            self.reset_buffers();
        }
    }

    pub fn get_model(&mut self, position: &mut glm::Vec3) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, position);
    }

}