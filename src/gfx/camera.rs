use gl;
use gl::types::*;
extern crate nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec3,
    pub projection: glm::Mat4,
    pub model: glm::Mat4,
    pub view: glm::Mat4,
    pub MVP: glm::Mat4
}

impl Camera {
    pub fn new(position: glm::Vec3) -> Camera {
            // Rad(45) = 0.785398
        let projection: glm::Mat4 = glm::perspective(4.0 / 3.0, 0.785398, 0.1, 100.0);
        let view: glm::Mat4 = glm::look_at::<GLfloat>(
            &position,
            &glm::vec3(0.0, 0.0, 2.0),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
        let model: glm::Mat4 = glm::Mat4::identity();
        let MVP: glm::Mat4 = projection * view * model;
        return Camera {
            position: position,
            model: model,
            view: view,
            projection: projection,
            MVP: MVP
        }
    }

    pub fn translate(& mut self, translate_vector: glm::Vec3) {
        self.position += translate_vector;
        println!("{}", self.position);
        self.update();
    }

    pub fn get_model(& mut self, pos: glm::Vec3) -> glm::Mat4 {
        let c_model: glm::Mat4 = glm::Mat4::identity();
        return glm::translate(&c_model, &pos);
    }

    pub fn get_view(&self) -> glm::Mat4 {
        return glm::look_at::<GLfloat>(
            &self.position,
            &(self.position + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
    }

    pub fn update(& mut self) {
        self.view = glm::look_at::<GLfloat>(
            &self.position,
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0 ,1.0, 0.0)
        );
    }

}