use crate::Model;
use crate::AreaInstance;
use crate::ModelInstance;
use crate::TextureInfo;
use crate::Face;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::{self, File, DirEntry};
use std::io;
use std::path;
use std::str::Bytes;
use serde_json::{Result, Value};
use std::str::Split;

use super::model::FacePartition;


pub struct World {
    base_folder: String,
    model_map: HashMap<String, Model>,
    areas: Vec<AreaInstance>
}

impl World {
    pub fn new(base_folder: String) -> Self {
        Self::create_if_exists(base_folder.to_string());
        return World {
            base_folder: base_folder,
            model_map: HashMap::new(),
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
                            if cont_fname.contains(".json") || cont_fname.contains(".obj") || cont_fname.contains(".mtl") {
                                paths.push(self.base_folder.to_string() + "/" + &sub_dir + "/" + &cont_fname.to_string());
                            }
                        },
                        None => {}
                    };
                },
                None => {}
            };
            //println!("{:?}", path.file_name().ok_or("No file"));
        }
        Ok(paths)
    }

    pub fn get_dir(&mut self, sub_dir: String) -> io::Result<Vec<String>> {
        let mut paths: Vec<String> = vec![];
        for entry in fs::read_dir(self.base_folder.to_string() + "/" + &sub_dir.to_string())? {
            let entry = entry?;
            let path = entry.path();

            match path.file_name() {
                Some(fname) => {
                    match fname.to_str() {
                        Some(cont_fname) => {
                            if !cont_fname.contains(".") {
                                paths.push(sub_dir.to_string() + "/" + &cont_fname.to_string());
                            }
                        },
                        None => {}
                    };
                },
                None => {}
            };
            //println!("{:?}", path.file_name().ok_or("No file"));
        }
        Ok(paths)
    }

    pub fn set_areas(&mut self, paths: Vec<String>) {
        for path in paths.iter() {
            if path.contains(".json") {
                match fs::read_to_string(path) {
                    Ok(res) => {
                        let area: AreaInstance = match serde_json::from_str(&res) {
                            Ok(areaInstance) => areaInstance,
                            Err(err) => {
                                panic!("{}", err.to_string());
                            }
                        };
                        self.areas.push(area);
                    },
                    Err(err) => { println!("{}", err.to_string()) }
                }
            }
        }
    }

    pub fn process_split(vals: &mut Split<char>) -> String {
        return vals.next().unwrap().to_string();
    }

    pub fn process_face_value(&mut self, content: String) -> Face {
        let mut face_values = content.split('/');
        let mode = if face_values.clone().count() == 2 {2} else {3};
        let first_index: usize = face_values.next().unwrap().parse::<usize>().unwrap() - 1;
        let second_index: usize = face_values.next().unwrap().parse::<usize>().unwrap() - 1;
        let third_index: usize = if mode == 2 { 0 } else { face_values.next().unwrap().parse::<usize>().unwrap() - 1};
        return Face {
            mode: mode,
            vertex_index: first_index,
            texture_map_index: second_index,
            normals_index: third_index
        }
    }

    pub fn init_texture_info(&mut self) -> TextureInfo {
        return TextureInfo { 
            name: "".to_string(),
            ambient_color: vec![],
            diffuse_color: vec![],
            specular_color: vec![],
            emissive_coeficient: vec![],
            transmission_filter: vec![],
            specular_highlights: 10.0,
            optical_density: 1.0,
            dissolve: 1.0,
            illum: 1,
            img: vec![]
        };
    }

    pub fn init_model(&mut self) -> Model {
        return Model { 
            name: "".to_string(),
            faces: vec![],
            vertices: vec![],
            texture_mappings: vec![],
            normals: vec![],
            texture_info: vec![]
        };
    }

    pub fn get_byte_from_file(&mut self, fname: String) -> io::Result<Vec<u8>> {
        match fs::read(fname) {
            Ok(res) => { 
                Ok(res)
            },
            Err(err) => { panic!("{}", err.to_string()) }
        }
    }

    pub fn process_texture_info(&mut self, dir_name: String, fname: String) -> io::Result<Vec<TextureInfo>> {
        match fs::read_to_string([dir_name.to_string(), "/".to_string(), fname].join("")) {
            Ok(res) => { 
                let mut new_res = res.replace("\t", "");
                let mut lines = new_res.split('\n');
                let mut texture_infos = vec![];
                let mut texture_info = self.init_texture_info();
                for line in lines {
                    let mut rep_val = line.trim();
                    let mut vals = rep_val.split(' ');
                    let first_val = vals.next().unwrap();
                    if first_val  == "newmtl" {
                        if texture_info.name != "" {                    
                            texture_infos.push(texture_info);
                            texture_info = self.init_texture_info();
                        }
                        texture_info.name = vals.next().unwrap().to_string();
                    }
                    else if first_val == "Ns" {
                        texture_info.specular_highlights = vals.next().unwrap().parse().unwrap();
                    }
                    else if first_val == "Ni" {
                        texture_info.optical_density = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "d" {
                        texture_info.optical_density = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "Tr" {
                        texture_info.optical_density = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "Tf" {
                        texture_info.transmission_filter.push(vals.next().unwrap().parse().unwrap());
                        texture_info.transmission_filter.push(vals.next().unwrap().parse().unwrap());
                        texture_info.transmission_filter.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "illum" {
                        texture_info.illum = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "Ka" {
                        texture_info.ambient_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.ambient_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.ambient_color.push(vals.next().unwrap().parse().unwrap());

                    }
                    else if first_val == "Kd" {
                        texture_info.diffuse_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.diffuse_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.diffuse_color.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "Ks" {
                        texture_info.specular_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.specular_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.specular_color.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "Ke" {
                        texture_info.emissive_coeficient.push(vals.next().unwrap().parse().unwrap());
                        texture_info.emissive_coeficient.push(vals.next().unwrap().parse().unwrap());
                        texture_info.emissive_coeficient.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "map_Ka" {
                        let texture_fname = vals.next().unwrap().to_string();
                        texture_info.img = self.get_byte_from_file([dir_name.to_string(), "/".to_string(), texture_fname].join("")).unwrap();
                    }
                }
                texture_infos.push(texture_info);
                Ok(texture_infos)
            },
            Err(err) => { 
                panic!("{}", err.to_string()) 
            }
        }
    }

    pub fn process_model(&mut self, content: String, dir_name: String) -> Model {
        let mut lines = content.split("\n");
        let mut model = self.init_model();
        let mut cur_texture_index = 0;
        let mut cur_face_partition = FacePartition { faces: vec![], texture_info_index: 0 };
        for line in lines {
            let mut comp = line.trim().split(" ");
            let first_val = comp.next().unwrap();
            if first_val == "v" {
                comp.next().unwrap();
                let mut vertices: Vec<f32> = vec![];
                vertices.push(comp.next().unwrap().parse().unwrap());
                vertices.push(comp.next().unwrap().parse().unwrap());
                vertices.push(comp.next().unwrap().parse().unwrap());
                model.vertices.push(vertices);
            }
            else if first_val == "vn" {
                let mut normals: Vec<f32> = vec![];
                normals.push(comp.next().unwrap().parse().unwrap());
                normals.push(comp.next().unwrap().parse().unwrap());
                normals.push(comp.next().unwrap().parse().unwrap());
                model.normals.push(normals);
            }
            else if first_val == "f" {
                let mut faces: Vec<Face> = vec![];
                faces.push(self.process_face_value(comp.next().unwrap().to_string()));
                faces.push(self.process_face_value(comp.next().unwrap().to_string()));
                faces.push(self.process_face_value(comp.next().unwrap().to_string()));
                cur_face_partition.faces.push(faces);
            }
            else if first_val == "vt" {
                let mut texture_map: Vec<f32> = vec![];
                texture_map.push(comp.next().unwrap().parse().unwrap());
                texture_map.push(comp.next().unwrap().parse().unwrap());
                model.texture_mappings.push(texture_map);
            }
            else if first_val == "usemtl" {
                let texture_key = comp.next().unwrap().to_string();
                cur_texture_index = model.texture_info.iter().position(|x| x.name == texture_key).unwrap();
                if cur_face_partition.faces.len() > 0 {
                    model.faces.push(cur_face_partition);
                }
                cur_face_partition = FacePartition { faces: vec![], texture_info_index: cur_texture_index };
            }
            else if first_val == "mtllib" {
                let mut cur_str = "".to_string();
                loop {
                    let new_word = comp.next();
                    if new_word.is_none() {
                        break;
                    }
                    cur_str.push_str(" ");
                    cur_str.push_str(new_word.unwrap());
                }
                cur_str.remove(0);
                model.texture_info = self.process_texture_info(dir_name.to_string(), cur_str).unwrap();
                //let fname = comp.next().unwrap();
            }
        }
        if cur_face_partition.faces.len() > 0 {
            model.faces.push(cur_face_partition);
        }
        return model;
    }

    pub fn to_fname(&mut self, path_str: String) -> String {
        let path = std::path::Path::new(&path_str);
        return path.file_name().unwrap().to_str().unwrap().to_string().replace(".obj", "").replace(".mtl", "")
    }

    pub fn to_dir(&mut self, path_str: String) -> String {
        let path = std::path::Path::new(&path_str);
        let parent = path.parent().unwrap().to_str().unwrap();
        return parent.to_string();
    }

    pub fn set_models(&mut self, paths: Vec<String>) {
        for path in paths.iter() {
            match fs::read_to_string(path) {
                Ok(res) => { 
                    if path.contains(".obj") {
                        let mut mtl_name = self.to_fname(path.to_string());
                        let mut dir_name = self.to_dir(path.to_string());
                        let model = self.process_model(res, dir_name);
                        self.model_map.insert(mtl_name, model);
                    }
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
            // 2. Count of Area Model Instances
            pos += write_add(&mut buffer, &area.model_instances.len().to_be_bytes())?;
            for model_instance in area.model_instances.iter_mut() {
                // 3. Area's Model Instance Name
                pos += write_str(&mut buffer, &model_instance.model_name.to_string())?;
                // 4. Area's Model Instance Position
                pos += write_vec3(&mut buffer, &model_instance.position)?;
                // 5. Area's Model Instance Scale
                pos += write_add(&mut buffer, &model_instance.scale.to_be_bytes())?;
            }
        }
        // 6. Count of Model Hash Map
        pos += write_add(&mut buffer, &self.model_map.len().to_be_bytes())?;
        for (key, value) in self.model_map.iter_mut() {
            // 7. Model Hash Map Name
            pos += write_str(&mut buffer, &key.to_string())?;
            // 8. Count of Texture Info
            pos += write_add(&mut buffer, &value.texture_info.len().to_be_bytes())?;
            for texture_info in value.texture_info.iter_mut() {
                // 9. Texture Info Name
                pos += write_str(&mut buffer, &texture_info.name)?;
                // 10. Texture Info Ambient Color
                pos += write_vec3(&mut buffer, &texture_info.ambient_color)?;
                // 11. Texture Info Diffuse Color
                pos += write_vec3(&mut buffer, &texture_info.diffuse_color)?;
                // 12. Texture Info Specular Color
                pos += write_vec3(&mut buffer, &texture_info.specular_color)?;
                // 13. Texture Info Emissive Coeficient
                pos += write_vec3(&mut buffer, &texture_info.emissive_coeficient)?;
                // 14. Texture Info Transmission Filter
                pos += write_vec3(&mut buffer, &texture_info.transmission_filter)?;
                // 15. Texture Info Optical Density
                pos += write_add(&mut buffer, &texture_info.optical_density.to_be_bytes())?;
                // 16. Texture Info Dissolve
                pos += write_add(&mut buffer, &texture_info.dissolve.to_be_bytes())?;
                // 17. Texture Info Specular Highlights
                pos += write_add(&mut buffer, &texture_info.specular_highlights.to_be_bytes())?;
                // 18. Texture Info Illum
                pos += write_add(&mut buffer, &texture_info.illum.to_be_bytes())?;
                // 19. Texture Info Image Size
                pos += write_add(&mut buffer, &texture_info.img.len().to_be_bytes())?;
                // 20. Texture Info Image Byte Data
                pos += write_add(&mut buffer, &texture_info.img)?;
            }
            // 21. Count of Vertices
            pos += write_add(&mut buffer, &value.vertices.len().to_be_bytes())?;
            for vertices in value.vertices.iter_mut() {
                // 22. Vertices
                pos += write_vec3(&mut buffer, &vertices)?;
            }
            // 23. Count of Texture Mappings
            pos += write_add(&mut buffer, &value.texture_mappings.len().to_be_bytes())?;
            for texture_mappings in value.texture_mappings.iter_mut() {
                // 24. Texture Mappings
                pos += write_vec2(&mut buffer, &texture_mappings)?;
            }
            // 25. Count of Normals
            pos += write_add(&mut buffer, &value.normals.len().to_be_bytes())?;
            for normals in value.normals.iter_mut() {
                // 26. Normals
                pos += write_vec3(&mut buffer, &normals)?;
            }
            // 27. Count of face partitions
            pos += write_add(&mut buffer, &value.faces.len().to_be_bytes())?;
            for face_partitions in value.faces.iter_mut() {
                // 28. Count of faces in face partitions
                pos += write_add(&mut buffer, &face_partitions.faces.len().to_be_bytes())?;
                // 29. Texture Info Index of Partition
                pos + write_add(&mut buffer, &face_partitions.texture_info_index.to_be_bytes())?;
                for face in face_partitions.faces.iter_mut() {
                    for i in 0..3 {
                        // 30. Face Mode
                        pos += write_add(&mut buffer, &[face[i].mode])?;
                        // 31. Face Texture Vertex Index
                        pos += write_add(&mut buffer, &face[i].vertex_index.to_be_bytes())?;
                        // 32. Face Texture Map Index
                        pos += write_add(&mut buffer, &face[i].texture_map_index.to_be_bytes())?;
                        // 33. Face Texture Normals Index (If Applicable)
                        if face[i].mode == 3 {
                            pos += write_add(&mut buffer, &face[i].normals_index.to_be_bytes())?;
                        }
                    }
                }

            }
        }
        Ok(())
    }
}





fn write_add(buffer: &mut File, data: &[u8])  -> io::Result<usize> {
    let bytesWritten = buffer.write(data)?;
    Ok(bytesWritten)
}

fn write_vec(buffer: &mut File, data: &Vec<f32>) -> io::Result<usize> {
    let mut total_bytes: usize = 0;
    for item in data {
        total_bytes += write_add(buffer, &item.to_be_bytes())?;
    }
    Ok(total_bytes)
}

fn write_vec3(buffer: &mut File, data: &Vec<f32>) -> io::Result<usize> {
    return write_vec(buffer, data);
}

fn write_vec2(buffer: &mut File, data: &Vec<f32>) -> io::Result<usize> {
    return write_vec(buffer, data);
}

fn write_str(buffer: &mut File, data: &str)  -> io::Result<usize> {
    let bytesWrittenSize = write_add(buffer, &data.len().to_be_bytes())?;
    let bytesWrittenStr = write_add(buffer, data.as_bytes())?;
    //let bytesWritten = buffer.write(data)?;
    Ok(bytesWrittenSize + bytesWrittenStr)
}