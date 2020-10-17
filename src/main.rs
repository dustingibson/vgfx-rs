mod gfx;
mod sdl;
mod geo;
mod model;
mod mode;

use gl;
use gfx::shader::Shader;
use geo::cuboid::Cuboid;
use geo::plane::Plane;
use gfx::camera::Camera;
use model::model::Model;
use model::sub_model::SubModel;
use mode::demo::Demo;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Usage: Mode Param1 Param2 ...
    // Modes:
    // -------------
    // Demo

    sdl::window::run(&args[1], args[2..].iter().map(|e|e.to_string()).collect() );

}