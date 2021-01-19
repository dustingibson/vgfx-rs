use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::Shader;
#[derive(Clone)]

pub struct SubModel {
    pub cuboids: Vec<Cuboid>,
    pub position: glm::Vec3,
    pub size: glm::Vec3,
    pub vertex_buffer: GLuint,
    pub color_buffer: GLuint,
    pub normal_buffer: GLuint,
    pub texture_buffer: GLuint,
    pub length: i32
}

impl SubModel {
    pub fn new(position: glm::Vec3, size: glm::Vec3, cuboids: &mut Vec<Cuboid>) -> Self {

        let mut vertex_buffer: GLuint = 0;
        let mut color_buffer: GLuint = 0;
        let mut normal_buffer: GLuint = 0;
        let mut texture_buffer: GLuint = 0;
        let mut vertex_array = vec![];
        let mut color_array = vec![];
        let mut normal_array = vec![];
        let mut texture_array = vec![];

        for cur_cuboid in cuboids.iter_mut() {
            vertex_array.append(&mut cur_cuboid.vertex_array);
            color_array.append(&mut cur_cuboid.color_array);
            normal_array.append(&mut cur_cuboid.normal_array);
            texture_array.append(&mut cur_cuboid.texture_array);
        }
        
        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (vertex_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                vertex_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        unsafe {
            gl::GenBuffers(1, &mut color_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (color_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                color_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        unsafe {
            gl::GenBuffers(1, &mut normal_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (normal_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                normal_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        unsafe {
            gl::GenBuffers(1, &mut texture_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, texture_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (texture_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                texture_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        return SubModel {
            position: position,
            cuboids: cuboids.to_vec(),
            size: size,
            color_buffer: color_buffer,
            normal_buffer: normal_buffer,
            vertex_buffer: vertex_buffer,
            texture_buffer: texture_buffer,
            length:  cuboids.len() as i32
        };
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        unsafe {

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, 1);
            shader.set_texture(1);

            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model()[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), 1);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(3);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer);
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, self.length*12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    pub fn get_model(&mut self) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, &self.position);
    }

    pub fn insert_cuboid(&mut self, position: glm::Vec3) {

    }
}