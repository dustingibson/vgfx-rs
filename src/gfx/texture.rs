use gl;
use gl::types::*;
use std::fs;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;
use std::ptr::null_mut;
use std::mem;
use std::collections::HashMap;
use sdl2::surface::Surface;
use sdl2::image;
extern crate nalgebra_glm as glm;

#[derive(Clone)]

pub struct Texture {
    pub texture_id: GLuint
}

impl Texture {

    pub fn new(name: String) -> Self {
        println!("Setting texture");
        let mut texture_buffer: GLuint = 0;
        let mut surface: Surface = match sdl2::image::LoadSurface::from_file(format!("res/texture/{}.jpg", name)) {
            Ok(val) => val,
            Err(val) => panic!("unable to load")
        };
                //vertex_array.as_ptr() as *const gl::types::GLvoid, 
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

            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 64, 64, 0, gl::RGB, gl::FLOAT, test_texture.as_ptr() as *const gl::types::GLvoid);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, (*img_data).pixels as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        println!("texture ID {0}", texture_buffer);
        return Texture {
            texture_id: texture_buffer
        }
    }

    pub fn fromSurface(surface: Surface) -> Self {
        println!("Setting texture");
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
            println!("displaying...");
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        println!("texture ID {0}", texture_buffer);
        return Texture {
            texture_id: texture_buffer
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