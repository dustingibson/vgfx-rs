use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::keyboard::KeyboardState;
use std::time::Duration;
use std::rc::Rc;
use crate::Texture;
use crate::SwitchState;

extern crate nalgebra_glm as glm;

pub struct SDLContext {
    pub event_pump: EventPump,
    pub ttf_context: Sdl2TtfContext,
    pub left_mouse_state: SwitchState,
    pub left_click: bool,
    pub terrain_texture: Texture
}

impl SDLContext {
    pub fn new(sdl_context: sdl2::Sdl, ttf_context: Sdl2TtfContext ) -> Self {
        let mut left_mouse_state: SwitchState = SwitchState::new();
        return match sdl_context.event_pump() {
            Ok(val) => { 
                return SDLContext {
                    left_mouse_state: left_mouse_state,
                    event_pump: val,
                    ttf_context: ttf_context,
                    left_click: false,
                    terrain_texture: Texture::fromPackage("test".to_string())
                };
             },
            Err(val) => { panic!("{}", val); }
        }
    }

    pub fn update(&mut self) {
        self.left_click = false;
        if(self.event_pump.mouse_state().left() && !self.left_mouse_state.is_on())
        {
            self.left_mouse_state.flip();
            self.left_click = true;
        }
        else if(!self.event_pump.mouse_state().left() && self.left_mouse_state.is_on())
        {
            self.left_mouse_state.flip();
        }
    }

    pub fn run(&mut self) {
    }
}