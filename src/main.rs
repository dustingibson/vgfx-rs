mod gfx;
mod sdl;
mod geo;

use gl;
use gfx::shader::Shader;
use geo::cuboid::Cuboid;
use geo::plane::Plane;
use gfx::camera::Camera;

fn main() {
    sdl::window::run();
}