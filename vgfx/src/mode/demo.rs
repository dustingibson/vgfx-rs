use gl;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::Camera;
use crate::Label2D;
use crate::SDLContext;
use crate::World;
use crate::editor::editor::Editor;

pub struct Demo {
    pub world: World,
    pub editor: Editor
}

impl Demo {
    pub fn new(sdl_payload: &mut SDLContext, camera: &mut Camera) -> Self {
        let world: World = World::new_load(sdl_payload);
        return Demo {
            editor: Editor::new(sdl_payload, camera, &world.model_map),
            world: world
        };
    }

    pub fn draw(&mut self, camera: &mut Camera,  shader_container: &mut ShaderContainer) {
        unsafe { 
            gl::UseProgram(shader_container.get_shader("fragment".to_string()).program_id);
            self.world.draw(&mut shader_container.get_shader("fragment".to_string()), camera);
            camera.set_projection(shader_container);
            gl::UseProgram(0);

        }
    }

    pub fn draw_skybox(&mut self, camera: &mut Camera, shader_container: &mut ShaderContainer) {
        unsafe {
            gl::UseProgram(shader_container.get_shader("skybox".to_string()).program_id);
            self.world.draw_skybox(&mut shader_container.get_shader("skybox".to_string()));
            camera.set_projection(shader_container);
            gl::UseProgram(0);
        }
    }

    pub fn clean_up(&mut self) {
        self.world.clean_up();
    }

    pub fn run(&mut self, sdl_context: &mut SDLContext, camera: &mut Camera, shader_container: &mut ShaderContainer) {
        self.draw(camera, shader_container);
        self.editor.run(sdl_context, camera, shader_container, &self.world.model_map);
        self.draw_skybox(camera, shader_container)
    }

    pub fn draw_debug(&mut self, sdl_payload: &mut SDLContext) {
        
    }

}