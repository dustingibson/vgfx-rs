use std::collections::HashMap;

use gl;
use sdl2::keyboard::Scancode;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;
use crate::World;

use super::terrain_crud::TerrainCrud;
use super::texture_crud::TextureCrud;

pub struct Editor {
    camera_coord_label: Label2D,
    editor_mode_label: Label2D,
    texture_crud: TextureCrud,
    terrain_crud: TerrainCrud,
    mode_index: i32,
    max_mode_index: i32,
}

pub enum EditorModes {
    TerrainCrud,
    TextureCrud
}

impl Editor {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let camera_coord_label: Label2D = Label2D::new( sdl_payload, camera, camera.coord_front_str(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 32 );
        let editor_mode_label: Label2D = Label2D::new( sdl_payload, camera, " ".to_string(), glm::vec4(1.0,0.0,0.0,1.0),glm::vec3(0.0, 0.1, 0.0), 32 );
        return Editor {
            camera_coord_label: camera_coord_label,
            editor_mode_label: editor_mode_label,
            mode_index: 0,
            max_mode_index: 1,
            texture_crud: TextureCrud::new(sdl_payload, camera, model_map),
            terrain_crud: TerrainCrud::new(sdl_payload, camera, model_map)
        };
    }

    fn prev_mode(&mut self) {
        self.mode_index -= 1;
        if (self.mode_index < 0) {
            self.mode_index = self.max_mode_index;
        }
    }

    fn next_mode(&mut self) {
        self.mode_index += 1;
        if (self.mode_index > self.max_mode_index) {
            self.mode_index = 0;
        }
    }

    fn to_editor_mode_str(&self) -> String {
        return match self.mode_index {
            0 => "Terrain".to_string(),
            1 => "Texture".to_string(),
            _ => "None".to_string()
        }
    }

    fn update_labels(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera) {
        self.camera_coord_label.change_text(sdl_context, camera.coord_front_str());
        self.editor_mode_label.change_text(sdl_context, self.to_editor_mode_str());
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, world: &mut World) {
        self.update_labels(sdl_context, camera);
        if (sdl_context.check_pressed("Left".to_string())) {
            self.prev_mode();
        }
        if (sdl_context.check_pressed("Right".to_string())) {
            self.next_mode();
        }
        if (sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
        }
        if (sdl_context.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
        }
        match self.mode_index {
            0 => self.texture_crud.run(sdl_context, camera, shader_container, world),
            1 => self.terrain_crud.run(sdl_context, camera, shader_container, &world.model_map),
            _ => {}
        }
    }

    pub fn draw_labels(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer) {
        //unsafe { gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id); }
        shader_container.use_shader(&"fragment".to_string());
        camera.set_projection_ortho(shader_container, &"fragment".to_string());
        self.camera_coord_label.draw(&mut shader_container.get_shader(&"fragment".to_string()));
        self.editor_mode_label.draw(&mut shader_container.get_shader(&"fragment".to_string()));
        camera.set_projection(shader_container, &"fragment".to_string());
        shader_container.unuse_shader();
    }
}