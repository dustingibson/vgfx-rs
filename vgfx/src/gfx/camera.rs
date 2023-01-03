use gl;
use gl::types::*;
use crate::ShaderContainer;

use super::shader::Shader;
extern crate nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec3,
    pub projection: glm::Mat4,
    pub model: glm::Mat4,
    pub view: glm::Mat4,
    pub front: glm::Vec3,
    pub mvp: glm::Mat4,
    width: f32,
    height: f32
}

impl Camera {
    pub fn new(position: glm::Vec3, width: f32, height: f32) -> Camera {
            // Rad(45) = 0.785398
        let projection: glm::Mat4 = glm::perspective( width / height, 0.785398, 0.1, 10000.0);
        let view: glm::Mat4 = glm::look_at::<GLfloat>(
            &position,
            &glm::vec3(0.0, 0.0, 1.0),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
        let model: glm::Mat4 = glm::Mat4::identity();
        let mvp: glm::Mat4 = projection * view * model;
        return Camera {
            position: position,
            model: model,
            view: view,
            front: glm::vec3(0.0, 0.0, -1.0),
            projection: projection,
            mvp: mvp,
            width: width,
            height: height
        }
    }

    pub fn coord_str(&self) -> String {
        return format!("{:.4} {:.4} {:.4}", self.position.x, self.position.y, self.position.z);
    }

    pub fn coord_front_str(&self) -> String {
        return format!("{:.4} {:.4} {:.4}", self.front.x, self.front.y, self.front.z);
    }

    pub fn abs_camera_position(&self, m: f32) -> glm::Vec3 {
        return glm::vec3(self.position.x + self.front.x*m, self.position.y + self.front.y*m, self.position.z + self.front.z*m);
    }

    pub fn translate(& mut self, translate_vector: glm::Vec3, product: f32) {
        self.position += translate_vector * product;
        self.update();
    }


    pub fn set_projection_skybox(&mut self, shader_container: &mut ShaderContainer, name: &String) {
        shader_container.set_projection_skybox(name, self.get_view(), self.projection);
    }

    pub fn set_projection(&mut self, shader_container: &mut ShaderContainer, name: &String) {
        shader_container.set_projection(name, self.get_view(), self.projection);
    }

    pub fn set_projection_ortho(&mut self, shader_container: &mut ShaderContainer, name: &String) {
        let new_projection: glm::Mat4 = self.ortho(0.0, 1.0, 1.0, 0.0, -1.0, 1000.0);
        //let new_projection: glm::Mat4 = glm::ortho(0.0, self.width, 0.0, self.height, 0.1, 1000.0);
        let view: glm::Mat4 = glm::Mat4::identity();
        shader_container.set_projection(name, view, new_projection);
    }

    pub fn get_view(&self) -> glm::Mat4 {
        return glm::look_at::<GLfloat>(
            &self.position,
            &(self.position + self.front),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
    }

    pub fn change_yaw(&mut self, delta_x: f32) {
        let angle: f32 = (0.5 - delta_x) * 3.14 * 2.0;
        self.front.x = angle.cos();
        self.front.y = 0.0;
        self.front.z = angle.sin() * -1.0;
    }

    pub fn change_pitch(&mut self, delta_y: f32) {
        let angle: f32 = (0.5 - delta_y) * 3.14 * 2.0;
        self.front.x = angle.cos();
        self.front.y = angle.sin() * -1.0;
        self.front.z = angle.cos() * -1.0;
    }

    pub fn change_angle(&mut self, delta_x: f32, delta_y: f32) {
        let angle_x: f32 = (0.5 - delta_x) * 3.14 * 2.0;
        let angle_y: f32 = (0.5 - delta_y) * 3.14 * 2.0;
        self.front.x = angle_x.cos() * angle_y.cos();
        self.front.y = angle_y.sin()*-1.0;
        self.front.z = angle_x.sin() * angle_y.cos();
        self.front = glm::normalize(&self.front);
    }

    pub fn update(& mut self) {
        self.view = glm::look_at::<GLfloat>(
            &self.position,
            &self.front,
            &glm::vec3(0.0 ,1.0, 0.0)
        );
    }

    pub fn ortho(&mut self, left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> glm::Mat4 {
        return glm::mat4(
            2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
            0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
            0.0, 0.0, -2.0 / (z_far - z_near), -(z_far + z_near) / (z_far - z_near),
            0.0, 0.0, 0.0, 1.0
        );
    }

}