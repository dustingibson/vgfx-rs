use gl;
use gl::types::*;
use sdl2::mouse::Cursor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;
extern crate nalgebra_glm as glm;
use crate::ShaderContainer;
use crate::Camera;
use crate::Demo;
use crate::SDLContext;

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
    let target_ms: f32 = (1.0/60.0)*1000.0;
    let mut delta_time: u32 = 0;
    let mut sleep_time: u64 = 0;

    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    

    let window = video_subsystem.window("rust-gl demo", WIDTH, HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);


    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        //gl::Viewport(0, 0, 1920, 1080);
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut sdl_timer = sdl_context.timer()?;

    //Vertex array ID
    let mut vertex_array_id: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut shader_container: ShaderContainer = ShaderContainer::new();

    //Uncomment if you need cursor

    //sdl_context.mouse().show_cursor(true);
    sdl_context.mouse().set_relative_mouse_mode(true);
    
    let mut sdl_payload: SDLContext = SDLContext::new(sdl_context, ttf_context);
    let mut camera: Camera = Camera::new( glm::vec3(0.0, 0.0, 0.0), WIDTH as f32, HEIGHT as f32);

    let mut demo: Demo = Demo::new(&mut sdl_payload, &mut camera);

    let mut offset_mouse_x: i32 = 0;
    let mut offset_mouse_y: i32 = 0;

    'running: loop {
        start_ticks = sdl_timer.ticks();
        
        for event in sdl_payload.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        sdl_payload.update();

        // TODO: Offload any camera controls to seperate module

        let mouse_state = sdl_payload.event_pump.relative_mouse_state();
        offset_mouse_x -= mouse_state.x();
        let mouse_delta_x: f32 = (offset_mouse_x) as f32 / WIDTH as f32;
        offset_mouse_y += mouse_state.y();
        let mouse_delta_y = (offset_mouse_y) as f32 / HEIGHT as f32;
        camera.change_angle(mouse_delta_x, mouse_delta_y);

        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::D)) {
            camera.position += glm::normalize(&glm::cross(&camera.front, &glm::vec3(0.0, 0.1, 0.0)) );
            camera.update();
        }
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::A)) {
            camera.position -= glm::normalize(  &glm::cross(&camera.front,&glm::vec3(0.0, 0.1, 0.0)) );
            camera.update();
        }
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::W)) {
            camera.translate(camera.front, 0.1);
        }
        if(sdl_payload.event_pump.keyboard_state().is_scancode_pressed(Scancode::S)) {
            camera.translate(camera.front, -0.1);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            camera.set_projection( &mut shader_container);
            if(command.eq("demo")) {
                demo.run(&mut camera, &mut shader_container);
            }
        }
        window.gl_swap_window();
        delta_time = sdl_timer.ticks() - start_ticks;
        sleep_time = if (target_ms - delta_time as f32) < 0.0 {0} else { (target_ms - delta_time as f32) as u64};
        ::std::thread::sleep(Duration::from_millis(sleep_time));
    }

    //Clean up
    demo.clean_up();
    unsafe{ gl::DeleteVertexArrays(1, &vertex_array_id); }
    shader_container.clean_up();

    Ok(())
}
