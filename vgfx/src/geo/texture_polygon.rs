use gl;
use gl::types::*;
use crate::SDLContext;
extern crate nalgebra_glm as glm;
extern crate libc;

#[derive(Clone)]
pub struct TexturePolygon {
    pub vertex_array: Vec<GLfloat>,
    pub normal_array: Vec<GLfloat>,
    pub texture_array: Vec<GLfloat>,
    pub position: glm::Vec3,
    texture_buffer: GLuint,
    texture_id: GLuint
}

impl TexturePolygon {

    pub fn new(sdl_context: &mut SDLContext, point: glm::Vec3, texture_name: String) -> Self {
        let vertex_array = Self::init_vertex_array(point);
        let normal_array = Self::init_normal_array();
        let texture_rect: glm::Vec4 = sdl_context.terrain_texture.get_rect_from_texture_image(texture_name.to_string());
        let texture_array = Self::init_texture_array(texture_rect.x, texture_rect.y, texture_rect.z, texture_rect.w);
        return TexturePolygon {
            position: point,
            vertex_array: vertex_array,
            normal_array: normal_array,
            texture_array: texture_array,
            texture_buffer: 0,
            texture_id: sdl_context.terrain_texture.texture_id
        }
    }

    pub fn get_model(&mut self) -> glm::Mat4 { 
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, &self.position);
    }

    pub fn clean_up(&mut self) {

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

    fn init_texture_array(low_x: GLfloat, low_y: GLfloat, high_x: GLfloat, high_y: GLfloat) -> Vec<GLfloat> {
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