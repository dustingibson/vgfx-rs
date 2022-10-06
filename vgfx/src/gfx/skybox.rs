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
        texture.create_cube_map_texture_buffer_from_byte_data([left_bytes, right_bytes, top_bytes, bottom_bytes, front_bytes, back_bytes].to_vec());
        let mut skybox = Skybox {
            texture: Texture::new("left".to_string()),
            vertex_buffer: Self::init_texture_array(),
            vertex_buffer_id: 0,
            vao: 0
        };
        skybox.init_gl();
        return skybox;
    }

    fn init_vertex_buffer() -> Vec<GLfloat> {
        return [
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
        
            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,
        
             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,
        
            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,
        
            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,
        
            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0
        ].to_vec();
    }

    fn init_texture_array() -> Vec<GLfloat> {
        let low_x = 0.0;
        let low_y = 0.0;
        let high_x = 1.0;
        let high_y = 1.0;

        return vec![
            low_x, high_y,
            high_x, high_y, 
            high_x, low_y,
            low_x, high_y,
            high_x, low_y, 
            low_x, low_y
        ];
    }

    pub fn init_gl(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vertex_buffer_id);
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.vertex_buffer.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                self.vertex_buffer.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(0);
        }
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    // pub fn draw(&self, shader: &mut Shader, position: &mut glm::Vec3, texture: &Texture) {
    //     unsafe {
        
    //         gl::BindVertexArray(self.vao);

    //         gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model(position)[(0,0)]);
    //         gl::Uniform1i(shader.get_uniform_location("textured".to_string()), if texture.has_img {1} else {0} );
    //         gl::Uniform4f(shader.get_uniform_location("ambientColor".to_string()),  texture.texture_properties.ambient_color[0], texture.texture_properties.ambient_color[1], texture.texture_properties.ambient_color[2], 1.0);
    //         gl::Uniform1i(shader.get_uniform_location("textureSample".to_string()), 0);

    //         gl::ActiveTexture(gl::TEXTURE0);
    //         gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);

    //         gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
    //         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

    //         gl::DrawArrays(gl::TRIANGLES, 0, 36);

    //         gl::EnableVertexAttribArray(0);
    //         gl::BindVertexArray(0);
    //     }
    // }
}