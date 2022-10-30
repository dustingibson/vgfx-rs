use gl::COLOR_BUFFER_BIT;
use glm::sqrt;

use crate::gfx::{face::FacePartitionRender, texture::Texture, shader::Shader};
extern crate nalgebra_glm as glm;

pub struct MeshVertex {
    pub id: u32,
    pub position: glm::Vec3
}

pub struct MeshEdge {
    pub vertex_a_id: u32,
    pub vertex_b_id: u32
}

pub struct Mesh {
    pub face_partitions: Vec<FacePartitionRender>,
    pub vertices: Vec<MeshVertex>,
    pub edges: Vec<MeshEdge>,
    pub mesh_texture: Texture
}

pub struct MeshInstance {
    pub position: glm::Vec3,
    pub mesh: Mesh
}

impl Mesh {

    fn new() -> Mesh {
        Mesh {
            face_partitions: vec![],
            vertices: vec![],
            edges: vec![],
            mesh_texture: Texture::new("transparent".to_string())
        }
    }

    fn create_mesh_texture(&mut self) { 
        self.mesh_texture.texture_properties.ambient_color = vec![1.0, 1.0, 1.0];
    }

    fn half_distance(s: f32) -> f32 {
        return f32::sqrt(s*s - ((s/2.0)*(s/2.0)));
    }

    pub fn new_triangle(front: glm::Vec3) -> Mesh {
        let mut size: f32 = 8.0;
        let mut dist = Self::half_distance(size);
        let mut new_mesh = Self::new();
        
        
        new_mesh.vertices.push( MeshVertex { 
            id: 0,
            position: glm::vec3(front.x - dist, front.y - dist, 0.0)
        });

        new_mesh.vertices.push( MeshVertex { 
            id: 1,
            position: glm::vec3(front.x + dist, front.y - dist , 0.0)
        });

                
        new_mesh.vertices.push( MeshVertex { 
            id: 2,
            position: glm::vec3(front.x, front.y + dist, 0.0)
        });


        new_mesh.edges.push( MeshEdge { 
            vertex_a_id: 0,
            vertex_b_id: 1
        });

        new_mesh.edges.push( MeshEdge { 
            vertex_a_id: 1,
            vertex_b_id: 2
        });

        new_mesh.edges.push( MeshEdge { 
            vertex_a_id: 2,
            vertex_b_id: 0
        });
        new_mesh.convert_face_partition();
        return new_mesh;
    }

    fn to_vertex_buffer(&mut self) -> Vec<f32> {
        let mut vertex_buffer: Vec<f32> = vec![];
        for vertex in self.vertices.as_slice() {
            vertex_buffer.push(vertex.position.x);
            vertex_buffer.push(vertex.position.y);
            vertex_buffer.push(vertex.position.z);
        }
        return vertex_buffer;
    }

    fn to_normal_buffer(&mut self) -> Vec<f32> {
        let mut normal_buffer: Vec<f32> = vec![];
        for vertex in self.vertices.as_slice() {
            normal_buffer.push(0.0);
            normal_buffer.push(0.0);
            normal_buffer.push(0.0);
        }
        return normal_buffer;
    }

    fn to_texture_buffer(&mut self) -> Vec<f32> {
        let mut texture_buffer: Vec<f32> = vec![];
        for vertex in self.vertices.as_slice() {
            texture_buffer.push(0.0);
            texture_buffer.push(0.0);
        }
        return texture_buffer;
    }

    fn convert_face_partition(&mut self) {
        self.create_mesh_texture();
        let vertex_buffer = self.to_vertex_buffer();
        let normal_buffer = self.to_normal_buffer();
        let texture_buffer = self.to_texture_buffer();
        self.face_partitions.push(FacePartitionRender::new(vertex_buffer, normal_buffer, texture_buffer, 0, 1, 2, true));
    }

    pub fn draw_stencil_mesh(&mut self, shader: &mut Shader, position: &mut glm::Vec3) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            self.draw(shader, position, false);
            gl::StencilMask(0xFF);
            gl::StencilFunc(gl::ALWAYS, 0, 0xFF);
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn draw(&mut self, shader: &mut Shader, position: &mut glm::Vec3, stencil: bool) {
        if stencil {
            unsafe {
                gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
                gl::StencilMask(0xFF);
            }
        }
        for face_partition in self.face_partitions.as_slice() {
            face_partition.draw(shader, position, &self.mesh_texture);
        }
        if stencil {
            unsafe {
                gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
                gl::StencilMask(0x00);
            }
        }
    }
}

impl MeshInstance {

    pub fn draw(&mut self, shader: &mut Shader) {
        self.mesh.draw(shader, &mut self.position, true);
    }

    pub fn draw_stencil(&mut self, shader: &mut Shader) {
        self.mesh.draw_stencil_mesh(shader, &mut self.position);
    }
}