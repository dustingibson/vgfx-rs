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
                        Err(err) => AreaInstance { texture_polygons: Vec::new(), model_instances: Vec::new() }
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

        // 1. Count of Areas
        pos += write_add(&mut buffer, &self.areas.len().to_be_bytes())?;
        for area in self.areas.iter_mut() {
            // 2. Count of Area Texture Polygons
            pos += write_add(&mut buffer, &area.texture_polygons.len().to_be_bytes())?;
            for area_texture in area.texture_polygons.iter_mut() {
                // 3. Area's Texture Polygon Texture Name
                pos += write_str(&mut buffer, &area_texture.texture_name)?;
                // 4. Area's Texture Polygon Texture Vertices
                pos += write_vec(&mut buffer, &area_texture.vertices)?;
            }
            // 5. Count of Area Model Instances
            pos += write_add(&mut buffer, &area.model_instances.len().to_be_bytes())?;
            for model_instance in area.model_instances.iter_mut() {
                // 6. Area's Model Instance Name
                pos += write_str(&mut buffer, &model_instance.model_name.to_string())?;
                // 7. Area's Model Instance Position
                pos += write_vec(&mut buffer, &model_instance.position)?;
            }
        }
        // 8. Count of Model Hash Map
        pos += write_add(&mut buffer, &self.model_map.len().to_be_bytes())?;
        for (key, value) in self.model_map.iter_mut() {
            // 9. Model Hash Map Name
            pos += write_str(&mut buffer, &key.to_string())?;
            // 10. Count of Model Hash Map Submodel
            pos += write_add(&mut buffer, &value.submodels.len().to_be_bytes())?;
            for submodel in value.submodels.iter_mut() {
                // 11. Model Hash Map Submodel Name
                pos += write_str(&mut buffer, &submodel.name.to_string())?;
                // 12. Model Hash Map Texture Polygon Count
                pos += write_add(&mut buffer, &submodel.texture_polygons.len().to_be_bytes())?;
                for texture_polygon in submodel.texture_polygons.iter_mut() {
                    // 13. Model Hash Map Texture Polygon Name
                    pos += write_str(&mut buffer, &texture_polygon.texture_name.to_string())?;
                    // 14. Model Hash Map Texture Polygon Vertices
                    pos += write_vec(&mut buffer, &texture_polygon.vertices)?;
                }
            }
        }
        Ok(())
    }
}


fn write_add(buffer: &mut File, data: &[u8])  -> io::Result<(usize)> {
    let bytesWritten = buffer.write(data)?;
    Ok((bytesWritten))
}

fn write_vec(buffer: &mut File, data: &Vec<f32>) -> io::Result<(usize)> {
    let mut total_bytes: usize = 0;
    for item in data {
        total_bytes += write_add(buffer, &item.to_be_bytes())?;
    }
    Ok((total_bytes))
}

fn write_str(buffer: &mut File, data: &str)  -> io::Result<(usize)> {
    let bytesWrittenSize = write_add(buffer, &data.len().to_be_bytes())?;
    let bytesWrittenStr = write_add(buffer, data.as_bytes())?;
    //let bytesWritten = buffer.write(data)?;
    Ok((bytesWrittenSize + bytesWrittenStr))
}