use gl;
use gl::types::*;
use std::mem;
use crate::Shader;
use crate::Texture;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct Cuboid {
    pub point: glm::Vec3,
    pub length: GLfloat,
    pub width: GLfloat,
    pub height: GLfloat,
    pub vertex_array: Vec<GLfloat>,
    pub color_array: Vec<GLfloat>,
    pub normal_array: Vec<GLfloat>,
    pub texture_array: Vec<GLfloat>,
    pub position: glm::Vec3,
    vertex_buffer: GLuint,
    color_buffer: GLuint,
    normal_buffer: GLuint,
    texture_buffer: GLuint,
    texture: Texture
}

impl Cuboid {

    pub fn new(point: glm::Vec3, color: glm::Vec4, texture_coord: glm::Vec4, length: GLfloat, width: GLfloat, height: GLfloat) -> Self {
        let vertex_buffer: GLuint = 0;
        let color_buffer: GLuint = 0;
        let normal_buffer: GLuint = 0;
        let texture_buffer: GLuint = 0;
        let texture = Texture::new("test".to_string());
        let vertex_array = Self::init_vertex_array(point, length, width, height);
        let color_array = Self::init_color_array(color);
        let normal_array = Self::init_normal_array();
        let texture_array = Self::init_texture_array(texture_coord.x, texture_coord.y, texture_coord.z, texture_coord.w);
        

        return Cuboid {
            point: point,
            length: length,
            width: width,
            height: height,
            position: point,
            texture_array: texture_array,
            vertex_array: vertex_array,
            color_array: color_array,
            normal_array: normal_array,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer,
            normal_buffer: normal_buffer,
            texture_buffer: texture_buffer,
            texture: texture
        }
    }

    pub fn draw(&mut self, shader: &mut Shader, light_pos: glm::Vec3) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.texture_id);

            //shader.set_texture(self.texture.texture_id);
            gl::Uniform3fv(shader.get_uniform_location("lightPos".to_string()), 1, &light_pos[0]);
            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model()[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), 1);

            shader.set_texture(self.texture.texture_id);

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
            gl::DeleteBuffers(1, &mut self.texture_buffer);
        }
    }

    fn init_vertex_array(point: glm::Vec3, width: GLfloat, height: GLfloat, depth: GLfloat) -> Vec<GLfloat> {
        let lowX: GLfloat = point.x + (width / -2.0); 
        let highX: GLfloat = point.x + (width / 2.0);
        let lowY: GLfloat = point.y + (height / -2.0);
        let highY: GLfloat = point.y + (height / 2.0);
        let lowZ: GLfloat = point.z + (depth / -2.0);
        let highZ: GLfloat = point.z + (depth / 2.0);
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

    fn init_texture_array(lowX: GLfloat, lowY: GLfloat, highX: GLfloat, highY: GLfloat) -> Vec<GLfloat> {
        return vec![
            // 1 - DONE (front left)
            lowX, highY,
            highX, highY, 
            highX, lowY,

            // 2 - DONE (left left)
            highX, highY,
            lowX, lowY,
            highX, lowY, 

            // 3 - DONE (bottom left)
            highX, lowY,
            lowX, lowY, 
            lowX, highY,

            // 4 - DONE (left right)
            highX, highY,
            lowX, highY,
            lowX, lowY, 


            // 5 - DONE (front right)
            lowX, highY,
            highX, lowY, 
            lowX, lowY,

            // 6 - DONE (bottom right)
            highX, lowY,
            lowX, highY,
            highX, highY, 
            
            // 7 - DONE (right left)
            lowX, highY,
            highX, highY, 
            lowX, lowY,

            // 8 - DONE (back right)
            lowX,  lowY,
            highX, lowY, 
            highX, highY,

            // 9 - done (back left)
            highX, highY,
            lowX, highY,
            lowX, lowY, 

            // 10 = done (top right)
            lowX, highY, 
            highX, highY,
            highX, lowY,

            // 11 - done (top left)
            lowX, highY,
            highX, lowY, 
            lowX, lowY,

            // 12 - DONE (right right)
            highX, lowY,
            lowX, lowY, 
            highX, highY,
        ];
    }

    fn init_normal_array() -> Vec<GLfloat> {
        return vec![
            //1
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,

            //2
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,

            //3
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,

            //4
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,

            //5
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,

            //6
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,
            
            //7
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,

            //8
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,
            
            //9
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,

            //10
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,

            //11
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,

            //12
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,
        ];
    }
}