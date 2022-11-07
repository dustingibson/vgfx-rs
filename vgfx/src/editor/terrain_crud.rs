use std::collections::HashMap;
use gl;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::geo::line::Line;
use crate::geo::plane::Plane;
use crate::model::mesh::Mesh;
use crate::model::mesh::MeshInstance;
use crate::model::model::Model;
use crate::geo::line;

pub struct TerrainCrud {
    pub main_label: Label2D,
    pub mesh_cursor: Option<MeshInstance>,
    pub model_index: i32,
    pub line: Line,
}

impl TerrainCrud {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera, model_map: &HashMap<String, Model>) -> Self {
        let label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 128);
        let mut texture_crud = TerrainCrud {
            main_label: label,
            mesh_cursor: None,
            model_index: 0,
            line: Line::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(100.0, 100.0, 100.0), glm::vec4(1.0,0.0,0.0,1.0), 0.03)
        };
        texture_crud.mesh_cursor = Some(texture_crud.new_mesh_instance(camera, model_map, 0));
        return texture_crud;
    }

    fn new_mesh_instance(&mut self, camera: &mut Camera, model_map: &HashMap<String, Model>, index: u32) -> MeshInstance {
        return MeshInstance {
            position: glm::vec3(0.0, 0.0, 0.0),
            mesh: Mesh::new_triangle(camera.front)
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

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        if (sdl_context.check_pressed("Up".to_string())) {
            //self.next_texture(camera, model_map);
        }
        if (sdl_context.check_pressed("Down".to_string())) {
            //self.prev_texture(camera, model_map);
        }
        self.draw(camera, shader_container, model_map);
    }

    pub fn draw(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer, model_map: &HashMap<String, Model>) {
        self.mesh_cursor.as_mut().unwrap().position = camera.abs_camera_position(100.0);

        shader_container.use_shader(&"fragment".to_string());
        self.mesh_cursor.as_mut().unwrap().draw(&mut shader_container.get_shader(&"fragment".to_string()));
        self.line.draw(&mut shader_container.get_shader(&"fragment".to_string()));
        //self.plane.draw(&mut shader_container.get_shader(&"fragment".to_string()));
        shader_container.unuse_shader();

        shader_container.use_shader(&"color".to_string());
        camera.set_projection(shader_container, &"color".to_string());
        self.mesh_cursor.as_mut().unwrap().draw_stencil(&mut shader_container.get_shader(&"color".to_string()));
        shader_container.unuse_shader();

    }
}