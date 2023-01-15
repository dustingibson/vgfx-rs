
use std::collections::HashMap;

use sdl2::EventPump;
use sdl2::ttf::Sdl2TtfContext;
use crate::Texture;
use crate::SwitchState;
use crate::KeyState;

extern crate nalgebra_glm as glm;

pub struct SDLContext {
    pub event_pump: EventPump,
    pub ttf_context: Sdl2TtfContext,
    pub left_mouse_state: SwitchState,
    pub key_press_state: HashMap<String, KeyState>,
    pub left_click: bool,
    pub terrain_texture: Texture,
    pub ms: u32,
    pub res_width: u32,
    pub res_height: u32
}

impl SDLContext {
    pub fn new(sdl_context: sdl2::Sdl, ttf_context: Sdl2TtfContext, res_width: u32, res_height: u32 ) -> Self {
        
        let left_mouse_state: SwitchState = SwitchState::new();
        return match sdl_context.event_pump() {
            Ok(val) => { 
                return SDLContext {
                    left_mouse_state: left_mouse_state,
                    key_press_state: HashMap::new(),
                    event_pump: val,
                    ttf_context: ttf_context,
                    left_click: false,
                    terrain_texture: Texture::from_package("test".to_string()),
                    ms: 0,
                    res_width: res_width, 
                    res_height: res_height
                };
             },
            Err(val) => { panic!("Eror intalizing SDL Context {}", val); }
        }
    }

    fn update_mouse_click(&mut self) {
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

    fn update_key_press(&mut self) {
        let mut pressed = vec![];
        for scancode in self.event_pump.keyboard_state().pressed_scancodes() {
            pressed.push(scancode.name().to_string());
            if (self.key_press_state.contains_key(scancode.name())) {
                self.key_press_state.get_mut(&scancode.name().to_string()).unwrap().press();
            } else {
                self.key_press_state.insert(scancode.clone().name().to_string(), KeyState::new());
            }
        }
        for (scancode, keystate) in self.key_press_state.iter_mut() {
            if (!pressed.contains(&scancode)) {
                keystate.release();
            }
        }
    }

    pub fn check_pressed(&mut self, scancode: String) -> bool {
        if (self.key_press_state.contains_key(&scancode.to_string())) {
            if (self.key_press_state.get_mut(&scancode.to_string()).unwrap().is_pressed()) {
                return true;
            }
        }
        return false;
    }

    pub fn update(&mut self) {
        self.update_mouse_click();
        self.update_key_press();
    }

    pub fn update_ms(&mut self, ms: u32) {
        self.ms = ms;
    }

    pub fn get_fps(&self) -> u32 {
        return 1000 / self.ms;
    }

    pub fn ms_ratio(&self) -> f32 {
        // Normalize to 60 FPS (i.e. 16.6667 ms)
        return self.ms as f32 / 16.6667;
    }

    pub fn run(&mut self) {
    }
}