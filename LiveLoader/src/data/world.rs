use crate::Model;
use crate::AreaInstance;
use crate::ModelInstance;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::{self, File, DirEntry};
use std::io;
use std::str::Bytes;
use serde_json::{Result, Value};


pub struct World {
    base_folder: String,
    model_map: HashMap<String, Model>,
    models: Vec<ModelInstance>,
    areas: Vec<AreaInstance>
}

impl World {
    pub fn new(base_folder: String) -> Self {
        Self::create_if_exists(base_folder.to_string());
        return World {
            base_folder: base_folder,
            model_map: HashMap::new(),
            models: Vec::new(),
            areas: Vec::new()
        }
    }

    pub fn get_paths(&mut self, sub_dir: String) -> io::Result<Vec<String>> {
        let mut paths: Vec<String> = vec![];
        for entry in fs::read_dir(self.base_folder.to_string() + "/" + &sub_dir.to_string())? {
            let entry = entry?;
            let path = entry.path();
            match path.file_name() {
                Some(fname) => {
                    match fname.to_str() {
                        Some(cont_fname) => {
                            if cont_fname.contains(".json") {
                                paths.push(self.base_folder.to_string() + "/" + &sub_dir + "/" + &cont_fname.to_string());
                            }
                        },
                        None => {}
                    };
                },
                None => {}
            };
            println!("{:?}", path.file_name().ok_or("No file"));
        }
        Ok((paths))
    }

    pub fn set_areas(&mut self, paths: Vec<String>) {
        for path in paths.iter() {
            match fs::read_to_string(path) {
                Ok(res) => { 
                    //serde_json::from_str(&res).unwrap();
                    let area: AreaInstance = match serde_json::from_str(&res) {
                        Ok(areaInstance) => areaInstance,
                        Err(err) => AreaInstance { color_polygons: Vec::new(), texture_polygons: Vec::new(), model_instances: Vec::new() }
                    };
                    self.areas.push(area);
                },
                Err(err) => { println!("{}", err.to_string()) }
            }
        }
    }

    pub fn set_models(&mut self, paths: Vec<String>) {
        for path in paths.iter() {
            match fs::read_to_string(path) {
                Ok(res) => { 
                    //serde_json::from_str(&res).unwrap();
                    let model: Model = match serde_json::from_str(&res) {
                        Ok(model) => model,
                        Err(err) => Model { name: "error".to_string(), submodels: Vec::new() }
                    };
                    self.model_map.insert(model.name.to_string(), model);
                },
                Err(err) => { println!("{}", err.to_string()) }
            }
        }
    }

    pub fn create_if_exists(base_folder: String) {
        let world_file = [base_folder,"/world.pak".to_string()].join("");
        if !std::path::Path::new(&world_file).exists() { 
            match File::create(&world_file) {
                Err(why) => panic!("Unable to write world file to {}", why),
                Ok(file) => file
            };
         }
    }


    pub fn save(&mut self, base_folder: String) -> io::Result<()> {
        let world_file = [base_folder,"/world.pak".to_string()].join("");
        let mut pos: usize = 0;
        let mut buffer = File::create(world_file)?;

        // Area
        for area in self.areas.iter_mut() {
            for area_texture in area.texture_polygons.iter_mut() {
                for vertex in area_texture.vertices.iter_mut() {
                    pos += write_add(&mut buffer, &vertex.to_be_bytes())?;
                }
            }
            for area_color in area.color_polygons.iter_mut() {
                for vertex in area_color.vertices.iter_mut() {
                    pos += write_add(&mut buffer, &vertex.to_be_bytes())?;
                }
                for color in area_color.color.iter_mut() {
                    pos += write_add(&mut buffer, &color.to_be_bytes())?;
                }
            }
        }
        for (key, value) in self.model_map.iter_mut() {
            pos += write_add(&mut buffer, &key.to_string().as_bytes())?;
        }
        Ok(())
    }
}


fn write_add(buffer: &mut File, data: &[u8])  -> io::Result<(usize)> {
    let bytesWritten = buffer.write(data)?;
    Ok((bytesWritten))
}