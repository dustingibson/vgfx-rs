use gl;
use gl::types::*;
use std::fs;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;
use std::ptr::null_mut;
use std::mem;
use std::collections::HashMap;

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
        return Shader {
            name: String::new(),
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program_id: program_id,
            uniforms: HashMap::new()
        }
    }

    pub fn compile_shader(shader_id: GLuint, shader_name: &String, shader_types: GLuint) {
        let shader_path = format!("{}{}{}", "C:/code/vgfx-rs/src/shaders/", shader_name, if shader_types == gl::FRAGMENT_SHADER {".frag"} else {".vert"});
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
                println!("{}", String::from_utf8(buffer).unwrap());
            }
        }
    }

    pub fn add_uniform(&mut self, name: String) {
        let c_name: CString = CString::new(name.clone()).expect("CSTring failed");
        unsafe {
            let loc = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
            self.uniforms.insert(name.clone(), loc);
        }
    }

    pub fn get_uniform_location(&mut self, name: String) -> GLint {
        match self.uniforms.get(&name) {
            Some(&v) => v,
            None => 0
        }
    }

    pub fn clean_up(&self) {
        unsafe { gl::DeleteProgram(self.program_id); }
    }

}