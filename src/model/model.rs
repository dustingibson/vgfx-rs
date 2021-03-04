use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
use crate::BFile;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::SubModel;
use crate::Shader;
#[derive(Clone)]

pub struct Model {
    pub sub_models: Vec<SubModel>,
    pub size: glm::Vec3
}

impl Model {
    pub fn new(size: glm::Vec3) -> Self {
        return Model {
            sub_models: vec![],
            size: size
        };
    }

    pub fn fromFile(size: glm::Vec3) -> Self {
        let mut model = Model {
            sub_models: vec![],
            size: size
        };


        //Number Properties (4 byes)
        //Size (4 bytes)
        //Data (n bytes)
        let mut cuboids: Vec<Cuboid> = Vec::new();
        let mut model_file: BFile = BFile::new("res/test.bin".to_string());
        let num_process: u32 = model_file.readu32();
        for i in 0..num_process {

            let mut geo: String = "".to_string();
            let mut model_type: String = "".to_string();
            let mut texture: String = "".to_string();
            let mut size: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
            let mut position: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);

            let b_name = model_file.readu32();

            // 0 - geo
            match b_name {
                0 => {
                    geo = model_file.autoReadString();
                },
                1 => {
                    model_type = model_file.autoReadString();
                },
                2 => {
                    texture = model_file.autoReadString();
                },
                3 => {
                    size = model_file.readvec3()
                },
                4 => {
                    position = model_file.readvec3();
                },
                _ => panic!("no type found")
            }
            println!("{}", model_type);
            println!("{}", texture);
            println!("{} {} {}", size.x, size.y, size.z);
            println!("{} {} {}", position.x, position.y, position.z);

            //point: glm::Vec3, color: glm::Vec4, texture_coord: glm::Vec4, length: GLfloat, width: GLfloat, height: GLfloat
            cuboids.push(Cuboid::new(position,  glm::vec4(1.0, 1.0, 1.0, 0.1), glm::vec4(0.0,0.0,1.0,1.0), size.x, size.y, size.z));
        }
        model.insert_submodel(glm::vec3(0.0,0.0, 0.0), size, &mut cuboids);
        return model;
    }

    pub fn insert_submodel(&mut self, position: glm::Vec3, size: glm::Vec3, cuboids: &mut Vec<Cuboid>) {
        self.sub_models.push(SubModel::new(position, size, cuboids));
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        for sub_model in self.sub_models.iter_mut() {
            sub_model.draw(shader);
        }
    }
}