use gl;
use gl::types::*;
use sdl2::ttf::Font;
use sdl2::ttf::PartialRendering;
use sdl2::surface::Surface;
use crate::SDLContext;
use crate::Texture;
use crate::BFile;
use std::fs;
use std::fs::File;
use std::io::Read;

extern crate nalgebra_glm as glm;

#[derive(Clone)]

pub struct Text {
    pub text: String,
    pub point: glm::Vec3,
    pub texture: Texture,
    pub font_size: u16,
    pub font_bytes: Vec<u8>,
    pub surface_size: (u32, u32)
}

impl Text {
    pub fn new(sdl_payload: & mut SDLContext, text: String, point: glm::Vec3, font_size: u16) -> Self {

        let texture_id: GLuint = 0;
        let mut f = File::open(&"res/font/arial.ttf").expect("no file found");
        let metadata = fs::metadata(&"res/font/arial.ttf").expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");


        let mut rwops: sdl2::rwops::RWops = match sdl2::rwops::RWops::from_bytes(&buffer) {
            Ok(val) => val,
            Err(val) => panic!("unable to load rwop")
        };

        let mut font = match sdl_payload.ttf_context.load_font_from_rwops(rwops, font_size) {
            Ok(x) => x,
            Err(e) => panic!("Cannot load font {}", e.as_str())
        };
        let renderer: PartialRendering = font.render(&text);
        let surface: Surface = match renderer.blended(sdl2::pixels::Color::RGBA(255 as u8, 0  as u8, 0  as u8, 255  as u8)) {
            Ok(x) => x,
            Err(e) => panic!("Cannot render font {}", e.to_string())
        };
        let surface_size = surface.size();
        let mut text = Text {
            texture: Texture::from_surface(surface),
            text: text,
            point: point,
            font_size: font_size,
            font_bytes: vec![],
            surface_size: surface_size
        };
        text.font_bytes = buffer.clone();
        return text;

    }

    pub fn change_text(&mut self, sdl_payload: &mut SDLContext, text: String) -> bool {
        if !self.text.eq(&text) {
            self.texture.removeTexture();
            let rwops: sdl2::rwops::RWops = match sdl2::rwops::RWops::from_bytes(&self.font_bytes) {
                Ok(val) => val,
                Err(val) => panic!("unable to load rwop")
            };
            let font = match sdl_payload.ttf_context.load_font_from_rwops(rwops, self.font_size) {
                Ok(x) => x,
                Err(e) => panic!("Cannot load font")
            };
            let renderer: PartialRendering = font.render(&text);
            let surface: Surface = match renderer.blended(sdl2::pixels::Color::RGBA(255 as u8, 0  as u8, 0  as u8, 255  as u8)) {
                Ok(x) => x,
                Err(e) => panic!("Cannot render font")
            };
            self.surface_size = surface.size();
            self.text = text;
            self.texture.update_from_surface(surface);
            return true;
        }
        return false;
    }
}