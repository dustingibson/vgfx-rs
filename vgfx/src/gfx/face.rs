use gl;
use gl::types::*;
use crate::Shader;
use crate::Texture;

use super::shader::AdditionalUniforms;
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
    pub vao: GLuint,
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
            vao: 0,
            length: length,
            mode: mode
        };
        if init_gl {
            partition.init_gl();
        }
        return partition;
    }

    pub fn init_gl(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vertex_buffer_id);
            gl::GenBuffers(1, &mut self.normal_buffer_id);
            gl::GenBuffers(1, &mut self.texture_buffer_id);
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

    pub fn clean_up(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer_id);
            gl::DeleteBuffers(1, &mut self.normal_buffer_id);
            gl::DeleteBuffers(1, &mut self.texture_buffer_id);
        }
    }

    pub fn draw(&self, shader: &mut Shader, additional_uniforms: Option<&AdditionalUniforms>, position: &mut glm::Vec3, texture: &Texture, additional_textures: Option<&Vec<Texture>>, scale: &mut glm::Vec3, rotate: &mut glm::Vec3) {
        unsafe {            
            gl::BindVertexArray(self.vao);

            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model(position, scale, rotate)[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), if texture.has_img {1} else {0} );
            gl::Uniform4f(shader.get_uniform_location("ambientColor".to_string()),  texture.texture_properties.ambient_color[0], texture.texture_properties.ambient_color[1], texture.texture_properties.ambient_color[2], 1.0);
            gl::Uniform1i(shader.get_uniform_location("textureSample".to_string()), 0);
            //gl::Uniform3f(shader.get_uniform_location("lightPos".to_string()), 10075.0, 70.0, 8429.0 );
            if (additional_uniforms.is_some()) {
                additional_uniforms.unwrap().BindUniforms(shader);
            }


            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);

            if additional_textures.is_some() {
                //println!("Had additional textures {}", additional_textures.unwrap().len());
                for additional_texture in additional_textures.unwrap() {
                    gl::ActiveTexture(additional_texture.sampler_id);
                    gl::BindTexture(gl::TEXTURE_2D, additional_texture.texture_id);
                }
            }


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
    }

    pub fn rotate(&self, rotate: &mut glm::Vec3) -> glm::Mat4 {
        let c_model: glm::Mat4 = glm::Mat4::identity();
        let rx = glm::rotate(&c_model, rotate.x, &glm::Vec3::new(1.0, 0.0, 0.0));
        let ry = glm::rotate(&c_model, rotate.y, &glm::Vec3::new(0.0, 1.0, 0.0));
        let rz = glm::rotate(&c_model, rotate.z, &glm::Vec3::new(0.0, 0.0, 1.0));;
        return rx*ry*rz;

    }

    pub fn get_model(&self, position: &mut glm::Vec3, scale: &mut glm::Vec3, rotate: &mut glm::Vec3) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        let s = glm::scale(&c_model, scale);
        let r = self.rotate(rotate);
        let t = glm::translate(&c_model, position);
        return t*r*s;
    }

}