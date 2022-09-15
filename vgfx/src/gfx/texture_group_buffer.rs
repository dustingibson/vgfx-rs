use gl;
use gl::types::*;

#[derive(Clone)]
pub struct TextureGroupBuffer {
    pub vertex_buffer: Vec<GLfloat>,
    pub normal_buffer: Vec<GLfloat>,
    pub texture_buffer: Vec<GLfloat>,
    pub texture_index: usize
}

impl TextureGroupBuffer {

    pub fn new(texture_index: u32) -> Self {
        let partition = TextureGroupBuffer {
            texture_index: texture_index as usize,
            vertex_buffer: vec![],
            normal_buffer: vec![],
            texture_buffer: vec![]
        };
        return partition;
    }

    pub fn append_buffer(&mut self, vertex_buffer: &mut Vec<GLfloat>, normal_buffer: &mut Vec<GLfloat>, texture_buffer: &mut Vec<GLfloat>) {
        self.vertex_buffer.append(vertex_buffer);
        self.normal_buffer.append(normal_buffer);
        self.texture_buffer.append(texture_buffer);
    }


}