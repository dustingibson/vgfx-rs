use gl;
use gl::types::*;
use libc::c_void;
use sdl2::sys::SDL_Surface;
use std::collections::HashMap;
use sdl2::surface::Surface;

use crate::BFile;
extern crate nalgebra_glm as glm;


#[derive(Clone)]
pub struct  TextureProperties {
    pub ambient_color: Vec<f32>,
    pub diffuse_color: Vec<f32>,
    pub specular_color: Vec<f32>,
    pub emissive_coeficient: Vec<f32>,
    pub transmission_filter: Vec<f32>,
    pub optical_density: f32,
    pub dissolve: f32,
    pub specular_highlights: f32,
    pub illum: i32,
}

#[derive(Clone)]
pub struct TextureImage {
    pub key: String,
    pub rect: glm::Vec4
}

impl TextureProperties {
    pub fn new() -> Self{
        return TextureProperties {
            ambient_color: vec![],
            diffuse_color: vec![],
            specular_color: vec![],
            emissive_coeficient: vec![],
            transmission_filter: vec![],
            specular_highlights: 10.0,
            optical_density: 1.0,
            dissolve: 1.0,
            illum: 1,
        }
    }
}

impl TextureImage {

    pub fn new(key: String, rect: glm::Vec4) -> Self {
        return TextureImage {
            key: key.to_string(),
            rect: rect
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    pub name: String,
    pub texture_id: GLuint,
    pub has_img: bool,
    pub texture_images: HashMap<String, TextureImage>,
    pub texture_properties: TextureProperties,
    //Optional
    pub sampler_id: u32
}

impl Texture {

    pub fn new(name: String) -> Self {
        return Texture {
            name: name,
            texture_id: 0,
            has_img: false,
            texture_images: HashMap::new(),
            texture_properties: TextureProperties::new(),
            sampler_id: gl::TEXTURE0
        }
    }

    pub fn get_rect_from_texture_image(&mut self, key: String) -> glm::Vec4  {
        return match self.texture_images.get(&key) {
            Some(val) => val.rect,
            None => panic!("did not find texture in package")
        };
    }

    pub fn from_package(name: String) -> Self {
        let mut texture_buffer: GLuint = 0;
        let mut package_file = BFile::new(format!("res/texture/test.tmf"), false);
        let mut image_bytes: Vec<u8> = Vec::new();
        let mut texture_images: HashMap<String, TextureImage> = HashMap::new();

        let mut surface: Surface = match Surface::new(32, 32, sdl2::pixels::PixelFormatEnum::ARGB32) {
            Ok(val) => val,
            Err(val) => panic!("cannot create surface!")
        };

        while !package_file.is_end()
        {
            let n: u32 = package_file.readu32();
            let width: u32 = package_file.readu32();
            let height: u32 = package_file.readu32();
            let image_size: usize = package_file.readu32() as usize;
            image_bytes = package_file.readbytes(image_size);
            let mut rwops: sdl2::rwops::RWops = match sdl2::rwops::RWops::from_bytes(&mut image_bytes) {
                Ok(val) => val,
                Err(val) => panic!("unable to load rwop")
            };
            surface = match sdl2::image::ImageRWops::load_png(&mut rwops) {
                Ok(val) => val,
                Err(val) => panic!("unable to load surface")
            };
            for i in 0..n {
                let key: String = package_file.auto_read_string();
                println!("{}", key);
                let x: u32 = package_file.readu32();
                let y: u32 = package_file.readu32();
                let w: u32 = package_file.readu32();
                let h: u32 = package_file.readu32();
                let rect: glm::Vec4 = glm::vec4(x as f32 / width as f32 , y as f32 / height as f32, (x+w) as f32 / width as f32 , (y+h) as f32 / height as f32 );
                texture_images.insert(key.to_string(), TextureImage::new(key, rect));
            }
            break;
        }
        let img_data = surface.raw();

        unsafe {
            //let mut surface: sdl2::surface::Surface =  SDL_LoadBMP("texture.bmp");
            gl::GenTextures(1, &mut texture_buffer);
            gl::BindTexture(gl::TEXTURE_2D, texture_buffer);


            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);

            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 64, 64, 0, gl::RGB, gl::FLOAT, test_texture.as_ptr() as *const gl::types::GLvoid);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return Texture {
            name: name,
            texture_id: texture_buffer,
            texture_images: texture_images,
            has_img: true,
            texture_properties: TextureProperties::new(),
            sampler_id: gl::TEXTURE0
        }
    }

    fn flip_surface(surface: &Surface) {
        unsafe {
            let pitch = surface.pitch();
            let height = surface.height();
            let mut buffer = vec![0; pitch as usize];
            let img_data = surface.raw();
            let pixels = (*img_data).pixels as *mut c_void;
            for i in 0..surface.height()/2 {
                let bot_pos = ( ( (height- i - 1)*pitch ) as isize) as isize;
                let top_row = pixels.add((i * pitch) as usize);
                let bot_row = pixels.add(bot_pos as usize);
                std::ptr::copy_nonoverlapping(top_row, buffer.as_mut_ptr() as *mut c_void, pitch as usize);
                std::ptr::copy_nonoverlapping(bot_row, top_row, pitch as usize);
                std::ptr::copy_nonoverlapping(buffer.as_mut_ptr() as *mut c_void, bot_row, pitch as usize);
            }
        }
    }

    fn surface_from_byte_data(image_bytes: &[u8], flip_surface: bool) -> Surface {
        let mut rwops: sdl2::rwops::RWops = match sdl2::rwops::RWops::from_bytes(&image_bytes) {
            Ok(val) => val,
            Err(val) => panic!("unable to load rwop")
        };
        let mut surface = match sdl2::image::ImageRWops::load_png(&mut rwops) {
            Ok(val) => val,
            Err(val) => panic!("unable to load surface")
        };
        surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();
        if flip_surface {
            Self::flip_surface(&surface);
        }
        return surface;
    }

    pub fn create_texture_buffer_from_byte_data(&mut self, image_bytes: &[u8]) {
        let surface = Self::surface_from_byte_data(image_bytes, false);
        let img_data = surface.raw();
        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.has_img = true;
    }

    pub fn create_cube_map_texture_buffer_from_byte_data(&mut self, all_bytes: Vec<Vec<u8>> ) {

        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);	
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
            let mut index = 0;
            for image_bytes in all_bytes {
                let surface = Self::surface_from_byte_data(&image_bytes, false);
                let img_data = surface.raw();
                gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + index, 0, gl::RGBA as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
                index = index + 1;
            }
        }
        self.has_img = true;
    }

    pub fn from_surface(surface: Surface) -> Self {
        let mut texture_buffer: GLuint = 0;
        let img_data = surface.raw();
        unsafe {
            gl::GenTextures(1, &mut texture_buffer);
            gl::BindTexture(gl::TEXTURE_2D, texture_buffer);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, surface.width() as i32, surface.height() as i32, 0, gl::BGRA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return Texture {
            name: "".to_string(),
            texture_id: texture_buffer,
            has_img: true,
            texture_images: HashMap::new(),
            texture_properties: TextureProperties::new(),
            sampler_id: gl::TEXTURE0
        }
    }

    pub fn update_from_surface(&mut self, surface: Surface) {
        let img_data = surface.raw();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, surface.width() as i32, surface.height() as i32, 0, gl::BGRA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn removeTexture(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture_id); }
    }

    fn test_vector() -> Vec<GLfloat> {
        let mut test_texture = vec![];

        for w in 0..64 {
            for h in 0..64 {
                if w % 2 == 0 {
                    test_texture.push(1.0);
                    test_texture.push(0.0);
                    test_texture.push(0.0);
                    //test_texture.push(1.0);
                } else {
                    test_texture.push(0.0);
                    test_texture.push(0.0);
                    test_texture.push(1.0);
                    //test_texture.push(1.0);
                }
            }
        }
        return test_texture;
    }
}