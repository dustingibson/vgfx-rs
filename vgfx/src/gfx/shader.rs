use gl;
use gl::types::*;
use std::ascii::AsciiExt;
use std::fs;
use std::ffi::CString;
use std::ptr::null;
use std::ptr::null_mut;
use std::collections::HashMap;
use std::str;
extern crate nalgebra_glm as glm;

#[derive(Clone)]

pub struct ShaderContainer {
    pub shaders: HashMap<String, Shader>,
    pub default_shader: Shader
}

impl ShaderContainer {

    pub fn new() -> Self {
        let mut all_shaders = HashMap::new();

        let mut fragment_shader = Shader::new("fragment".to_string());
        let mut skybox_shader = Shader::new("skybox".to_string());
        let mut color_shader = Shader::new("color".to_string());

        fragment_shader.add_uniform("lightPos".to_string());
        fragment_shader.add_uniform("textureSample".to_string());
        fragment_shader.add_uniform("textured".to_string());
        skybox_shader.add_uniform("textureSample".to_string());

        all_shaders.insert("fragment".to_string(), fragment_shader.clone());
        all_shaders.insert("skybox".to_string(), skybox_shader.clone());
        all_shaders.insert("color".to_string(), color_shader.clone());

        return ShaderContainer {
            shaders: all_shaders,
            default_shader: fragment_shader
        }
    }

    pub fn set_projection_old(&self, view: glm::Mat4, projections: glm::Mat4) {
        for (key, shader) in &self.shaders {
            unsafe {
                if key.eq( &"skybox".to_string() ) {
                    let view_copy = view.clone();
                    let skybox_view = glm::mat3_to_mat4( &glm::mat4_to_mat3( &view_copy ));
                    gl::UniformMatrix4fv(shader.get_uniform_location("view".to_string()), 1, gl::FALSE, &skybox_view[(0,0)]);
                } else {
                    gl::UniformMatrix4fv(shader.get_uniform_location("view".to_string()), 1, gl::FALSE, &view[(0,0)]);
                }
                gl::UniformMatrix4fv(shader.get_uniform_location("projection".to_string()), 1, gl::FALSE, &projections[(0,0)]);
            }
        }
    }

    pub fn set_projection(&self, name: &String, view: glm::Mat4, projections: glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.get_shader(&name).get_uniform_location("view".to_string()), 1, gl::FALSE, &view[(0,0)]);
            gl::UniformMatrix4fv(self.get_shader(&name).get_uniform_location("projection".to_string()), 1, gl::FALSE, &projections[(0,0)]);
        }
    }

    pub fn set_projection_skybox(&self, name: &String, view: glm::Mat4, projections: glm::Mat4) {
        unsafe {
            let view_copy = view.clone();
            let skybox_view = glm::mat3_to_mat4( &glm::mat4_to_mat3( &view_copy ));
            gl::UniformMatrix4fv(self.get_shader(&name).get_uniform_location("view".to_string()), 1, gl::FALSE, &skybox_view[(0,0)]);
            gl::UniformMatrix4fv(self.get_shader(&name).get_uniform_location("projection".to_string()), 1, gl::FALSE, &projections[(0,0)]);
        }
    }

    pub fn get_shader(&self, name: &String) -> Shader {

        match self.shaders.get(name) {
            Some(v) => v.clone(),
            None => self.default_shader.clone()
        }
    }

    pub fn use_shader(&mut self, name: &String) {
        unsafe {
            gl::UseProgram(self.get_shader(name).program_id);
        }
    }

    pub fn unuse_shader(&mut self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn clean_up(self) {
        for (key, value) in self.shaders.iter() {
            value.clean_up();
        }
    }

}

#[derive(Clone)]
pub struct Shader {
    pub vertex_shader: GLuint,
    pub fragment_shader: GLuint,
    pub program_id: u32,
    pub name: String,
    pub uniforms: HashMap<String,GLint>
}

impl Shader {

    pub fn new(name: String) -> Self {
        let vertex_shader : GLuint = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let fragment_shader : GLuint = unsafe{ gl::CreateShader(gl::FRAGMENT_SHADER) };
        Self::compile_shader(vertex_shader, &name, gl::VERTEX_SHADER);
        Self::compile_shader(fragment_shader, &name, gl::FRAGMENT_SHADER);
        let program_id : GLuint = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);
            gl::DetachShader(program_id, vertex_shader);
            gl::DetachShader(program_id, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
        let mut curShader = Shader {
            name: name.clone(),
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program_id: program_id,
            uniforms: HashMap::new()
        };
        curShader.add_uniform("model".to_string());
        curShader.add_uniform("projection".to_string());
        curShader.add_uniform("view".to_string());
        curShader.add_uniform("textureSample".to_string());
        curShader.add_uniform("textured".to_string());
        curShader.add_uniform("ambientColor".to_string());
        //curShader.add_uniform("lightPos".to_string());
        return curShader;
    }

    pub fn set_texture(&self, texture_id: GLuint)
    {
        unsafe  {
            gl::Uniform1i(self.get_uniform_location("textureSample".to_string()), 0);
        }
    }

    pub fn set_texture_id(&self, texture_id: GLuint)
    {
        unsafe  {
            gl::Uniform1i(self.get_uniform_location("textureSample".to_string()), texture_id as i32);
        }
    }

    pub fn compile_shader(shader_id: GLuint, shader_name: &String, shader_types: GLuint) {
        let shader_path = format!("{}{}{}", "src/shaders/", shader_name, if shader_types == gl::FRAGMENT_SHADER {".frag"} else {".vert"});
        let vertex_source = fs::read_to_string(shader_path)
            .expect("Error");
        let c_vertex_source = CString::new(vertex_source).expect("CSTring failed");
        let mut success = 2;
        unsafe {
            gl::ShaderSource(shader_id, 1, &c_vertex_source.as_ptr(), null());
            gl::CompileShader(shader_id);
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }
        if success <= 0 {
            println!("ERROR");
            let mut len = 0;
            unsafe { gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len); }
            let mut buffer = Vec::with_capacity(len as usize);
            unsafe { 
                gl::GetShaderInfoLog(shader_id, len, null_mut(), buffer.as_mut_ptr() as *mut gl::types::GLchar);
                buffer.set_len(len as usize);
            }
            println!("{}", str::from_utf8(&buffer).unwrap());
            panic!("Compile shader {} {}", shader_name, shader_types);
        }
    }

    pub fn add_uniform(&mut self, name: String) {
        let c_name: CString = CString::new(name.clone()).expect("CSTring failed");
        unsafe {
            let loc = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
            self.uniforms.insert(name.clone(), loc);
        }
    }

    pub fn get_uniform_location(&self, name: String) -> GLint {
        match self.uniforms.get(&name) {
            Some(&v) => v,
            None => 0
        }
    }

    pub fn clean_up(&self) {
        unsafe { gl::DeleteProgram(self.program_id); }
    }

}