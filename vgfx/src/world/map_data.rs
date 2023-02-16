use std::io::prelude::*;
use std::fs::{File};
use std::convert::TryInto;
use std::io;
use super::world::World;
use crate::model::floor::Floor;
use crate::model::model::ModelInstance;
use crate::model::wall::Wall;
use crate::model::ceiling::Ceiling;
use crate::gfx::camera::Camera;
use crate::utils::octo::OctTree;

extern crate nalgebra_glm as glm;



pub struct MapData {
    pub floors: Vec<FloorData>,
    pub floor_height: f32,
    pub sizes: Vec<f32>
}

pub struct RoomData {
    pub id: u8,
    pub size: u8,
    pub adj_rooms: Vec<u8>,
    pub room_type: u8,
    pub block_coord: Vec<u8>
}

pub struct FloorData {
    pub rooms: Vec<RoomData>,
    pub blocks: Vec<Vec<u8>>,
    pub max_blocks: u8
}

impl MapData {
    
    pub fn new() -> Self {
        let height_pixel = 150.0;
        let floor_size = 540.0;
        return MapData {
            floors: vec![],
            floor_height: height_pixel,
            sizes: vec![floor_size, floor_size*2.0, floor_size*3.0]
        }
    }

    pub fn new_load() -> Self {
        let mut map = MapData::new();
        map = map.load("res".to_string()).unwrap();
        println!("Floor Size: {}", map.floors.len());
        return map;
    }

    pub fn populate_world(&self, oct_tree: &mut OctTree<ModelInstance>, camera: &mut Camera) {
        let mut story = 0;
        let mut cur_height = 0.0;
        for floor in self.floors.iter() {
            story += 1;
            cur_height = story as f32 * self.floor_height;
            for room in floor.rooms.iter() {

                let cur_size = self.sizes[room.size as usize];

                let x_pos = (room.block_coord[0] as f32 * (self.sizes[1]) as f32) + cur_size;
                let z_pos = (room.block_coord[1] as f32 * (self.sizes[1]) as f32) + cur_size;

                let position_floor = glm::Vec3::new(x_pos, cur_height - self.floor_height, z_pos );
                let position_ceiling = glm::Vec3::new(x_pos, cur_height + self.floor_height, z_pos );
                let position_wall = glm::Vec3::new(x_pos, cur_height, z_pos);

                let mut floor_model = Floor::new();
                let mut ceiling_model = Ceiling::new();
                let mut wall_model = Wall::new();
                if (room.size == 0) {
                    floor_model.insert_texture(position_floor, "floor_small".to_string(), self.sizes[0], self.floor_height, camera, oct_tree);
                    wall_model.insert_textures(position_wall, "wall_small".to_string(), self.sizes[0], camera, oct_tree);
                    ceiling_model.insert_texture(position_ceiling, "ceiling_small".to_string(), self.sizes[0], self.floor_height, camera, oct_tree);
                } else if (room.size == 1) {
                    floor_model.insert_texture(position_floor, "floor_medium".to_string(), self.sizes[1], self.floor_height, camera, oct_tree);
                    wall_model.insert_textures(position_wall, "wall_medium".to_string(), self.sizes[1], camera, oct_tree);
                    ceiling_model.insert_texture(position_ceiling, "ceiling_medium".to_string(), self.sizes[1], self.floor_height, camera, oct_tree);
                } else if (room.size == 2) {
                    floor_model.insert_texture(position_floor, "floor_large".to_string(), self.sizes[2], self.floor_height, camera, oct_tree);
                    wall_model.insert_textures(position_wall, "wall_large".to_string(), self.sizes[2], camera, oct_tree);
                    ceiling_model.insert_texture(position_ceiling, "ceiling_large".to_string(), self.sizes[2], self.floor_height, camera, oct_tree);
                }
            }
            break;
        }
    }

    fn load(&mut self, base_folder: String) -> io::Result<MapData> {
        let map_file = [base_folder.to_string(), "/map.bin".to_string()].join("");
        let mut file = File::open(map_file)?;
        let mut buffer = vec![];
        let mut pos: usize = 0;
        let mut map: MapData = MapData::new();
        file.read_to_end(&mut buffer);
    
        for i in 0..3 {
            let mut cur_floor = FloorData {
                rooms: vec![],
                blocks: vec![],
                max_blocks: 0
            };
            // Number of Blocks
            cur_floor.max_blocks = read_u8(&buffer, &mut pos);
            for m in 0..cur_floor.max_blocks {
                let mut temp_blocks = vec![];
                for n in 0..cur_floor.max_blocks {
                    // Block
                    temp_blocks.push( read_u8(&buffer, &mut pos) );
                }
                cur_floor.blocks.push(temp_blocks);
            }
            // Number of Rooms
            let num_rooms = read_u8(&buffer, &mut pos);
            for m in 0..num_rooms {
                let mut cur_room = RoomData {
                    id: 0,
                    size: 0,
                    adj_rooms: vec![],
                    room_type: 0,
                    block_coord: vec![0, 0]
                };
                // ID
                cur_room.id = read_u8(&buffer, &mut pos);
                // Room Type
                cur_room.room_type = read_u8(&buffer, &mut pos);
                // Room Size
                cur_room.size = read_u8(&buffer, &mut pos);
                // Start X
                cur_room.block_coord[0] = read_u8(&buffer, &mut pos);
                // Start Y
                cur_room.block_coord[1] = read_u8(&buffer, &mut pos);
                for n in 0..4 {
                    cur_room.adj_rooms.push(read_u8(&buffer, &mut pos));
                }
                cur_floor.rooms.push(cur_room);
            }
            map.floors.push(cur_floor);
        }
        Ok(map)
    }
}



fn read(data: &Vec<u8>, pos: usize, size: usize) -> Vec<u8> {
    let buffer_slice = &data[pos..pos+size];
    return buffer_slice.to_vec();
}

fn read_u8(data: &Vec<u8>, pos: &mut usize) -> u8 {
    let buffer_slice = read(data, *pos, 1);
    *pos = *pos + 1;
    return u8::from_be_bytes(buffer_slice.try_into().unwrap());
}