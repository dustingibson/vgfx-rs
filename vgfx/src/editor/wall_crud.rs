use std::collections::HashMap;
use gl;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;
use crate::model::model::ModelInstance;
use crate::World;
use crate::model::wall::Wall;
use uuid::Uuid;

pub struct WallCrud {
    pub texture_cursors: Vec<Option<ModelInstance>>,
    pub model_index: i32,
    pub prev_model_ids: Vec<String>,
    pub size: f32,
    pub wall: Wall
}

impl WallCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let mut texture_crud = WallCrud {
            texture_cursors: vec![None, None, None, None],
            model_index: 0,
            prev_model_ids: vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            size: 360.0,
            wall: Wall::new()
        };
        return texture_crud;
    }

    fn model_map_to_index(&mut self, model_map: &HashMap<String, Model>, index: u32) -> String {
        let mut cur_index: u32 = 0;
        for (key, model) in model_map.into_iter() {
            if index == cur_index {
                return model.name.to_string();
            }
            cur_index = cur_index + 1;
        }
        return "".to_string();
    }

    pub fn contains_wall_texture(&mut self, model_map: &HashMap<String, Model>) -> bool {
        let mut has_wall = false;
        model_map.keys().for_each( |k| {
            if k.contains("wall") {
                has_wall = true;
            }
        });
        return has_wall;
    }

    pub fn next_or_prev_texture_name(&mut self, model_map: &HashMap<String, Model>, direction: i32) -> String {
        while self.contains_wall_texture(model_map) {
            if direction > 0 {
                if (self.model_index + 1 >= model_map.len() as i32) { self.model_index = 0; } else { self.model_index += direction; }
            } else {
                if (self.model_index < 0) { self.model_index = model_map.len() as i32 - 1; } else { self.model_index += direction; }
            }
            let index_name = self.model_map_to_index(model_map, self.model_index as u32);
            if index_name.contains("wall") {
                return index_name;
            }
        }
        return "".to_string();
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, world: &mut World) {
        if (sdl_context.check_pressed("O".to_string())) {
            let model_name = self.next_or_prev_texture_name(&world.model_map, 1);
            self.wall.remove_textures(world);
            println!("Name {}, Size {}", model_name, self.size);
            self.wall.insert_textures(model_name.to_string(), self.size, camera, world);
            // self.next_or_prev_texture(camera, &world.model_map, 1);
            // for i in 0..4 {
            //     world.oct_tree.remove_item_by_name(self.prev_model_ids.get(i).unwrap().to_string());
            //     world.oct_tree.insert_item(Box::new(self.texture_cursors.get(i).unwrap().clone().unwrap()), 0.0, 0.0, 0.0);
            // }
        }
        if (sdl_context.check_pressed("P".to_string())) {
            let model_name = self.next_or_prev_texture_name(&world.model_map, -1);
            self.wall.remove_textures(world);
            self.wall.insert_textures(model_name.to_string(), self.size, camera, world);
            // self.next_or_prev_texture(camera, &world.model_map, -1);
            // for i in 0..4 {
            //     world.oct_tree.remove_item_by_name(self.prev_model_ids.get(i).unwrap().to_string());
            //     world.oct_tree.insert_item(Box::new(self.texture_cursors.get(i).unwrap().clone().unwrap()), 0.0, 0.0, 0.0);
            // }
        }
    }
}