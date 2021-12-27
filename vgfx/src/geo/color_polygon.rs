use gl;
use gl::types::*;
use std::mem;
use crate::Shader;
use crate::Texture;
use crate::SDLContext;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct ColorPolygon {
    pub vertex_array: Vec<GLfloat>,
    pub color_array: Vec<GLfloat>,
    pub normal_array: Vec<GLfloat>,
    pub texture_array: Vec<GLfloat>,
    pub position: glm::Vec3,
    vertex_buffer: GLuint,
    color_buffer: GLuint,
    normal_buffer: GLuint,
    texture_buffer: GLuint,
    texture_id: GLuint
}

impl ColorPolygon {

    pub fn new(sdl_context: &mut SDLContext, point: glm::Vec3, color: glm::Vec4) -> Self {
        let mut vertex_buffer: GLuint = 0;
        let mut color_buffer: GLuint = 0;
        let mut normal_buffer: GLuint = 0;
        let vertex_array = Self::init_vertex_array(point);
        let color_array = Self::init_color_array(color);
        let normal_array = Self::init_normal_array();
        let texture_rect: glm::Vec4 = sdl_context.terrain_texture.get_rect_from_texture_image("select".to_string());
        let texture_array = Self::init_texture_array(texture_rect.x, texture_rect.y, texture_rect.z, texture_rect.w);
        // unsafe {
        //     gl::GenBuffers(1, &mut vertex_buffer);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        //     gl::BufferData(
        //         gl::ARRAY_BUFFER, 
        //         (vertex_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
        //         vertex_array.as_ptr() as *const gl::types::GLvoid, 
        //         gl::STATIC_DRAW);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // }
        // unsafe {
        //     gl::GenBuffers(1, &mut color_buffer);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
        //     gl::BufferData(
        //         gl::ARRAY_BUFFER, 
        //         (color_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
        //         color_array.as_ptr() as *const gl::types::GLvoid, 
        //         gl::STATIC_DRAW);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // }
        // unsafe {
        //     gl::GenBuffers(1, &mut normal_buffer);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
        //     gl::BufferData(
        //         gl::ARRAY_BUFFER, 
        //         (normal_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
        //         normal_array.as_ptr() as *const gl::types::GLvoid, 
        //         gl::STATIC_DRAW);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // }
        return ColorPolygon {
            position: point,
            vertex_array: vertex_array,
            color_array: color_array,
            normal_array: normal_array,
            texture_array: texture_array,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer,
            normal_buffer: normal_buffer,
            texture_buffer: 0,
            texture_id: sdl_context.terrain_texture.texture_id
        }
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        unsafe {
            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model()[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), 0);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, 12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    pub fn get_model(&mut self) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, &self.position);
    }

    pub fn clean_up(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer);
            gl::DeleteBuffers(1, &mut self.color_buffer);
            gl::DeleteBuffers(1, &mut self.normal_buffer);
        }
    }

    fn init_vertex_array(point: glm::Vec3) -> Vec<GLfloat> {
        return vec![
        //1
            -0.5 + point.x, -0.5 + point.y, 0.0  + point.z,
            0.5 + point.x, -0.5 + point.y, 0.0  + point.z,
            0.0 + point.x, 0.5 + point.y, 0.0  + point.z
        ];
    }

    fn init_color_array(color: glm::Vec4) -> Vec<GLfloat> {
        let mut resulting_vector: Vec<GLfloat> = Vec::new();
        for x in 0..1 {
            resulting_vector.push(color.x);
            resulting_vector.push(color.y);
            resulting_vector.push(color.z);
            resulting_vector.push(color.w);
        }
        return resulting_vector;
    }

    fn init_normal_array() -> Vec<GLfloat> {
        return vec![
            //1
            0.0,  -1.0,  0.0,
            0.0,  -1.0,  0.0,
            0.0,  -1.0,  0.0,
        ];
    }

    fn init_texture_array(lowX: GLfloat, lowY: GLfloat, highX: GLfloat, highY: GLfloat) -> Vec<GLfloat> {
        return vec![
            // 1 - DONE (front left)
            lowX, highY,
            highX, highY, 
            highX, lowY,

            // 5 - DONE (front right)
            lowX, highY,
            highX, lowY, 
            lowX, lowY
        ];
    }


}