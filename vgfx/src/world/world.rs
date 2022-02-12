use crate::Model;
use crate::AreaInstance;
use crate::ModelInstance;
use crate::dep::events::SDLContext;
use crate::geo::texture_polygon::TexturePolygon;
use crate::model::model::ModelComponent;
use crate::model::model::SubModelComponent;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::{self, File, DirEntry};
use std::io;
extern crate nalgebra_glm as glm;
use std::convert::TryInto;

pub struct World {
    base_folder: String,
    model_map: HashMap<String, ModelComponent>,
    models: Vec<ModelInstance>,
    areas: Vec<AreaInstance>
}

impl World {

    pub fn new() -> Self {
        let mut world = World {
            base_folder: "res".to_string(),
            model_map: HashMap::new(),
            models: Vec::new(),
            areas: Vec::new()
        };
        return world;
    }

    pub fn new_load(sdl_context: &mut SDLContext) -> Self {
        let mut world = World {
            base_folder: "res".to_string(),
            model_map: HashMap::new(),
            models: Vec::new(),
            areas: Vec::new()
        };
        return world.load(sdl_context, "res".to_string()).unwrap();
    }

    pub fn load(&mut self, sdl_context: &mut SDLContext ,base_folder: String) -> io::Result<World> {
        let mut world = World::new();
        let world_file = [base_folder.to_string(),"/world.pak".to_string()].join("");
        let mut file = File::open(world_file)?;
        let mut buffer = vec![];
        let mut pos: usize = 0;
        file.read_to_end(&mut buffer);

        // 1. Count of Areas
        let num_areas = read_usize(&buffer, &mut pos);
        for i in 0..num_areas {
            // 2. Count of Area Texture Polygons
            let num_text_poly = read_usize(&buffer, &mut pos);
            let mut texture_polys = Vec::new();
            for  j in 0..num_text_poly {
                // 3. Area's Texture Polygon Texture Name
                let texture_name = read_str(&buffer, &mut pos);
                // 4. Area's Texture Polygon Texture Vertices
                let poly_vert = read_vec(&buffer, 3, &mut pos);
                texture_polys.push(TexturePolygon::new(sdl_context, to_vec3(poly_vert), texture_name));
            }
            // 5. Count of Area Model Instances
            let num_model_instances = read_usize(&buffer, &mut pos);
            let mut model_inst = Vec::new();
            for j in 0..num_model_instances {
                // 6. Area's Model Instance Name
                let model_name = read_str(&buffer, &mut pos);
                // 7. Area's Model Instance Position
                let model_pos = read_vec(&buffer, 3, &mut pos);
                model_inst.push(ModelInstance {
                    model_name: model_name,
                    position: model_pos
                });
            }
            world.areas.push(AreaInstance {
                texture_polygons: texture_polys,
                model_instances: model_inst
            });
        }  
        // 8. Count of Model Hash Map
        let num_hash_map = read_usize(&buffer, &mut pos);
        for i in 0..num_hash_map {
            // 9. Model Hash Map Name
            let model_key = read_str(&buffer, &mut pos);
            // 10. Count of Model Hash Map Submodel
            let num_submodel = read_usize(&buffer, &mut pos);
            let mut submodels = Vec::new();
            for j in 0..num_submodel {
                // 11. Model Hash Map Submodel Name
                let submodel_name = read_str(&buffer, &mut pos);
                // 12. Model Hash Map Texture Polygon Count
                let num_poly_count = read_usize(&buffer, &mut pos);
                let mut texture_polygons = Vec::new();
                for k in 0..num_poly_count {
                    // 13. Model Hash Map Texture Polygon Name
                    let model_poly_name = read_str(&buffer, &mut pos);
                    // 14. Model Hash Map Texture Polygon Vertices
                    let poly_vert = read_vec(&buffer, 3, &mut pos);
                    texture_polygons.push(TexturePolygon::new(sdl_context, to_vec3(poly_vert), model_poly_name));
                }
                submodels.push(SubModelComponent {
                    name: submodel_name,
                    texture_polygons: texture_polygons
                });
            }
            let model_key_copy  = model_key.clone();
            world.model_map.insert(model_key, ModelComponent { name: model_key_copy, sub_models: submodels } );
        }
        Ok(world)
    }
}

fn to_vec3(vert: Vec<f32>) -> glm::Vec3 {
    return glm::Vec3::new(vert[0], vert[1], vert[2]);
}

fn read(data: &Vec<u8>, pos: usize, size: usize) -> Vec<u8> {
    let buffer_slice = &data[pos..pos+size];
    return buffer_slice.to_vec();
}

fn read_f32(data: &Vec<u8>, pos: &mut usize) -> f32 {
    let buffer_slice = read(data, *pos, 4);
    *pos = *pos + 4;
    return f32::from_be_bytes(buffer_slice.try_into().unwrap());
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