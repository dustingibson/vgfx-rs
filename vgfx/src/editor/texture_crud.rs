use std::collections::HashMap;
use gl;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::model::model::Model;
use crate::model::model::ModelInstance;
use crate::World;

pub struct TextureCrud {
    pub main_label: Label2D,
    pub texture_cursor: Option<ModelInstance>,
    pub model_index: i32
}

impl TextureCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 128);
        let mut texture_crud = TextureCrud {
            main_label: label,
            texture_cursor: None,
            model_index: 0
        };
        texture_crud.texture_cursor = Some(texture_crud.new_model_instance(camera, model_map, 0));
        return texture_crud;
    }

    fn new_model_instance(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, index: u32) -> ModelInstance {
        return ModelInstance {
            model_name: self.model_map_to_index(model_map, index), 
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
            name: "texture_crud".to_string()
        }
    }

    fn model_map_to_index(&mut self, model_map: &HashMap<String, Model>, index: u32) -> String {
        let mut cur_index: u32 = 0;
        for (key, model) in model_map.into_iter() {
            if index == cur_index {
                return model.name.to_string();
            }
            cur_index = cur_index + 1;
        }
        return "".to_string();
    }

    pub fn set_new_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>) {
        self.texture_cursor = Some(self.new_model_instance(camera, model_map, self.model_index as u32));
    }

    pub fn next_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>) {
        if (self.model_index + 1 >= model_map.len() as i32) { self.model_index = 0; } else { self.model_index += 1; }
        self.set_new_texture(camera, model_map);
    }

    pub fn prev_texture(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>) {
        if ( (self.model_index - 1) < 0) { self.model_index = model_map.len() as i32 - 1; } else { self.model_index -= 1; }
        self.set_new_texture(camera, model_map);
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, world: &mut World) {
        if (sdl_context.check_pressed("Up".to_string())) {
            self.next_texture(camera, &world.model_map);
        }
        if (sdl_context.check_pressed("Down".to_string())) {
            self.prev_texture(camera, &world.model_map);
        }
        if (sdl_context.left_click) {
            let x = self.texture_cursor.as_ref().unwrap().position.x;
            let y = self.texture_cursor.as_ref().unwrap().position.y;
            let z = self.texture_cursor.as_ref().unwrap().position.z;
            world.oct_tree.insert_item(Box::new(self.texture_cursor.clone().unwrap()), x, y, z);
        }
        self.draw(camera, shader_container, &world.model_map);
    }

    pub fn draw(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        self.texture_cursor.as_mut().unwrap().position = camera.abs_camera_position(50.0);

        shader_container.use_shader(&"fragment".to_string());
        self.texture_cursor.as_mut().unwrap().draw(&mut shader_container.get_shader(&"fragment".to_string()), model_map, true);
        shader_container.unuse_shader();

        shader_container.use_shader(&"color".to_string());
        camera.set_projection(shader_container, &"color".to_string());
        self.texture_cursor.as_mut().unwrap().draw_stencil(&mut shader_container.get_shader(&"color".to_string()), model_map);
        shader_container.unuse_shader();

    }


}