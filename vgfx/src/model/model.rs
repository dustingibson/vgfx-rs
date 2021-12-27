use gl;
use crate::BFile;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::ColorPolygon;
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
        let mut model_file: BFile = BFile::new("res/test.bin".to_string(), true);
        while !model_file.isEnd() {
            let num_process: u32 = model_file.readu32();
            let mut geo: String = "".to_string();
            let mut model_type: String = "".to_string();
            let mut texture: String = "".to_string();
            let mut size: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
            let mut position: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
            for i in 0..num_process {

                let b_name = model_file.readu32();

                // 0 - geo
                match b_name {
                    0 => {
                        geo = model_file.auto_read_string();
                    },
                    1 => {
                        model_type = model_file.auto_read_string();
                    },
                    2 => {
                        texture = model_file.auto_read_string();
                    },
                    3 => {
                        size = model_file.readvec3()
                    },
                    4 => {
                        position = model_file.readvec3();
                    },
                    _ => panic!("no type found")
                }
                //point: glm::Vec3, color: glm::Vec4, texture_coord: glm::Vec4, length: GLfloat, width: GLfloat, height: GLfloat
            }
            cuboids.push(Cuboid::new(position,  glm::vec4(1.0, 1.0, 1.0, 0.1), glm::vec4(0.0,0.0,1.0,1.0), size.x, size.y, size.z));
        }
        model.insert_submodel(glm::vec3(0.0,0.0, 0.0), size, &mut cuboids, &mut Vec::new());
        return model;
    }

    pub fn from_single_cuboid(&mut self, cuboid: &mut Cuboid) {
        let mut cuboid_vec: Vec<Cuboid> = Vec::new();
        cuboid_vec.push(cuboid.clone());
        self.sub_models.push(SubModel::new(cuboid.position, cuboid.size(), &mut cuboid_vec, &mut Vec::new()));
    }

    pub fn pos_from_cuboid(&mut self) -> glm::Vec3
    {
        return self.sub_models[0].cuboids[0].position;
    }

    pub fn insert_submodel(&mut self, position: glm::Vec3, size: glm::Vec3, cuboids: &mut Vec<Cuboid>, color_polygons: &mut Vec<ColorPolygon>) {
        self.sub_models.push(SubModel::new(position, size, cuboids, color_polygons));
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        for sub_model in self.sub_models.iter_mut() {
            sub_model.draw(shader);
        }
    }
}