mod gfx;
mod sdl;
mod geo;
mod model;
mod mode;
mod dep;

use gl;
use gfx::shader::Shader;
use gfx::shader::ShaderContainer;
use geo::cuboid::Cuboid;
use geo::plane::Plane;
use geo::label::Label;
use geo::label_2d::Label2D;
use gfx::camera::Camera;
use gfx::texture::Texture;
use gfx::text::Text;
use model::model::Model;
use model::sub_model::SubModel;
use mode::demo::Demo;
use mode::edit::ModelEditor;
use dep::events::SDLContext;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Usage: Mode Param1 Param2 ...
    // Modes:
    // -------------
    // Demo

    sdl::window::run(&args[1], args[2..].iter().map(|e|e.to_string()).collect() );

}