use gl;
use gl::types::*;
use crate::Shader;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct Plane {
    pub width: GLfloat,
    pub depth: GLfloat,
    pub vertex_array: Vec<GLfloat>,
    pub color_array: Vec<GLfloat>,
    pub normal_array: Vec<GLfloat>,
    pub position: glm::Vec3,
    vertex_buffer: GLuint,
    color_buffer: GLuint,
    normal_buffer: GLuint
}

impl Plane {

    pub fn new(point: glm::Vec3, color: glm::Vec4, width: GLfloat, depth: GLfloat) -> Self {
        let mut vertex_buffer: GLuint = 0;
        let mut color_buffer: GLuint = 0;
        let mut normal_buffer: GLuint = 0;
        let vertex_array = Self::init_vertex_array(point, width, 1.0, depth);
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
        return Plane {
            depth: depth,
            width: width,
            position: point,
            vertex_array: vertex_array,
            color_array: color_array,
            normal_array: normal_array,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer,
            normal_buffer: normal_buffer
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

            gl::DrawArrays(gl::TRIANGLES, 0, 6*3);

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

    fn init_vertex_array(point: glm::Vec3, width: GLfloat, height: GLfloat, depth: GLfloat) -> Vec<GLfloat> {
        let low_x: GLfloat = width / -2.0; 
        let high_x: GLfloat = width / 2.0;
        let low_y: GLfloat = height / -2.0;
        let high_y: GLfloat = height / 2.0;
        let low_z: GLfloat = depth / -2.0;
        let high_z: GLfloat = depth / 2.0;
        return vec![

        //1
        high_x, low_y, high_z,
        low_x, low_y, low_z,
        high_x,low_y,low_z,
        //2
        high_x,low_y, high_z,
        low_x, low_y, high_z,
        low_x, low_y,low_z,
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