

extern crate nalgebra_glm as glm;
use std::collections::HashMap;
use std::string;
use glm::Vec3;
use uuid::Uuid;

use crate::Shader;
use crate::geo::line::Line;
use crate::gfx::face::FacePartitionRender;
use crate::gfx::texture::Texture;

#[derive(Clone)]
pub struct ModelInstance {
    pub model_name: String,
    pub position: glm::Vec3,
    pub scale: glm::Vec3,
    pub rotate: glm::Vec3,
    pub name: String
}

pub struct AreaInstance {
    pub model_instances: Vec<ModelInstance>
}

// TODO: If needed optimize face partitions by model instead of instance
// Tradeoff cost of memory vs cost of processing streaming VBOs
pub struct Model {
    pub name: String,
    pub textures: Vec<Texture>,
    pub face_partitions: Vec<FacePartitionRender>,
    pub boundary_points: Vec<glm::Vec3>,
    pub boundary_lines: Vec<Line>
}

impl Model {
    pub fn new(name: String) -> Self {
        return Model {
            name: name.to_string(),
            textures: vec![],
            face_partitions: vec![],
            boundary_points: vec![],
            boundary_lines: vec![]
        };
    }

    pub fn setup(&mut self) {
        let left_up_front = self.boundary_points[0];
        let left_up_back = self.boundary_points[1];
        let right_down_back = self.boundary_points[2];
        let right_down_front = self.boundary_points[3];
        let left_down_back = self.boundary_points[4];
        let left_down_front = self.boundary_points[5];
        let right_up_back = self.boundary_points[6];
        let right_up_front = self.boundary_points[7];

        let color = glm::vec4(1.0, 0.0, 0.0, 1.0);
        let width = 5.0;

        // left_up_front connects to left_down_front (first square top)
        self.boundary_lines.push(Line::new(left_up_front, left_down_front, color, width));
        self.boundary_lines.push(Line::new(left_down_front, right_down_front, color, width));
        self.boundary_lines.push(Line::new(right_down_front, right_up_front, color, width));
        self.boundary_lines.push(Line::new(right_up_front, left_up_front, color, width));


        self.boundary_lines.push(Line::new(left_up_front, left_up_back, color, width));
        self.boundary_lines.push(Line::new(left_down_front, left_down_back, color, width));
        self.boundary_lines.push(Line::new(right_down_front, right_down_back, color, width));
        self.boundary_lines.push(Line::new(right_up_front, right_up_back, color, width));


        self.boundary_lines.push(Line::new(left_up_back, left_down_back, color, width));
        self.boundary_lines.push(Line::new(left_down_back, right_down_back, color, width));
        self.boundary_lines.push(Line::new(right_down_back, right_up_back, color, width));
        self.boundary_lines.push(Line::new(right_up_back, left_up_back, color, width));
    }

    pub fn draw_stencil(&self, shader: &mut Shader, position: &mut glm::Vec3, scale: &mut glm::Vec3, rotate: &mut glm::Vec3) {
        unsafe {
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);
        }
        let mut new_position = position.clone() + glm::vec3(10.0, 10.0, 10.0);
        self.draw(shader, &mut new_position, scale, rotate, false);
        unsafe {
            gl::StencilMask(0xFF);
            gl::StencilFunc(gl::ALWAYS, 0, 0xFF);
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn draw(& self, shader: &mut Shader, position: &mut glm::Vec3, scale: &mut glm::Vec3, rotate: &mut glm::Vec3, stencil: bool) {
        if stencil {
            unsafe {
                gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
                gl::StencilMask(0xFF);
            }
        }
        for face_partition in self.face_partitions.iter() {
            face_partition.draw(shader, position, &self.textures[face_partition.texture_index], scale, rotate);
        }
        // for line in self.boundary_lines.iter() {
        //     line.draw(shader, position);
        // }
        if stencil {
            unsafe {

            }
        }
    }

    pub fn clean_up(&mut self) {
        for face_partition in self.face_partitions.iter_mut() {
            face_partition.clean_up();
        }
    }
}

impl ModelInstance {
    pub fn new(name: String, position: glm::Vec3, scale: glm::Vec3, rotate: glm::Vec3) -> Self {
        return ModelInstance {
            model_name: name.to_string(),
            position: position,
            scale: scale,
            rotate: rotate,
            name: Uuid::new_v4().to_string()
        };
    }

    pub fn draw(&mut self, shader: &mut Shader, model_map: &HashMap<String, Model>, stencil: bool) {
        model_map.get(&self.model_name).unwrap().draw(shader, &mut self.position, &mut self.scale, &mut self.rotate, stencil);
    }

    pub fn draw_stencil(&mut self, shader: &mut Shader, model_map: &HashMap<String, Model>) {
        //model_map.get(&self.model_name).unwrap().draw_stencil(shader, &mut self.position);
    }
}