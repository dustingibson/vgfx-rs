use gl;
use gl::types::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::mem;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::Shader;
use crate::Camera;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn render_test() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::UseProgram(0); 
    }
}

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 1024, 768)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    // let mut canvas = window.into_canvas().index(find_sdl_gl_driver().unwrap()).build()
    //     .expect("could not make a canvas");

    // initialization
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);


    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    // ... in the main loop ...

    unsafe {
        //gl::Viewport(0, 0, 1024, 768);
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }


    
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    //Vertex array ID
    let mut vertex_array_id: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    //Set up shader
    let mut shader = Shader::new("fragment".to_string());
    let light_pos = glm::vec3(-0.5, 2.0, -0.5);
    shader.add_uniform("MVP".to_string());
    shader.add_uniform("lightPos".to_string());
    
    let mut camera: Camera = Camera::new( glm::vec3(4.0, 3.0, 3.0));

    let cuboid: Cuboid = Cuboid::new(glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 0.5, 0.31), 1.0, 1.0, 1.0);
    let cuboidB: Cuboid = Cuboid::new(light_pos, glm::vec3(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Left),..} => {
                    camera.translate(glm::vec3(0.0, 0.0, -0.5));
                    break;
                }
                Event::KeyDown {keycode: Some(Keycode::Right),..} => {
                    camera.translate(glm::vec3(0.0, 0.0, 0.5));
                    break;
                }
                Event::KeyDown {keycode: Some(Keycode::Up),..} => {
                    camera.translate(glm::vec3(0.0, 0.5, 0.0));
                    break;
                }
                Event::KeyDown {keycode: Some(Keycode::Down),..} => {
                    camera.translate(glm::vec3(0.0, -0.5, 0.0));
                    break;
                }
                Event::KeyDown {keycode: Some(Keycode::LeftBracket),..} => {
                    camera.translate(glm::vec3(-0.5, 0.0, 0.0));
                    break;
                }
                Event::KeyDown {keycode: Some(Keycode::RightBracket),..} => {
                    camera.translate(glm::vec3(0.5, -0.5, 0.0));
                    break;
                }
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        unsafe {
            //gl::ClearColor(0.8, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader.program_id);
            gl::UniformMatrix4fv(shader.get_uniform_location("MVP".to_string()), 1, gl::FALSE, &camera.MVP[(0,0)]);
            gl::Uniform3fv(shader.get_uniform_location("lightPos".to_string()), 1, &light_pos[0]);

            cuboid.draw();
            cuboidB.draw();
        }
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
