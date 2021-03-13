use gl;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::Plane;
use crate::Shader;
use crate::ShaderContainer;
use crate::Model;
use crate::SDLContext;
use crate::Label2D;
use crate::Camera;
use crate::BFile;
use crate::Quad;

pub struct WorldEditor {
    pub cursor: Model,
    pub markers: Vec<Model>,
    pub quads: Vec<Quad>,
    pub main_label: Label2D,
    pub block_label: Label2D,
    pub floor_plane: Plane,
    pub temp_pnts: Vec<GLfloat>
}

impl WorldEditor {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera) -> Self {
        let mut label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.0, 0.0), 1.0, 0.2);
        let mut block_label: Label2D = Label2D::new( sdl_payload, camera, "BLAH".to_string(), glm::vec4(1.0,0.0,0.0,1.0), glm::vec3(0.0, 0.4, 0.0), 1.0, 0.2);

        let mut cursor_model = Model::new(glm::vec3(0.0,0.0,0.0));
        let mut cursor_cuboid = Cuboid::from_texture(sdl_payload, camera.position, "brick".to_string(), 0.05, 0.05, 0.05);
        cursor_model.from_single_cuboid(& mut cursor_cuboid);

        return WorldEditor {
            cursor: cursor_model,
            main_label: label,
            block_label: block_label,
            markers: vec![],
            quads: vec![],
            temp_pnts: vec![],
            floor_plane: Plane::new( glm::vec3(0.0,0.0,0.0), glm::vec4(0.0,1.0,0.0, 1.0), 10.0, 10.0)
        };
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader: &mut ShaderContainer) {
        let camera_pos: glm::Vec4 = glm::vec4(camera.position.x + 0.1, camera.position.y, camera.position.z, 1.0);
        let camera_view: glm::Mat4 = camera.get_view();
        let mut dist: f32 = 0.2;
        self.cursor.sub_models[0].position = glm::vec3(camera.position.x + dist*camera.front.x, camera.position.y + dist*camera.front.y, camera.position.z + dist*camera.front.z);
        if(sdl_context.left_click)
        {
            let mut size: f32 = 0.1;
            let final_pos = glm::vec3(camera.position.x + dist*camera.front.x, camera.position.y + dist*camera.front.y, camera.position.z + dist*camera.front.z);
            let mut marker_model = Model::new(final_pos);
            //glm::vec4(0.0,0.0,1.0,1.0)
            let mut marker_cuboid = Cuboid::from_texture(sdl_context, final_pos, "select".to_string(), size, size, size);
            let mut block_str = format!("{} {} {}", final_pos.x, final_pos.y, final_pos.z);
            self.block_label.change_text(sdl_context, block_str);
            marker_model.from_single_cuboid(&mut marker_cuboid);
            self.markers.push(marker_model);
            if(self.markers.len() == 4)
            {
                let mut pnts = vec![self.markers[0].pos_from_cuboid(), self.markers[1].pos_from_cuboid(), self.markers[2].pos_from_cuboid(), self.markers[3].pos_from_cuboid() ];
                self.quads.push(Quad::new(&mut pnts, glm::vec4(1.0, 0.0, 0.0, 1.0)));
                //self.markers.clear();
            }
        }
        let camera_text = format!("{} {} {}", camera.front.x, camera.front.y, camera.front.z);
        self.main_label.change_text(sdl_context, camera_text);
        self.draw(camera, shader);
    }

    pub fn draw(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer) {
        let mut shader: Shader =  shader_container.get_shader("fragment".to_string());
        unsafe { gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id); }
        //self.cursor.draw(&mut shader);
        for cur_marker in self.markers.iter_mut() {
            cur_marker.draw(&mut shader);
        }
        for cur_quad in self.quads.iter_mut() {
            cur_quad.draw(&mut shader);
        }
        self.floor_plane.draw(&mut shader);
        // camera.set_projection_ortho(shader_container);
        // self.block_label.draw(camera, &mut shader);
        // self.main_label.draw(camera, &mut shader);
        // camera.set_projection(shader_container);
    }





}