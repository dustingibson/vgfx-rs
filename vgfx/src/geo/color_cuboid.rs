use gl;
use gl::types::*;
use std::mem;
use crate::Shader;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct ColorCuboid {
    pub size: glm::Vec3,
    pub vertex_array: Vec<GLfloat>,
    pub color_array: Vec<GLfloat>,
    pub position: glm::Vec3,
    vertex_buffer: GLuint,
    color_buffer: GLuint
}

impl ColorCuboid {

    pub fn new(position: glm::Vec3, size: glm::Vec3, color: glm::Vec4, texture_coord: glm::Vec4) -> Self {
        let vertex_buffer: GLuint = 0;
        let color_buffer: GLuint = 0;
        let vertex_array = Self::init_vertex_array(position, size.x, size.y, size.z);
        let color_array = Self::init_color_array(color);        

        return ColorCuboid {
            size: size,
            vertex_array: vertex_array,
            color_array: color_array,
            position: position,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer,
        }
    }

    pub fn size(&mut self) -> glm::Vec3
    {
        return self.size;
    }

    pub fn draw(&mut self, shader: &mut Shader, light_pos: glm::Vec3) {
        unsafe {

            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model()[(0,0)]);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

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
        }
    }

    fn init_vertex_array(point: glm::Vec3, width: GLfloat, height: GLfloat, depth: GLfloat) -> Vec<GLfloat> {
        let lowX: GLfloat =    (width / -2.0); 
        let highX: GLfloat =  (width / 2.0);
        let lowY: GLfloat =    (height / -2.0);
        let highY: GLfloat =  (height / 2.0);
        let lowZ: GLfloat =    (depth / -2.0);
        let highZ: GLfloat =  (depth / 2.0);
        return vec![

        //1
            lowX, lowY,lowZ,
            lowX, lowY, highZ,
            lowX, highY, highZ,
        //2
            lowX, lowY,lowZ,
            highX, highY,lowZ,
            lowX, highY,lowZ,

        //3
            highX, lowY, highZ,
             highX, lowY, lowZ,
             lowX,lowY,lowZ,
        //4
            lowX, lowY,lowZ,
            highX, lowY,lowZ,
             highX,highY,lowZ,
        //5
            lowX, lowY,lowZ,
            lowX, highY, highZ,
            lowX, highY,lowZ,
        //6
            highX,lowY, highZ,
            lowX, lowY, lowZ,
            lowX, lowY,highZ,
        //7
            lowX, lowY, highZ,
            highX, lowY, highZ,
            lowX, highY, highZ,
        //8
            highX, highY, highZ,
            highX, highY,lowZ,
            highX, lowY,lowZ,
        //9
            highX, lowY,lowZ,
            highX, lowY, highZ,
            highX, highY, highZ,
        //10
            highX, highY, highZ,
            highX, highY,lowZ,
            lowX, highY,lowZ,
        //11
            highX, highY, highZ,
           lowX, highY,lowZ,
           lowX, highY, highZ,
        //12
            highX, highY, highZ,
            lowX, highY, highZ,
            highX, lowY, highZ,

        ];
    }

    fn init_color_array(color: glm::Vec4) -> Vec<GLfloat> {
        let mut resulting_vector: Vec<GLfloat> = Vec::new();
        for x in 0..36 {
            resulting_vector.push(color.x);
            resulting_vector.push(color.y);
            resulting_vector.push(color.z);
            resulting_vector.push(color.w);
        }
        return resulting_vector;
    }

}