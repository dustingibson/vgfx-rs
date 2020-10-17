use gl;
use gl::types::*;
use sdl2::mouse::Cursor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;

use crate::Cuboid;
use crate::Plane;
use crate::Shader;
use crate::Camera;
use crate::Demo;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn run(command: &str, params: Vec<String>) -> Result<(), String> {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    const CENTER_X: f32 = WIDTH as f32 / 2.0;
    const CENTER_Y: f32 = HEIGHT as f32 / 2.0;

    let mut start_ticks: u32 = 0;
    let mut end_ticks: u32 = 0;
    // Aim for 60 fps
    let target_ms: f32 = (1.0/30.0)*1000.0;
    let mut delta_time: u32 = 0;
    let mut sleep_time: u64 = 0;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    

    let window = video_subsystem.window("rust-gl demo", WIDTH, HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    // let mut canvas = window.into_canvas().index(find_sdl_gl_driver().unwrap()).build()
    //     .expect("could not make a canvas");

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);


    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        //gl::Viewport(0, 0, 1024, 768);
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut event_pump = sdl_context.event_pump()?;
    let mut sdl_timer = sdl_context.timer()?;

    //Vertex array ID
    let mut vertex_array_id: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    //Set up shader
    let mut shader = Shader::new("fragment".to_string());
    let light_pos = glm::vec3(2.5, 3.0, -0.5);
    shader.add_uniform("model".to_string());
    shader.add_uniform("projection".to_string());
    shader.add_uniform("view".to_string());
    shader.add_uniform("lightPos".to_string());

    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().set_relative_mouse_mode(true);
    
    let mut camera: Camera = Camera::new( glm::vec3(0.0, 0.0, 3.0), WIDTH as f32, HEIGHT as f32);

    let mut demo: Demo = Demo::new();

    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;
    let mut prev_mouse_x: i32 = 0;
    let mut prev_mouse_y: i32 = 0;
    let mut offset_mouse_x: i32 = 0;
    let mut offset_mouse_y: i32 = 0;

    'running: loop {
        start_ticks = sdl_timer.ticks();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        //event_pump.mouse_state().x();
        mouse_x = event_pump.mouse_state().x();
        mouse_y = event_pump.mouse_state().y();

        offset_mouse_x -= (mouse_x - prev_mouse_x);
        offset_mouse_y += (mouse_y - prev_mouse_y);
        //let mouse_ang_x =  if  (mouse_x as f32) < CENTER_X {mouse_x as f32 / CENTER_X} else { CENTER_X / mouse_x as f32 *-1.0 };
        let mouse_delta_x: f32 = (offset_mouse_x) as f32 / WIDTH as f32;
        let mouse_delta_y = (offset_mouse_y) as f32 / HEIGHT as f32;
        //camera.change_yaw(mouse_delta_x);
        //camera.change_pitch(mouse_delta_y);
        camera.change_angle(mouse_delta_x, mouse_delta_y);

        if(event_pump.keyboard_state().is_scancode_pressed(Scancode::Right)) {
            camera.position += glm::normalize(&glm::cross(&camera.front, &glm::vec3(0.0, 1.0, 0.0)) );
            //camera.position += glm::normalize(&glm::cross(&glm::vec3(0.0,0.0,-1.0), &glm::vec3(0.0, 1.0, 0.0)) );
            camera.update();
        }
        if(event_pump.keyboard_state().is_scancode_pressed(Scancode::Left)) {
            camera.position -= glm::normalize(  &glm::cross(&camera.front,&glm::vec3(0.0, 1.0, 0.0)) );
            //camera.position -= glm::normalize(  &glm::cross(&glm::vec3(0.0,0.0,-1.0),&glm::vec3(0.0, 1.0, 0.0)) );
            camera.update();
        }
        if(event_pump.keyboard_state().is_scancode_pressed(Scancode::Up)) {
            camera.translate(camera.front, 1.0);
            //camera.translate(glm::vec3(0.0, 0.0, -1.0));

        }
        if(event_pump.keyboard_state().is_scancode_pressed(Scancode::Down)) {
            camera.translate(camera.front, -1.0);
            //camera.translate(glm::vec3(0.0, 0.0, -1.0));

        }




        unsafe {
            //gl::ClearColor(0.8, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader.program_id);


            camera.set_projection(&mut shader);
            gl::Uniform3fv(shader.get_uniform_location("lightPos".to_string()), 1, &light_pos[0]);


            // cuboid.draw(&mut shader);
            // cuboidB.draw(&mut shader);
            if(command.eq("demo")) {
                demo.run(&mut shader);
            }
        }
        prev_mouse_x = mouse_x;
        prev_mouse_y = mouse_y;
        window.gl_swap_window();
        delta_time = sdl_timer.ticks() - start_ticks;
        sleep_time = if (target_ms - delta_time as f32) < 0.0 {0} else { (target_ms - delta_time as f32) as u64};
        ::std::thread::sleep(Duration::from_millis(sleep_time));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    //Clean up
    demo.clean_up_cuboids();
    unsafe{ gl::DeleteVertexArrays(1, &vertex_array_id); }
    shader.clean_up();

    Ok(())
}
