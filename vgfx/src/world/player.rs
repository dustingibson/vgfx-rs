use crate::dep::events::SDLContext;
use sdl2::{keyboard::Scancode, sys::True};
use crate::Camera;
use super::map::Map;
extern crate nalgebra_glm as glm;

#[derive(PartialEq, Eq)]
enum MovementState {
    NoMovement,
    InputMovement,
    MomentumMovement
}

pub struct Player {
    position: glm::Vec3,
    speed: f32,
    acceleration: f32,
    deaccerlation: f32,
    max_speed: f32,
    // North, East, South, West
    cur_dir: Vec<bool>,
    movement_state: MovementState
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: glm::Vec3::new(0.0, 0.0, 0.0),
            speed: 0.0,
            acceleration: 0.01,
            deaccerlation: 0.05,
            max_speed: 0.5,
            cur_dir: vec![false, false, false, false],
            movement_state: MovementState::NoMovement
        }
    }

    pub fn run(&mut self, sdl_payload: &SDLContext, camera: &mut Camera) {
        self.change_movement(sdl_payload, camera);
    }

    pub fn translate(& mut self, translate_vector: glm::Vec3, product: f32) {
        self.position += translate_vector * product;
    }

    pub fn adjust_speed(&mut self, modify_speed: &mut bool) {
        if (*modify_speed == true) {
            if (self.speed + self.acceleration >= self.max_speed) {
                self.speed = self.max_speed;
            } else {
                self.speed += self.acceleration;
            }
            *modify_speed = false;
        }
    }

    pub fn reduce_speed(&mut self) {
        if (self.speed - self.deaccerlation <= 0.0) {
            self.speed = 0.0;
        } else {
            self.speed -= self.deaccerlation;
        }
    }

    pub fn move_east(&mut self, camera: &mut Camera) {
        self.position += glm::cross(&camera.front, &glm::vec3(0.0, self.speed, 0.0));
    }

    pub fn move_west(&mut self, camera: &mut Camera) {
        self.position -= glm::cross(&camera.front,&glm::vec3(0.0, self.speed, 0.0));
    }

    pub fn move_north(&mut self, camera: &mut Camera) {
        self.translate(camera.front, self.speed);
    }

    pub fn move_south(&mut self, camera: &mut Camera) {
        self.translate(camera.front, -1.0*self.speed);
    }

    pub fn change_movement(&mut self, sdl_payload: &SDLContext, camera: &mut Camera) {
        let mut modify_speed: bool = true;
        let mut prev_dir: Vec<bool> = vec![false, false, false, false];
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
            self.adjust_speed(&mut modify_speed);
            self.move_north(camera);
            prev_dir[0] = true;
        }
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::D)) {
            self.adjust_speed(&mut modify_speed);
            self.move_east(camera);
            prev_dir[1] = true;
        }
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
            self.adjust_speed(&mut modify_speed);
            self.move_south(camera);
            prev_dir[2] = true;
        }
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::A)) {
            self.adjust_speed(&mut modify_speed);
            self.move_west(camera);
            prev_dir[3] = true;
        }
        if (!modify_speed) {
            self.movement_state = MovementState::InputMovement;
        } else {
            if (self.speed > 0.0) {
                self.movement_state = MovementState::MomentumMovement;
            }
        }
        if (self.movement_state == MovementState::InputMovement) {
            self.cur_dir = prev_dir;
        }
        else if (self.movement_state == MovementState::MomentumMovement) {
            // TODO: Maybe a good idea to set a threshold here
            // For an example. Someone just wants to walk then there should be no momentum
            // Perhaps on apply when running. i.e. top speed changes
            self.reduce_speed();
            if (self.cur_dir[0]) {
                self.move_north(camera);
            }
            if (self.cur_dir[1]) {
                self.move_east(camera);
            }
            if (self.cur_dir[2]) {
                self.move_south(camera);
            }
            if (self.cur_dir[3]) {
                self.move_west(camera);
            }
            if (self.speed <= 0.0) {
                self.cur_dir = prev_dir;
                self.movement_state = MovementState::NoMovement;
            }
        }
        // TODO: Check collision here. Adjust camera
        // Remember when bumped hault speed to 0.0
        camera.position = self.position;
        camera.update();
    }
}