use gl;
use gl::types::*;
use std::collections::HashMap;
use sdl2::surface::Surface;

use crate::BFile;
extern crate nalgebra_glm as glm;

#[derive(Clone)]
pub struct TextureImage {
    pub key: String,
    pub rect: glm::Vec4
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
    pub texture_id: GLuint,
    pub texture_images: HashMap<String, TextureImage>
}

impl Texture {

    pub fn new(name: String) -> Self {

        let mut texture_buffer: GLuint = 0;
        let mut surface: Surface = match sdl2::image::LoadSurface::from_file(format!("res/texture/{}.jpg", name)) {
            Ok(val) => val,
            Err(val) => panic!("unable to load")
        };

        let img_data = surface.raw();
        //unsafe { println!("{:?}",(*img_data).pixels); }

        let mut test_texture = Self::testVector();

        unsafe {
            //let mut surface: sdl2::surface::Surface =  SDL_LoadBMP("texture.bmp");
            gl::GenTextures(1, &mut texture_buffer);
            gl::BindTexture(gl::TEXTURE_2D, texture_buffer);


            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 5000 as i32, 5000 as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, (image_bytes.as_mut_ptr()) as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        let mut texture_images: HashMap<String, TextureImage> = HashMap::new();


        return Texture {
            texture_id: texture_buffer,
            texture_images: texture_images
        }
    }

    pub fn getRectFromTextureImage(&mut self, key: String) -> glm::Vec4  {
        return match self.texture_images.get(&key) {
            Some(val) => val.rect,
            None => panic!("did not find texture in package")
        };
    }

    pub fn fromPackage(name: String) -> Self {
        let mut texture_buffer: GLuint = 0;
        let mut package_file = BFile::new(format!("res/texture/test.tmf"), false);
        let mut image_bytes: Vec<u8> = Vec::new();
        let mut texture_images: HashMap<String, TextureImage> = HashMap::new();

        let mut surface: Surface = match Surface::new(32, 32, sdl2::pixels::PixelFormatEnum::ARGB32) {
            Ok(val) => val,
            Err(val) => panic!("cannot create surface!")
        };

        while(!package_file.isEnd())
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
                let key: String = package_file.autoReadString();
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
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 64, 64, 0, gl::RGB, gl::FLOAT, test_texture.as_ptr() as *const gl::types::GLvoid);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return Texture {
            texture_id: texture_buffer,
            texture_images: texture_images
        }
    }

    pub fn blank() -> Self {
        return Texture {
            texture_id: 1,
            texture_images: HashMap::new()
        }
    }

    pub fn fromSurface(surface: Surface) -> Self {
        let mut texture_buffer: GLuint = 0;
        let img_data = surface.raw();
        unsafe {
            gl::GenTextures(1, &mut texture_buffer);
            gl::BindTexture(gl::TEXTURE_2D, texture_buffer);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, surface.width() as i32, surface.height() as i32, 0, gl::BGRA, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return Texture {
            texture_id: texture_buffer,
            texture_images: HashMap::new()
        }
    }

    pub fn removeTexture(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture_id); }
    }

    fn testVector() -> Vec<GLfloat> {
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