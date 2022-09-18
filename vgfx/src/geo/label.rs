use gl;
use gl::types::*;
use crate::Shader;
use crate::Text;
use crate::SDLContext;
use crate::Camera;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct Label {
    pub width: GLfloat,
    pub height: GLfloat,
    pub vertex_array: Vec<GLfloat>,
    pub normal_array: Vec<GLfloat>,
    pub position: glm::Vec3,
    pub text_texture: Text,
    vertex_buffer: GLuint,
    normal_buffer: GLuint,
    texture_buffer: GLuint
}

impl Label {

    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, text: String, color: glm::Vec4, width: GLfloat, height: GLfloat, font_size: u16) -> Self {
        let mut vertex_buffer: GLuint = 0;
        let mut normal_buffer: GLuint = 0;
        let mut texture_buffer: GLuint = 0;
        //let new_position = glm::vec3(camera.position.x, camera.position.y, camera.position.z);
        let new_position = glm::vec3(camera.position.z, camera.position.x, camera.position.x);
        //let new_position = glm::vec3(0.0, 0.0, 3.0);
        let vertex_array = Self::init_vertex_array(new_position, 0.0, 1.0, 1.0);
        let normal_array = Self::init_normal_array();
        let texture_array = Self::init_texture_array();

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
        let text_texture = Text::new(sdl_payload, text, new_position, font_size);
        return Label {
            text_texture: text_texture,
            width: width,
            height: height,
            position: new_position,
            vertex_array: vertex_array,
            normal_array: normal_array,
            vertex_buffer: vertex_buffer,
            normal_buffer: normal_buffer,
            texture_buffer: texture_buffer
        }
    }

    pub fn draw(&mut self, camera: &mut Camera, shader: &mut Shader) {
        self.change_vertex(camera);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.text_texture.texture.texture_id);
            shader.set_texture(1);

            gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model()[(0,0)]);
            gl::Uniform1i(shader.get_uniform_location("textured".to_string()), 1);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, 12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);

        }
    }

    pub fn get_model(&mut self) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, &self.position);
    }

    pub fn clean_up(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer);
            gl::DeleteBuffers(1, &mut self.normal_buffer);
            gl::DeleteBuffers(1, &mut self.texture_buffer);
        }
    }

    fn change_vertex(&mut self, camera: &mut Camera) {
        // //let new_position = glm::vec3(camera.position.x, camera.position.y, camera.position.z);
        // //println!("{}", self.vertex_array[0]);
        // let new_position = glm::vec3(camera.position.z, camera.position.y, camera.position.x);
        // //let new_position = glm::vec3(camera.position.x, camera.position.z, camera.position.y);
        // self.vertex_array = Self::init_vertex_array(new_position, 0.0, 1.0, 1.0);
        // unsafe {
        //     gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
        //     gl::BufferData(
        //         gl::ARRAY_BUFFER, 
        //         (self.vertex_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
        //         self.vertex_array.as_ptr() as *const gl::types::GLvoid, 
        //         gl::STATIC_DRAW);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // }
    }

    fn init_vertex_array(point: glm::Vec3, width: GLfloat, height: GLfloat, depth: GLfloat) -> Vec<GLfloat> {
        let low_x: GLfloat = point.x + (width / -2.0); 
        let high_x: GLfloat = point.x + (width / 2.0);
        let low_y: GLfloat = point.y + (height / -2.0);
        let high_y: GLfloat = point.y + (height / 2.0);
        let low_z: GLfloat = point.z + (depth / -2.0);
        let high_z: GLfloat = point.z + (depth / 2.0);
        return vec![

        //1
            low_x, low_y,low_z,
            low_x, low_y, high_z,
            low_x, high_y, high_z,
        //5
            low_x, low_y,low_z,
            low_x, high_y, high_z,
            low_x, high_y,low_z,
        ];
    }

    fn init_color_array(color: glm::Vec4) -> Vec<GLfloat> {
        let mut resulting_vector: Vec<GLfloat> = Vec::new();
        for x in 0..12 {
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

    fn init_texture_array() -> Vec<GLfloat> {
        let low_x = 0.0;
        let low_y = 0.0;
        let high_x = 1.0;
        let high_y = 1.0;

        return vec![
            // 1 - DONE (front left)
            low_x, high_y,
            high_x, high_y, 
            high_x, low_y,

            // 5 - DONE (front right)
            low_x, high_y,
            high_x, low_y, 
            low_x, low_y
        ];
    }
}