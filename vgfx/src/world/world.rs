use crate::Model;
use crate::AreaInstance;
use crate::ModelInstance;
use crate::dep::events::SDLContext;
use crate::Texture;
use crate::geo::texture_polygon::TexturePolygon;
use crate::gfx::face::FacePartitionRender;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::{self, File, DirEntry};
use std::io;
extern crate nalgebra_glm as glm;
use std::convert::TryInto;

pub struct World {
    base_folder: String,
    model_map: HashMap<String, Model>,
    areas: Vec<AreaInstance>
}

impl World {

    pub fn new() -> Self {
        let mut world = World {
            base_folder: "res".to_string(),
            model_map: HashMap::new(),
            areas: Vec::new()
        };
        return world;
    }

    pub fn new_load(sdl_context: &mut SDLContext) -> Self {
        let mut world = World {
            base_folder: "res".to_string(),
            model_map: HashMap::new(),
            areas: Vec::new()
        };
        return world.load(sdl_context, "res".to_string()).unwrap();
    }

    pub fn load(&mut self, sdl_context: &mut SDLContext, base_folder: String) -> io::Result<World> {
        let mut world = World::new();
        let world_file = [base_folder.to_string(),"/world.pak".to_string()].join("");
        let mut file = File::open(world_file)?;
        let mut buffer = vec![];
        let mut pos: usize = 0;
        file.read_to_end(&mut buffer);

        // 1. Count of Areas
        let num_areas = read_usize(&buffer, &mut pos);
        for i in 0..num_areas {
            let mut cur_model_instance = vec![];
            // 2. Count of Area Model Instances
            let num_model_instances = read_usize(&buffer, &mut pos);
            for j in 0..num_model_instances {
                // 3. Area's Model Instance Name
                let model_instance_name = read_str(&buffer, &mut pos);
                // 4. Area's Model Instance Position
                let model_instance_pos = read_vec3(&buffer, &mut pos);
                cur_model_instance.push(ModelInstance{ 
                    model_name: model_instance_name,
                    position: model_instance_pos,
                    face_partitions: vec![]
                });
            }
            world.areas.push(AreaInstance { model_instances: cur_model_instance });
        }
        // 5. Count of Model Hash Map
        let hash_map_cnt = read_usize(&buffer, &mut pos);
        for i in 0..hash_map_cnt {

            let mut vertices: Vec<Vec<f32>> = vec![];
            let mut texture_maps: Vec<Vec<f32>> = vec![];
            let mut normals: Vec<Vec<f32>> = vec![];

            // 6. Model Hash Map Name
            let model_name = read_str(&buffer, &mut pos);
            let mut cur_model = Model::new(model_name);
            // 7. Count of Texture Info
            let texture_cnt = read_usize(&buffer, &mut pos);
            for j in 0..texture_cnt {
                // 8. Texutre Info Name
                let texture_name = read_str(&buffer, &mut pos);
                let mut cur_texture = Texture::new(texture_name);
                // 9. Texture Info Ambient Color
                cur_texture.texture_properties.ambient_color = read_vec3(&buffer, &mut pos);
                // 10. Texture Info Diffuse Color
                cur_texture.texture_properties.diffuse_color = read_vec3(&buffer, &mut pos);
                // 11. Texture Info Specular Color
                cur_texture.texture_properties.specular_color = read_vec3(&buffer, &mut pos);
                // 12. Texture Info Emissive Coeficient
                cur_texture.texture_properties.emissive_coeficient = read_vec3(&buffer, &mut pos);
                // 13. Texture Info Transmission FIlter
                cur_texture.texture_properties.transmission_filter = read_vec3(&buffer, &mut pos);
                // 14. Texture Info Optical Desntiy
                cur_texture.texture_properties.optical_density = read_f32(&buffer, &mut pos);
                // 15. Texture Info Dissolve
                cur_texture.texture_properties.dissolve = read_f32(&buffer, &mut pos);
                // 16. Texture Info Specular Highlights
                cur_texture.texture_properties.specular_highlights = read_f32(&buffer, &mut pos);
                // 17. Texture Info Illum
                cur_texture.texture_properties.illum = read_i32(&buffer, &mut pos);
                // 18. Texture Info Image Size
                let img_size = read_usize(&buffer, &mut pos);
                // 19. Texture Image Byte Data (If Image Exist)
                if img_size > 0 {
                    let img_bytes = read_to_array(&buffer, pos, img_size);
                    pos += img_size;
                    cur_texture.createTextureBufferFromByteData(&img_bytes);
                }
                cur_model.textures.push(cur_texture);
            }
            // 20. Count of Vertices
            let vertices_cnt = read_usize(&buffer, &mut pos);
            for i in 0..vertices_cnt {
                // 21. Vertices
                vertices.push(read_vec3(&buffer, &mut pos));
            }
            // 22. Count of Texture Mappings
            let texture_maps_cnt = read_usize(&buffer, &mut pos);
            for i in 0..texture_maps_cnt {
                // 23. Texture Mappings
                texture_maps.push(read_vec2(&buffer, &mut pos));
            }
            // 24. Count of Normals
            let normals_cnt = read_usize(&buffer, &mut pos);
            for i in 0..normals_cnt {
                // 25. Normals
                normals.push(read_vec3(&buffer, &mut pos));
            }
            // 26. Count of Face Partitions
            let face_partitions_cnt = read_usize(&buffer, &mut pos);
            for i in 0..face_partitions_cnt {
                // 27. Count of Faces
                let faces_cnt = read_usize(&buffer, &mut pos);
                let mut texture_buffer = vec![];
                let mut normal_buffer = vec![];
                let mut vertex_buffer = vec![];
                // 28. Texture Info Index
                let texture_info_index = read_usize(&buffer, &mut pos);
                for j in 0..faces_cnt {
                    for k in 0..3 {
                        // 29. Face Texture Vertex Index
                        let texture_vertex_index = read_usize(&buffer, &mut pos);
                        // 30. Face Texture Map Index
                        let texture_map_index = read_usize(&buffer, &mut pos);
                        // 31. Face Texture Normals Index
                        let texture_normals_index = read_usize(&buffer, &mut pos);
                        
                        texture_buffer = texture_buffer.iter().chain(&texture_maps[texture_map_index]).map(|&x|x).collect::<Vec<f32>>();
                        normal_buffer = normal_buffer.iter().chain(&normals[texture_normals_index]).map(|&x|x).collect::<Vec<f32>>();
                        vertex_buffer = vertex_buffer.iter().chain(&vertices[texture_vertex_index]).map(|&x|x).collect::<Vec<f32>>();
                    }
                }
                cur_model.face_partitions.push(FacePartitionRender::new(
                    vertex_buffer, normal_buffer, texture_buffer,
                    texture_info_index, faces_cnt as i32
                ));
            }
            world.model_map.insert(cur_model.name.to_string(), cur_model);
        }
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