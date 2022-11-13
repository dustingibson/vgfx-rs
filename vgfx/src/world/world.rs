

use crate::Model;
use crate::ModelInstance;
use crate::dep::events::SDLContext;
use crate::Texture;
use crate::gfx::face::FacePartitionRender;
use crate::gfx::shader::Shader;
use crate::gfx::skybox::Skybox;
use crate::gfx::texture_group::TextureGroupRenderer;
use crate::model::model::AreaInstance;
use crate::utils::octo::OctTree;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::{File};
use std::io;
use crate::Camera;
extern crate nalgebra_glm as glm;
use std::convert::TryInto;
use std::time::Instant;


pub struct World {
    base_folder: String,
    pub model_map: HashMap<String, Model>,
    model_instances: Vec<ModelInstance>,
    pub oct_tree: OctTree<ModelInstance>,
    texture_group: HashMap<String, TextureGroupRenderer>,
    skyboxes: Vec<Skybox>
}

impl World {

    pub fn new() -> Self {
        let world = World {
            base_folder: "res".to_string(),
            model_map: HashMap::new(),
            model_instances: vec![],
            oct_tree: OctTree::new(),
            texture_group: HashMap::new(),
            skyboxes: vec![]
        };
        return world;
    }

    pub fn new_load(sdl_context: &mut SDLContext) -> Self {
        let mut world = World {
            base_folder: "res".to_string(),
            model_map: HashMap::new(),
            model_instances: vec![],
            oct_tree: OctTree::new(),
            texture_group: HashMap::new(),
            skyboxes: vec![]
        };
        return world.load(sdl_context, "res".to_string()).unwrap();
    }

    pub fn draw_skybox(&mut self, shader: &mut Shader) {
        self.skyboxes[0].draw(shader);
    }

    pub fn draw(&mut self, shader: &mut Shader, camera: &mut Camera) {
        let range: f32 = 5000.0;
        let mut cur_instances: Vec<Box<ModelInstance>> = vec![];
        //TODO: Make values relative to camera
        self.oct_tree.get_items_from_range(&mut cur_instances, camera.position.x - range, camera.position.y - range, camera.position.z - range, camera.position.x + range, camera.position.y + range, camera.position.z + range);
        //self.oct_tree.get_all_items(& mut cur_instances);
        let now = Instant::now();

        for model_instance in cur_instances.iter_mut() {
            
            //UNCOMMENT TO DRAW
            let model = self.model_map.get_mut(& mut model_instance.model_name.to_string()).unwrap();
            //model.draw(shader, &mut glm::Vec3::new(model_instance.position[0], model_instance.position[1], model_instance.position[2]), false);
            
            // TODO: Refactor! Moving instance references in and out therefore needs to be reinserted.
            self.oct_tree.insert_item(model_instance.clone(), model_instance.position[0], model_instance.position[1], model_instance.position[2]);
        }



        // for (key, value) in self.texture_group.iter_mut() {
        //     value.draw(shader, &mut glm::Vec3::new(1.0, 1.0, 1.0));
        // }

        let elapsed = now.elapsed();
        //println!("Elapsed 1: {:.2?}", elapsed);
    }
    
    pub fn clean_up(&mut self) {
        // let mut all_instances: Vec<ModelInstance> = vec![];
        // self.oct_tree.get_all_items(&mut all_instances);
        // for model_instance in all_instances.iter_mut() {
        //     let mut model = self.model_map.get_mut(& mut model_instance.model_name.to_string()).unwrap();
        //     model.clean_up();
        //     model_instance.clean_up();
        // }
    }

    pub fn add_partition(&mut self, model_instances: &mut OctTree<ModelInstance>, model_name: String, face_partitions: Vec<FacePartitionRender>) {

        self.model_map.get_mut(&model_name.to_string()).unwrap().face_partitions = face_partitions;
        // let mut all_instances: Vec<Box<ModelInstance>> = vec![];
        // model_instances.get_all_items( &mut all_instances);
        // for model_instance in all_instances.iter_mut() {
        //     if model_instance.model_name == model_name {
        //         model_instance.face_partitions = face_partitions.to_owned();
        //         self.scale_vec(& mut model_instance.face_partitions, model_instance.scale);
        //     }
        //     model_instances.insert_item(model_instance.to_owned(), model_instance.position[0], model_instance.position[1], model_instance.position[2])
        // }
    }

    pub fn scale_vec(&mut self, partitions: &mut Vec<FacePartitionRender>, scale: f32) {
        // for partition in partitions.iter_mut() {
        //     for vertex in partition.vertex_buffer.iter_mut() {
        //         *vertex = scale * *vertex;
        //     }
        //     partition.initGL();
        // }
    }

    pub fn vec_to_vec3(&mut self, f32_vec: Vec<f32>) -> glm::Vec3 {
        return glm::vec3(f32_vec[0], f32_vec[1], f32_vec[2]);
    }

    pub fn load(&mut self, sdl_context: &mut SDLContext, base_folder: String) -> io::Result<World> {
        println!("Start Loading!");

        let mut world = World::new();
        let world_file = [base_folder.to_string(),"/world.pak".to_string()].join("");
        let mut file = File::open(world_file)?;
        let mut buffer = vec![];
        let mut pos: usize = 0;
        file.read_to_end(&mut buffer);

        // 1. Count of Areas
        let num_areas = read_usize(&buffer, &mut pos);
        for i in 0..num_areas {
            let cur_model_instance: Vec<ModelInstance> = vec![];
            // 2-3. Skybox Left Image
            let skybox_left = load_image(&buffer, &mut pos, "left".to_string()).unwrap();
            // 4-5. Skybox Right Image
            let skybox_right = load_image(&buffer, &mut pos, "right".to_string()).unwrap();
            // 6-7. Skybox Top Image
            let skybox_top = load_image(&buffer, &mut pos, "top".to_string()).unwrap();
            // 8-9. Skybox Bottom Image
            let skybox_bottom = load_image(&buffer, &mut pos, "bottom".to_string()).unwrap();
            // 10-11. Skybox Front Image Size            
            let skybox_front = load_image(&buffer, &mut pos, "front".to_string()).unwrap();
            // 12-13. Skybox Back Image
            let skybox_back = load_image(&buffer, &mut pos, "back".to_string()).unwrap();
            let mut skybox: Skybox = Skybox::new(skybox_left, skybox_right, skybox_top, skybox_bottom, skybox_front, skybox_back);
            world.skyboxes.push(skybox);
            // 14. Count of Area Model Instances
            let num_model_instances = read_usize(&buffer, &mut pos);
            for j in 0..num_model_instances {
                // 15. Area's Model Instance Name
                let model_instance_name = read_str(&buffer, &mut pos);
                // 16. Area's Model Instance Position
                let model_instance_pos = read_vec3(&buffer, &mut pos);
                // 17. Area's Model Instance Scale
                let model_instance_scale = read_f32(&buffer, &mut pos);
                let new_model_instance = ModelInstance{ 
                    model_name: model_instance_name.to_string(),
                    position: glm::Vec3::new(model_instance_pos[0], model_instance_pos[1], model_instance_pos[2]),
                    scale: model_instance_scale
                };
                //world.model_instances.push(new_model_instance);
                world.oct_tree.insert_item(Box::new(new_model_instance), model_instance_pos[0], model_instance_pos[1], model_instance_pos[2]);
            }
        }
        // 18. Count of Model Hash Map
        let hash_map_cnt = read_usize(&buffer, &mut pos);
        for i in 0..hash_map_cnt {

            let mut vertices: Vec<Vec<f32>> = vec![];
            let mut texture_maps: Vec<Vec<f32>> = vec![];
            let mut normals: Vec<Vec<f32>> = vec![];
            let mut mode: u8 = 0;

            // 19. Model Hash Map Name
            let model_name = read_str(&buffer, &mut pos);
            let mut cur_model = Model::new(model_name);
            // 20. Count of Texture Info
            let texture_cnt = read_usize(&buffer, &mut pos);
            for j in 0..texture_cnt {
                // 21. Texutre Info Name
                let texture_name = read_str(&buffer, &mut pos);
                let mut cur_texture = Texture::new(texture_name);
                // 22. Texture Info Ambient Color
                cur_texture.texture_properties.ambient_color = read_vec3(&buffer, &mut pos);
                // 23. Texture Info Diffuse Color
                cur_texture.texture_properties.diffuse_color = read_vec3(&buffer, &mut pos);
                // 24. Texture Info Specular Color
                cur_texture.texture_properties.specular_color = read_vec3(&buffer, &mut pos);
                // 25. Texture Info Emissive Coeficient
                cur_texture.texture_properties.emissive_coeficient = read_vec3(&buffer, &mut pos);
                // 26. Texture Info Transmission FIlter
                cur_texture.texture_properties.transmission_filter = read_vec3(&buffer, &mut pos);
                // 27. Texture Info Optical Desntiy
                cur_texture.texture_properties.optical_density = read_f32(&buffer, &mut pos);
                // 28. Texture Info Dissolve
                cur_texture.texture_properties.dissolve = read_f32(&buffer, &mut pos);
                // 29. Texture Info Specular Highlights
                cur_texture.texture_properties.specular_highlights = read_f32(&buffer, &mut pos);
                // 30. Texture Info Illum
                cur_texture.texture_properties.illum = read_i32(&buffer, &mut pos);
                // 31. Texture Info Image Size
                let img_size = read_usize(&buffer, &mut pos);
                // 32. Texture Image Byte Data (If Image Exist)
                if img_size > 0 {
                    let img_bytes = read_to_array(&buffer, pos, img_size);
                    pos += img_size;
                    cur_texture.create_texture_buffer_from_byte_data(&img_bytes);
                }
                let mode = if img_size > 0  { 3 } else { 2 };
                cur_model.textures.push(cur_texture.clone());
                world.texture_group.insert(cur_texture.name.to_string(), TextureGroupRenderer::new(cur_texture) );
            }
            // 33. Count of Vertices
            let vertices_cnt = read_usize(&buffer, &mut pos);
            for i in 0..vertices_cnt {
                // 34. Vertices
                let vert = read_vec3(&buffer, &mut pos);
                vertices.push(vert);
            }
            // 35. Count of Texture Mappings
            let texture_maps_cnt = read_usize(&buffer, &mut pos);
            for i in 0..texture_maps_cnt {
                // 36. Texture Mappings
                texture_maps.push(read_vec2(&buffer, &mut pos));
            }
            // 37. Count of Normals
            let normals_cnt = read_usize(&buffer, &mut pos);
            for i in 0..normals_cnt {
                // 38. Normals
                normals.push(read_vec3(&buffer, &mut pos));
            }
            // 39. Count of Face Partitions
            let face_partitions_cnt = read_usize(&buffer, &mut pos);
            let mut tmp_face_partitions = vec![];
            for i in 0..face_partitions_cnt {
                // 40. Count of Faces
                let faces_cnt = read_usize(&buffer, &mut pos);
                let mut texture_buffer = vec![];
                let mut normal_buffer = vec![];
                let mut vertex_buffer = vec![];
                // 41. Texture Info Index
                let texture_info_index = read_usize(&buffer, &mut pos);
                for j in 0..faces_cnt {
                    for k in 0..3 {
                        // 42. Face Mode
                        mode = read_u8(&buffer, &mut pos);
                        // 43. Face Texture Vertex Index
                        let texture_vertex_index = read_usize(&buffer, &mut pos);
                        // 44. Face Texture Map Index
                        let texture_map_index = read_usize(&buffer, &mut pos);
                        // 45. Face Texture Normals Index (if applicable)
                        if mode == 3 {
                            let texture_normals_index = read_usize(&buffer, &mut pos);
                            normal_buffer = normal_buffer.iter().chain(&normals[texture_normals_index]).map(|&x|x).collect::<Vec<f32>>();
                        }
                        texture_buffer = texture_buffer.iter().chain(&texture_maps[texture_map_index]).map(|&x|x).collect::<Vec<f32>>();
                        vertex_buffer = vertex_buffer.iter().chain(&vertices[texture_vertex_index]).map(|&x|x).collect::<Vec<f32>>();
                    }
                }
                let face_partition = FacePartitionRender::new(
                    vertex_buffer, normal_buffer, texture_buffer,
                    texture_info_index, faces_cnt as i32, mode,
                    true
                );
                tmp_face_partitions.push(face_partition);
                // cur_model.face_partitions.push(FacePartitionRender::new(
                //     vertex_buffer, normal_buffer, texture_buffer,
                //     texture_info_index, faces_cnt as i32, mode
                // ));
            }
            // 46. Boundary 1
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 47. Boundary 2
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 48. Boundary 3
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 49. Boundary 4
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 50. Boundary 5
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 51. Boundary 6
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 52. Boundary 7
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));
            // 53. Boundary 8
            cur_model.boundary_points.push(self.vec_to_vec3(read_vec3(&buffer, &mut pos)));     
            cur_model.setup();
            //self.add_partition2( &mut world.model_instances, cur_model.name.to_string(), tmp_face_partitions);
            cur_model.face_partitions = tmp_face_partitions;
            world.model_map.insert(cur_model.name.to_string(), cur_model);
            //self.add_partition( &mut world.oct_tree, cur_model.name.to_string(), tmp_face_partitions);
        }
        println!("Completed Loading!");
        Ok(world)
    }
}

fn to_vec2(vert: Vec<f32>) -> glm::Vec2 {
    return glm::Vec2::new(vert[0], vert[1]);
}


fn to_vec3(vert: Vec<f32>) -> glm::Vec3 {
    return glm::Vec3::new(vert[0], vert[1], vert[2]);
}

fn read(data: &Vec<u8>, pos: usize, size: usize) -> Vec<u8> {
    let buffer_slice = &data[pos..pos+size];
    return buffer_slice.to_vec();
}

fn read_to_array(data: &Vec<u8>, pos: usize, size: usize) -> &[u8] {
    let buffer_slice = &data[pos..pos+size];
    return buffer_slice;
}

fn read_u8(data: &Vec<u8>, pos: &mut usize) -> u8 {
    let buffer_slice = read(data, *pos, 1);
    *pos = *pos + 1;
    return u8::from_be_bytes(buffer_slice.try_into().unwrap());
}

fn read_f32(data: &Vec<u8>, pos: &mut usize) -> f32 {
    let buffer_slice = read(data, *pos, 4);
    *pos = *pos + 4;
    return f32::from_be_bytes(buffer_slice.try_into().unwrap());
}

fn read_i32(data: &Vec<u8>, pos: &mut usize) -> i32 {
    let buffer_slice = read(data, *pos, 4);
    *pos = *pos + 4;
    return i32::from_be_bytes(buffer_slice.try_into().unwrap());
}

fn read_usize(data: &Vec<u8>, pos: &mut usize) -> usize {
    let buffer_slice = read(data, *pos, 8);
    *pos = *pos + 8;
    return usize::from_be_bytes(buffer_slice.try_into().unwrap());
}

fn read_str(data: &Vec<u8>, pos: &mut usize) -> String {
    let str_usize = read_usize(data, pos);
    let buffer_slice = read(data, *pos, str_usize);
    *pos = *pos + str_usize;
    return std::str::from_utf8(&buffer_slice).unwrap().to_string();
}

fn read_vec(data: &Vec<u8>, length: usize, pos: &mut usize) -> Vec<f32> {
    let mut out: Vec<f32> = Vec::new();
    for i in 0..length {
        out.push(read_f32(data, pos));
    }
    return out;
}

fn read_vec2(data: &Vec<u8>, pos: &mut usize) -> Vec<f32> {
    return read_vec(data, 2, pos);
}

fn read_vec3(data: &Vec<u8>, pos: &mut usize) -> Vec<f32> {
    return read_vec(data, 3, pos);
}

fn load_image(data: &Vec<u8>, pos: &mut usize, texture_name: String) -> Option<Vec<u8>> {
    let img_size = read_usize(data, pos);
    return match img_size > 0 {
        true => {
            let mut new_texture = Texture::new(texture_name);
            let img_bytes = read_to_array(data, pos.clone(), img_size);
            *pos = *pos + img_size;
            return Some(img_bytes.to_vec());
        },
        false => {
            None
        }
    }
}