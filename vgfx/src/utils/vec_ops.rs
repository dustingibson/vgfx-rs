extern crate nalgebra_glm as glm;

pub struct VecOps;

impl VecOps {
    pub fn vec_to_vec3(val: Vec<f32>) -> glm::Vec3 {
        return glm::Vec3::new(val[0], val[1], val[2]);
    }
    
    pub fn vec_to_vec2(val: Vec<f32>) -> glm::Vec2 {
        return glm::Vec2::new(val[0], val[1]);
    }
}
