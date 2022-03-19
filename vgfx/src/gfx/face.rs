use gl;
use gl::types::*;
use std::mem;
use crate::Shader;
use crate::Texture;
use crate::SDLContext;
use crate::VecOps;

use super::text::Text;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct FacePartitionRender {
    pub vertex_buffer: Vec<GLfloat>,
    pub normal_buffer: Vec<GLfloat>,
    pub texture_buffer: Vec<GLfloat>,
    pub vertex_buffer_id: GLuint,
    pub normal_buffer_id: GLuint,
    pub texture_buffer_id: GLuint,
    pub length: i32,
    pub texture_index: usize,
    pub mode: u8
}

impl FacePartitionRender {

    pub fn new(vertex_buffer: Vec<GLfloat>, normal_buffer: Vec<GLfloat>, texture_buffer: Vec<GLfloat>, texture_index: usize, length: i32, mode: u8, init_gl: bool) -> Self {
        let mut partition = FacePartitionRender {
            texture_index: texture_index,
            vertex_buffer: vertex_buffer,
            normal_buffer: normal_buffer,
            texture_buffer: texture_buffer,
            vertex_buffer_id: 0,
            texture_buffer_id: 0,
            normal_buffer_id: 0,
            length: length,
            mode: mode
        };
        if init_gl {
            partition.initGL();
        }
        return partition;
    }

    pub fn initGL(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.vertex_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.vertex_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.vertex_buffer.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        if self.mode == 3 {
            unsafe {
                gl::GenBuffers(1, &mut self.normal_buffer_id);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer_id);
                gl::BufferData(
                    gl::ARRAY_BUFFER, 
                    (self.normal_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                    self.normal_buffer.as_ptr() as *const gl::types::GLvoid, 
                    gl::STATIC_DRAW);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }
        }
        unsafe {
            gl::GenBuffers(1, &mut self.texture_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.texture_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.texture_buffer.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn clean_up(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer_id);
            gl::DeleteBuffers(1, &mut self.normal_buffer_id);
            gl::DeleteBuffers(1, &mut self.texture_buffer_id);
        }
    }

    pub fn draw(&mut self, shader: &mut Shader, position: &mut glm::Vec3, texture: &mut Texture) {
        unsafe {
            //let ambient_color = &texture.texture_properties.ambient_color;

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);
            //shader.set_texture(texture.texture_id);

            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model(position)[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), if texture.has_img {1} else {0} );
            gl::Uniform4f(shader.get_uniform_location("ambientColor".to_string()),  texture.texture_properties.ambient_color[0], texture.texture_properties.ambient_color[1], texture.texture_properties.ambient_color[2], 1.0);
            gl::Uniform1i(shader.get_uniform_location("textureSample".to_string()), 0);
            //gl::Uniform4f(shader.get_uniform_location("ambientColor".to_string()),  ambient_color[0], ambient_color[1], ambient_color[2], 1.0);
            //gl::Uniform4f(shader.get_uniform_location("ambientColor".to_string()),  1.0, 0.0, 0.0, 1.0);


            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            if self.mode == 3 {
                gl::EnableVertexAttribArray(1);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer_id);
                gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());
            }

            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer_id);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, self.length*9);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);
        }
    }

    pub fn get_model(&mut self, position: &mut glm::Vec3) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, position);
    }

}