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
extern crate nalgebra_glm as glm;

pub struct SDLContext {
    pub event_pump: EventPump,
    pub ttf_context: Sdl2TtfContext
}

impl SDLContext {
    pub fn new(sdl_context: sdl2::Sdl, ttf_context: Sdl2TtfContext ) -> Self {
        return match sdl_context.event_pump() {
            Ok(val) => { 
                return SDLContext {
                    event_pump: val,
                    ttf_context: ttf_context
                };
             },
            Err(val) => { panic!("{}", val); }
        }
    }

    pub fn run(&mut self) {

    }
}