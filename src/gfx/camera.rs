use gl;
use gl::types::*;
extern crate nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec3, 
    pub MVP: glm::Mat4
}

impl Camera {
    pub fn new(position: glm::Vec3) -> Camera {
            // Rad(45) = 0.785398
        let projection: glm::Mat4 = glm::perspective(4.0 / 3.0, 0.785398, 0.1, 100.0);
        let view: glm::Mat4 = glm::look_at::<GLfloat>(
            &position,
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
        let model: glm::Mat4 = glm::Mat4::identity();
        let MVP: glm::Mat4 = projection * view * model;
        return Camera {
            position: position,
            MVP: MVP
        }
    }

    pub fn translate(& mut self, translate_vector: glm::Vec3) {
        self.position += translate_vector;
        let projection: glm::Mat4 = glm::perspective(4.0 / 3.0, 0.785398, 0.1, 100.0);
        let view: glm::Mat4 = glm::look_at::<GLfloat>(
            &self.position,
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
        let model: glm::Mat4 = glm::Mat4::identity();
        self.MVP = projection * view * model;
    }

}