use gl::types::{GLfloat, GLuint};

use crate::gfx::shader::Shader;
use super::texture::Texture;
extern crate nalgebra_glm as glm;

pub struct Skybox {
    pub texture: Texture,
    pub vertex_buffer: Vec<GLfloat>,
    pub vertex_buffer_id: GLuint,
    pub vao: GLuint
}

impl Skybox {
    pub fn new(left_bytes: Vec<u8>, right_bytes: Vec<u8>, top_bytes: Vec<u8>, bottom_bytes: Vec<u8>, front_bytes: Vec<u8>, back_bytes: Vec<u8> ) -> Self {
        let mut texture = Texture::new("skybox".to_string());
        texture.create_cube_map_texture_buffer_from_byte_data([right_bytes, left_bytes, top_bytes, bottom_bytes, front_bytes, back_bytes].to_vec());
        let mut skybox = Skybox {
            texture: texture,
            vertex_buffer: Self::init_vertex_buffer(),
            vertex_buffer_id: 0,
            vao: 0
        };
        skybox.init_gl();
        return skybox;
    }

    fn init_vertex_buffer() -> Vec<GLfloat> {
        let high = 1000.0;
        let low = -1000.0;
        return [
            low,  high, low,
            low, low, low,
             high, low, low,
             high, low, low,
             high,  high, low,
            low,  high, low,
        
            low, low,  high,
            low, low, low,
            low,  high, low,
            low,  high, low,
            low,  high,  high,
            low, low,  high,
        
             high, low, low,
             high, low,  high,
             high,  high,  high,
             high,  high,  high,
             high,  high, low,
             high, low, low,
        
            low, low,  high,
            low,  high,  high,
             high,  high,  high,
             high,  high,  high,
             high, low,  high,
            low, low,  high,
        
            low,  high, low,
             high,  high, low,
             high,  high,  high,
             high,  high,  high,
            low,  high,  high,
            low,  high, low,
        
            low, low, low,
            low, low,  high,
             high, low, low,
             high, low, low,
            low, low,  high,
             high, low,  high
        ].to_vec();
    }

    pub fn init_gl(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vertex_buffer_id);
        }
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.vertex_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.vertex_buffer.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        unsafe {
            gl::EnableVertexAttribArray(0);
        }
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self, shader: &mut Shader) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
            gl::BindVertexArray(self.vao);

            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model(&mut glm::vec3(0.0, 0.0, 0.0))[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textureSample".to_string()), 0);
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture.texture_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::DepthFunc(gl::LESS);
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
    }

    pub fn get_model(&self, position: &mut glm::Vec3) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, position);
    }
}