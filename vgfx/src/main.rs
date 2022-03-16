mod gfx;
mod sdl;
mod geo;
mod model;
mod mode;
mod dep;
mod utils;
mod world;

use gl;
use gfx::shader::Shader;
use gfx::shader::ShaderContainer;
use geo::texture_polygon::TexturePolygon;
use geo::plane::Plane;
use geo::label_2d::Label2D;
use geo::quad::Quad;
use gfx::camera::Camera;
use gfx::texture::Texture;
use gfx::text::Text;
use model::model::Model;
use model::model::AreaInstance;
use model::model::ModelInstance;
use model::sub_model::SubModel;
use mode::demo::Demo;
use world::world::World;
use dep::events::SDLContext;
use utils::bfile::BFile;
use utils::state::SwitchState;
use utils::vec_ops::VecOps;
use std::env;


fn main() {
    let path = match env::current_dir() {
        Ok(v) => v,
        Err(v) => panic!("no path")
    };

    let args: Vec<String> = env::args().collect();
    
    // Usage: Mode Param1 Param2 ...
    // Modes:
    // -------------
    // Demo

    sdl::window::run(&args[1], args[2..].iter().map(|e|e.to_string()).collect() );

}