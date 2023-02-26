use crate::Model;
use crate::AreaInstance;
use crate::TextureInfo;
use crate::Face;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::{self, File};
use std::io;
use std::ptr::write_bytes;
use std::str::Split;
use super::model::FacePartition;


pub struct World {
    base_folder: String,
    model_map: HashMap<String, Model>,
    image_map: HashMap<String, Vec<u8>>,
    areas: Vec<AreaInstance>
}

impl World {
    pub fn new(base_folder: String) -> Self {
        Self::create_if_exists(base_folder.to_string());
        return World {
            base_folder: base_folder,
            model_map: HashMap::new(),
            image_map: HashMap::new(),
            areas: Vec::new()
        }
    }

    pub fn get_paths(&mut self, sub_dir: String, accept_paths: &Vec<String>) -> io::Result<Vec<String>> {
        let mut paths: Vec<String> = vec![];
        for entry in fs::read_dir(self.base_folder.to_string() + "/" + &sub_dir.to_string())? {
            let entry = entry?;
            let path = entry.path();
            match path.file_name() {
                Some(fname) => {
                    match fname.to_str() {
                        Some(cont_fname) => {
                            let mut ignore = true;
                            for cur_ignore_path in accept_paths {
                                if cont_fname.contains(cur_ignore_path) {
                                    ignore = false;
                                }
                            }
                            if !ignore {
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
                            Ok(area_instance) => area_instance,
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
            ambient_color: vec![0.0, 0.0, 0.0],
            diffuse_color: vec![0.0, 0.0, 0.0],
            specular_color: vec![0.0, 0.0, 0.0],
            emissive_coeficient: vec![0.0, 0.0, 0.0],
            transmission_filter: vec![0.0, 0.0, 0.0],
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
            texture_info: vec![],
            boundary_points: vec![]
        };
    }

    pub fn get_byte_from_file(& self, fname: String) -> io::Result<Vec<u8>> {
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
                let new_res = res.replace("\t", "");
                let lines = new_res.split('\n');
                let mut texture_infos = vec![];
                let mut texture_info = self.init_texture_info();
                for line in lines {
                    let rep_val = line.trim();
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
                        texture_info.dissolve = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "Tr" {
                        texture_info.optical_density = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "Tf" {
                        texture_info.transmission_filter = vec![];
                        texture_info.transmission_filter.push(vals.next().unwrap().parse().unwrap());
                        texture_info.transmission_filter.push(vals.next().unwrap().parse().unwrap());
                        texture_info.transmission_filter.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "illum" {
                        texture_info.illum = vals.next().unwrap().parse().unwrap();
    
                    }
                    else if first_val == "Ka" {
                        texture_info.ambient_color = vec![];
                        texture_info.ambient_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.ambient_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.ambient_color.push(vals.next().unwrap().parse().unwrap());

                    }
                    else if first_val == "Kd" {
                        texture_info.diffuse_color = vec![];
                        texture_info.diffuse_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.diffuse_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.diffuse_color.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "Ks" {
                        texture_info.specular_color = vec![];
                        texture_info.specular_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.specular_color.push(vals.next().unwrap().parse().unwrap());
                        texture_info.specular_color.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "Ke" {
                        texture_info.emissive_coeficient = vec![];
                        texture_info.emissive_coeficient.push(vals.next().unwrap().parse().unwrap());
                        texture_info.emissive_coeficient.push(vals.next().unwrap().parse().unwrap());
                        texture_info.emissive_coeficient.push(vals.next().unwrap().parse().unwrap());
                    }
                    else if first_val == "map_Ka" || first_val == "map_Kd" {
                        let texture_fname = vals.next().unwrap().to_string();
                        texture_info.img = self.get_byte_from_file(texture_fname).unwrap();
                        //texture_info.img = self.get_byte_from_file([dir_name.to_string(), "/".to_string(), texture_fname].join("")).unwrap();
                        println!("{}", texture_info.img.len());
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

    pub fn points_to_vec(&mut self, x: f32, y: f32, z: f32) -> Vec<f32> {
        let mut out = vec![];
        out.push(x);
        out.push(y);
        out.push(z);
        return out;
    }

    pub fn process_model(&mut self, content: String, dir_name: String) -> Model {
        let lines = content.split("\n");
        let mut model = self.init_model();
        let mut cur_texture_index;
        let mut cur_face_partition = FacePartition { faces: vec![], texture_info_index: 0 };
        let mut min_x: Option<f32> = None;
        let mut min_y: Option<f32> = None;
        let mut min_z: Option<f32> = None;
        let mut max_x: Option<f32> = None;
        let mut max_y: Option<f32> = None;
        let mut max_z: Option<f32> = None;
        for line in lines {
            let mut comp = line.trim().split(" ");
            let first_val = comp.next().unwrap();
            if first_val == "v" {
                //let mut holder = comp.next().unwrap();
                let mut vertices: Vec<f32> = vec![];
                vertices.push(comp.next().unwrap().parse().unwrap());
                vertices.push(comp.next().unwrap().parse().unwrap());
                vertices.push(comp.next().unwrap().parse().unwrap());
                if min_x == None || vertices[0] <= min_x.unwrap() {
                    min_x = Some(vertices[0]);
                }
                if max_x == None || vertices[0] >= max_x.unwrap() {
                    max_x = Some(vertices[0]);
                }
                if min_y == None || vertices[1] <= min_y.unwrap() {
                    min_y = Some(vertices[1]);
                }
                if max_y == None || vertices[1] >= max_y.unwrap() {
                    max_y = Some(vertices[1]);
                }
                if min_z == None || vertices[2] <= min_z.unwrap() {
                    min_z = Some(vertices[2]);
                }
                if max_z == None || vertices[2] >= max_z.unwrap() {
                    max_z = Some(vertices[2]);
                }
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
        let uw_min_x = min_x.unwrap_or(0.0);
        let uw_min_y = min_y.unwrap_or(0.0);
        let uw_min_z = min_z.unwrap_or(0.0);
        let uw_max_x = max_x.unwrap_or(0.0);
        let uw_max_y = max_y.unwrap_or(0.0);
        let uw_max_z = max_z.unwrap_or(0.0);

        model.boundary_points.push(self.points_to_vec(uw_min_x, uw_min_y, uw_min_z));
        model.boundary_points.push(self.points_to_vec(uw_min_x, uw_min_y, uw_max_z));
        model.boundary_points.push(self.points_to_vec(uw_max_x, uw_max_y, uw_max_z));
        model.boundary_points.push(self.points_to_vec(uw_max_x, uw_max_y, uw_min_z));
        model.boundary_points.push(self.points_to_vec(uw_min_x, uw_max_y, uw_max_z));
        model.boundary_points.push(self.points_to_vec(uw_min_x, uw_max_y, uw_min_z));
        model.boundary_points.push(self.points_to_vec(uw_max_x, uw_min_y, uw_max_z));
        model.boundary_points.push(self.points_to_vec(uw_max_x, uw_min_y, uw_min_z));
        if cur_face_partition.faces.len() > 0 {
            model.faces.push(cur_face_partition);
        }
        return model;
    }

    pub fn to_fname(&mut self, path_str: String) -> String {
        let path = std::path::Path::new(&path_str);
        return path.file_name().unwrap().to_str().unwrap().to_string().replace(".obj", "").replace(".mtl", "").replace(".png", "")
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
                        let mtl_name = self.to_fname(path.to_string());
                        let dir_name = self.to_dir(path.to_string());
                        let model = self.process_model(res, dir_name);
                        self.model_map.insert(mtl_name, model);
                    }
                },
                Err(err) => { println!("{}", err.to_string()) }
            }
        }
    }

    pub fn set_image(&mut self, paths: Vec<String>) {
        for path in paths.iter() {
            let name = self.to_fname(path.to_string());
            let img = self.get_byte_from_file(path.to_string()).unwrap();
            self.image_map.insert(name, img);
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

    pub fn save(&mut self, out_folder: String) -> io::Result<()> {
        let world_file = [out_folder,"/world.pak".to_string()].join("");
        let mut pos: usize = 0;
        let mut buffer = File::create(world_file)?;

        // 1. Count of Areas
        pos += write_add(&mut buffer, &self.areas.len().to_be_bytes())?;
        for area in self.areas.iter() {
            // Skybox Left Image Size
            let left_img = self.get_byte_from_file([self.base_folder.to_string(), "/areas/".to_string(), area.skybox.left.to_string() ].join("")).unwrap();
            let right_img = self.get_byte_from_file([self.base_folder.to_string(), "/areas/".to_string(), area.skybox.right.to_string() ].join("")).unwrap();
            let top_img = self.get_byte_from_file([self.base_folder.to_string(), "/areas/".to_string(), area.skybox.top.to_string() ].join("")).unwrap();
            let bottom_img = self.get_byte_from_file([self.base_folder.to_string(), "/areas/".to_string(), area.skybox.bottom.to_string() ].join("")).unwrap();
            let front_img = self.get_byte_from_file([self.base_folder.to_string(), "/areas/".to_string(), area.skybox.front.to_string() ].join("")).unwrap();
            let back_img = self.get_byte_from_file([self.base_folder.to_string(), "/areas/".to_string(), area.skybox.back.to_string() ].join("")).unwrap();
            // 2. Skybox Left Image Size
            pos += write_add(&mut buffer, &left_img.len().to_be_bytes())?;
            // 3. Skybox Left Image Bytes
            pos += write_add(&mut buffer, &left_img)?;
            // 4. Skybox Right Image Size
            pos += write_add(&mut buffer, &right_img.len().to_be_bytes())?;
            // 5. Skybox Right Image Bytes
            pos += write_add(&mut buffer, &right_img)?;
            // 6. Skybox Top Image Size
            pos += write_add(&mut buffer, &top_img.len().to_be_bytes())?;
            // 7. Skybox Top Image Bytes
            pos += write_add(&mut buffer, &top_img)?;
            // 8. Skybox Bottom Image Size
            pos += write_add(&mut buffer, &bottom_img.len().to_be_bytes())?;
            // 9. Skybox Bottom Bytes
            pos += write_add(&mut buffer, &bottom_img)?;
            // 10. Skybox Front Image Size
            pos += write_add(&mut buffer, &front_img.len().to_be_bytes())?;
            // 11. Skybox Front Image Bytes
            pos += write_add(&mut buffer, &front_img)?;
            // 12. Skybox Back Image Size
            pos += write_add(&mut buffer, &back_img.len().to_be_bytes())?;
            // 13. Skybox Back Image Bytes
            pos += write_add(&mut buffer, &back_img)?;
            // 14. Count of Area Model Instances
            pos += write_add(&mut buffer, &area.model_instances.len().to_be_bytes())?;
            for model_instance in area.model_instances.iter() {
                // 15. Area's Model Instance Name
                pos += write_str(&mut buffer, &model_instance.model_name.to_string())?;
                // 16. Area's Model Instance Position
                pos += write_vec3(&mut buffer, &model_instance.position)?;
                // 17. Area's Model Instance Scale
                pos += write_add(&mut buffer, &model_instance.scale.to_be_bytes())?;
            }
        }
        // 18. Count of Model Hash Map
        pos += write_add(&mut buffer, &self.model_map.len().to_be_bytes())?;
        for (key, value) in self.model_map.iter_mut() {
            // 19. Model Hash Map Name
            pos += write_str(&mut buffer, &key.to_string())?;
            // 20. Count of Texture Info
            pos += write_add(&mut buffer, &value.texture_info.len().to_be_bytes())?;
            for texture_info in value.texture_info.iter_mut() {
                // 21. Texture Info Name
                pos += write_str(&mut buffer, &texture_info.name)?;
                // 22. Texture Info Ambient Color
                pos += write_vec3(&mut buffer, &texture_info.ambient_color)?;
                // 23. Texture Info Diffuse Color
                pos += write_vec3(&mut buffer, &texture_info.diffuse_color)?;
                // 24. Texture Info Specular Color
                pos += write_vec3(&mut buffer, &texture_info.specular_color)?;
                // 25. Texture Info Emissive Coeficient
                pos += write_vec3(&mut buffer, &texture_info.emissive_coeficient)?;
                // 26. Texture Info Transmission Filter
                pos += write_vec3(&mut buffer, &texture_info.transmission_filter)?;
                // 27. Texture Info Optical Density
                pos += write_add(&mut buffer, &texture_info.optical_density.to_be_bytes())?;
                // 28. Texture Info Dissolve
                pos += write_add(&mut buffer, &texture_info.dissolve.to_be_bytes())?;
                // 29. Texture Info Specular Highlights
                pos += write_add(&mut buffer, &texture_info.specular_highlights.to_be_bytes())?;
                // 30. Texture Info Illum
                pos += write_add(&mut buffer, &texture_info.illum.to_be_bytes())?;
                // 31. Texture Info Image Size
                pos += write_add(&mut buffer, &texture_info.img.len().to_be_bytes())?;
                // 32. Texture Info Image Byte Data
                pos += write_add(&mut buffer, &texture_info.img)?;
            }
            // 33. Count of Vertices
            pos += write_add(&mut buffer, &value.vertices.len().to_be_bytes())?;
            for vertices in value.vertices.iter_mut() {
                // 34. Vertices
                pos += write_vec3(&mut buffer, &vertices)?;
            }
            // 35. Count of Texture Mappings
            pos += write_add(&mut buffer, &value.texture_mappings.len().to_be_bytes())?;
            for texture_mappings in value.texture_mappings.iter_mut() {
                // 36. Texture Mappings
                pos += write_vec2(&mut buffer, &texture_mappings)?;
            }
            // 37. Count of Normals
            pos += write_add(&mut buffer, &value.normals.len().to_be_bytes())?;
            for normals in value.normals.iter_mut() {
                // 38. Normals
                pos += write_vec3(&mut buffer, &normals)?;
            }
            // 39. Count of face partitions
            pos += write_add(&mut buffer, &value.faces.len().to_be_bytes())?;
            for face_partitions in value.faces.iter_mut() {
                // 40. Count of faces in face partitions
                pos += write_add(&mut buffer, &face_partitions.faces.len().to_be_bytes())?;
                // 41. Texture Info Index of Partition
                pos += write_add(&mut buffer, &face_partitions.texture_info_index.to_be_bytes())?;
                for face in face_partitions.faces.iter_mut() {
                    for i in 0..3 {
                        // 42. Face Mode
                        pos += write_add(&mut buffer, &[face[i].mode])?;
                        // 43. Face Texture Vertex Index
                        pos += write_add(&mut buffer, &face[i].vertex_index.to_be_bytes())?;
                        // 44. Face Texture Map Index
                        pos += write_add(&mut buffer, &face[i].texture_map_index.to_be_bytes())?;
                        // 45. Face Texture Normals Index (If Applicable)
                        if face[i].mode == 3 {
                            pos += write_add(&mut buffer, &face[i].normals_index.to_be_bytes())?;
                        }
                    }
                }
            }
            // 46. Boundary 1
            pos += write_vec3(&mut buffer, &value.boundary_points[0])?;
            // 47. Boundary 2
            pos += write_vec3(&mut buffer, &value.boundary_points[1])?;
            // 48. Boundary 3
            pos += write_vec3(&mut buffer, &value.boundary_points[2])?;
            // 49. Boundary 4
            pos += write_vec3(&mut buffer, &value.boundary_points[3])?;
            // 50. Boundary 5
            pos += write_vec3(&mut buffer, &value.boundary_points[4])?;
            // 51. Boundary 6
            pos += write_vec3(&mut buffer, &value.boundary_points[5])?;
            // 52. Boundary 7
            pos += write_vec3(&mut buffer, &value.boundary_points[6])?;
            // 53. Boundary 8
            pos += write_vec3(&mut buffer, &value.boundary_points[7])?;
        }
        // 54. Count of Images
        pos += write_add(&mut buffer, &self.image_map.len().to_be_bytes())?;
        for (img_key, img_value) in self.image_map.iter_mut() {
            // 55. Name
            pos += write_str(&mut buffer, &img_key.to_string())?;
            // 56. Length of Image
            pos += write_add(&mut buffer, &img_value.len().to_be_bytes())?;
            // 57. Image
            pos += write_add(&mut buffer, &img_value)?;
        }
        Ok(())
    }
}

fn write_add(buffer: &mut File, data: &[u8])  -> io::Result<usize> {
    let bytes_written = buffer.write(data)?;
    Ok(bytes_written)
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
    let bytes_written_size = write_add(buffer, &data.len().to_be_bytes())?;
    let bytes_written_str = write_add(buffer, data.as_bytes())?;
    //let bytesWritten = buffer.write(data)?;
    Ok(bytes_written_size + bytes_written_str)
}