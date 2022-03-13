use gl;
use gl::types::*;
use sdl2::ttf::PartialRendering;
use sdl2::surface::Surface;
use sdl2::ttf;
use crate::SDLContext;
use crate::Texture;
use crate::Shader;

extern crate nalgebra_glm as glm;

#[derive(Clone)]

pub struct Text {
    pub text: String,
    pub point: glm::Vec3,
    pub texture: Texture
}

impl Text {
    pub fn new(sdl_payload: &mut SDLContext, text: String, point: glm::Vec3) -> Self {
        let mut texture_id: GLuint = 0;
        let font = match sdl_payload.ttf_context.load_font("res/font/arial.ttf", 128) {
            Ok(x) => x,
            Err(e) => panic!("Cannot load font")
        };
        let renderer: PartialRendering = font.render(&text);
        let surface: Surface = match renderer.blended(sdl2::pixels::Color::RGBA(255 as u8, 0  as u8, 0  as u8, 255  as u8)) {
            Ok(x) => x,
            Err(e) => panic!("Cannot render font")
        };
        return Text {
            //texture: unsafe { Texture::fromSurface(surface, (*img_data).pixels as *const gl::types::GLvoid) },
            texture: Texture::fromSurface(surface),
            text: text,
            point: point
        }
    }

    pub fn change_text(&mut self, sdl_payload: &mut SDLContext, text: String) {
        self.texture.removeTexture();
        let font = match sdl_payload.ttf_context.load_font("res/font/arial.ttf", 128) {
            Ok(x) => x,
            Err(e) => panic!("Cannot load font")
        };
        let renderer: PartialRendering = font.render(&text);
        let surface: Surface = match renderer.blended(sdl2::pixels::Color::RGBA(255 as u8, 0  as u8, 0  as u8, 255  as u8)) {
            Ok(x) => x,
            Err(e) => panic!("Cannot render font")
        };
        self.text = text;
        self.texture = Texture::fromSurface(surface);
    }

    // pub fn draw(&mut self, shader: &mut Shader) {
    //     unsafe {

    //         gl::ActiveTexture(gl::TEXTURE0);
    //         gl::BindTexture(gl::TEXTURE_2D, 1);
    //         shader.set_texture(1);

    //         gl::UniformMatrix4fv(shader.get_uniform_location("model".to_string()), 1, gl::FALSE, &self.get_model()[(0,0)]);
    //         gl::Uniform1i(shader.get_uniform_location("textured".to_string()), 1);

    //         gl::EnableVertexAttribArray(0);
    //         gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
    //         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

    //         gl::EnableVertexAttribArray(1);
    //         gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
    //         gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

    //         gl::EnableVertexAttribArray(2);
    //         gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer);
    //         gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

    //         gl::EnableVertexAttribArray(3);
    //         gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_buffer);
    //         gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

    //         gl::DrawArrays(gl::TRIANGLES, 0, self.length*12*3);

    //         gl::DisableVertexAttribArray(0);
    //         gl::DisableVertexAttribArray(1);
    //     }
    // }
}