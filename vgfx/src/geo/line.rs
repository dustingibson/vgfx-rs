use gl;
use gl::types::*;
use crate::Shader;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct Line {
    pub width: GLfloat,
    pub vertex_array: Vec<GLfloat>,
    pub color_array: Vec<GLfloat>,
    pub normal_array: Vec<GLfloat>,
    pub position_from: glm::Vec3,
    pub position_to: glm::Vec3,
    vertex_buffer: GLuint,
    color_buffer: GLuint,
    normal_buffer: GLuint
}

impl Line {

    pub fn new(position_from: glm::Vec3, position_to: glm::Vec3, color: glm::Vec4, width: GLfloat) -> Self {
        let mut vertex_buffer: GLuint = 0;
        let mut color_buffer: GLuint = 0;
        let mut normal_buffer: GLuint = 0;
        let vertex_array = Self::init_vertex_array(position_from, position_to, width);
        let color_array = Self::init_color_array(color);
        let normal_array = Self::init_normal_array();
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
        return Line {
            width: width,
            position_from: position_from,
            position_to: position_to,
            vertex_array: vertex_array,
            color_array: color_array,
            normal_array: normal_array,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer,
            normal_buffer: normal_buffer
        }
    }

    pub fn draw(& self, shader: &mut Shader, position: &mut glm::Vec3) {
        unsafe {
            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model(position)[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), 0);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            // gl::EnableVertexAttribArray(2);
            // gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer);
            // gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, 6*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    pub fn get_model(&self, position: &mut glm::Vec3) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, &position);
    }

    pub fn clean_up(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer);
            gl::DeleteBuffers(1, &mut self.color_buffer);
            gl::DeleteBuffers(1, &mut self.normal_buffer);
        }
    }

    fn init_vertex_array(position_from: glm::Vec3, position_to: glm::Vec3, width: f32) -> Vec<GLfloat> {

        let norm_from: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let norm_to: glm::Vec3 = position_to - position_from;
        let step = width * 0.7071;

        let low_x: GLfloat = position_from.x;
        let low_y: GLfloat = position_from.y;
        let low_z: GLfloat = position_from.z;
        let high_x: GLfloat = position_to.x;
        let high_y: GLfloat = position_to.y;
        let high_z: GLfloat = position_to.z;

        return vec![
            low_x, low_y, low_z,
            low_x + step, low_y + (step*-1.0), low_z,
            high_x, high_y, high_z,

            low_x, low_y, low_z,
            high_x + step*-1.0, high_y + step, high_z,
            high_x, high_y, high_z,
        ];


    }

    fn init_color_array(color: glm::Vec4) -> Vec<GLfloat> {
        let mut resulting_vector: Vec<GLfloat> = Vec::new();
        for x in 0..6 {
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

            //2
            0.0,  -1.0,  0.0,
            0.0,  -1.0,  0.0,
            0.0,  -1.0,  0.0,
        ];
    }

}