use std::io::prelude::*;
use std::fs::{File};
use std::convert::TryInto;
use std::io;

use nalgebra_glm::floor;


pub struct Map {
    floors: Vec<Floor>
}

pub struct Room {
    id: u8,
    size: u8,
    adj_rooms: Vec<u8>,
    room_type: u8
}

pub struct Floor {
    rooms: Vec<Room>,
    blocks: Vec<Vec<u8>>,
    max_blocks: u8
}

impl Map {
    
    pub fn new() -> Self {
        return Map {
            floors: vec![]
        }
    }

    pub fn new_load() -> Self {
        let mut map = Map {
            floors: vec![]
        };
        map.load("res".to_string());
        return map;
    }

    fn load(&mut self, base_folder: String) -> io::Result<Map> {
        let map_file = [base_folder.to_string(), "/map.bin".to_string()].join("");
        let mut file = File::open(map_file)?;
        let mut buffer = vec![];
        let mut pos: usize = 0;
        let mut map: Map = Map {
            floors: vec![]
        };
        file.read_to_end(&mut buffer);
    
        for i in 0..3 {
            let mut cur_floor = Floor {
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
                let mut cur_room = Room {
                    id: 0,
                    size: 0,
                    adj_rooms: vec![],
                    room_type: 0
                };
                // ID
                cur_room.id = read_u8(&buffer, &mut pos);
                cur_room.room_type = read_u8(&buffer, &mut pos);
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