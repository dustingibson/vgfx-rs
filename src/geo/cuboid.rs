use gl;
use gl::types::*;
extern crate nalgebra_glm as glm;

pub struct Cuboid {
    pub point: glm::Vec3,
    pub length: GLfloat,
    pub width: GLfloat,
    pub height: GLfloat,
    pub vertex_array: Vec<GLfloat>,
    pub color_array: Vec<GLfloat>,
    vertex_buffer: GLuint,
    color_buffer: GLuint
}

impl Cuboid {

    pub fn new(point: glm::Vec3, color: glm::Vec3, length: GLfloat, width: GLfloat, height: GLfloat) -> Self {
        let mut vertex_buffer: GLuint = 0;
        let mut color_buffer: GLuint = 0;
        let vertex_array = Self::init_vertex_array(point, length, width, height);
        let color_array = Self::init_color_array(color);
        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (vertex_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                vertex_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
        }
        unsafe {
            gl::GenBuffers(1, &mut color_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (color_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                color_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
        }
        return Cuboid {
            point: point,
            length: length,
            width: width,
            height: height,
            vertex_array: vertex_array,
            color_array: color_array,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, 12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    fn init_vertex_array(point: glm::Vec3, length: GLfloat, width: GLfloat, height: GLfloat) -> Vec<GLfloat> {
        return vec![
            point.x, point.y,point.z,
            point.x, point.y, point.z + height,
            point.x, point.y + width, point.z + height,
             point.x + length, point.y + width,point.z,
            point.x, point.y,point.z,
            point.x, point.y + width,point.z,
             point.x + length, point.y, point.z + height,
            point.x, point.y,point.z,
             point.x + length, point.y,point.z,
             point.x + length, point.y + width,point.z,
             point.x + length, point.y,point.z,
            point.x, point.y,point.z,
            point.x, point.y,point.z,
            point.x, point.y + width, point.z + height,
            point.x, point.y + width,point.z,
             point.x + length, point.y, point.z + height,
            point.x, point.y, point.z + height,
            point.x, point.y,point.z,
            point.x, point.y + width, point.z + height,
            point.x, point.y, point.z + height,
             point.x + length, point.y, point.z + height,
             point.x + length, point.y + width, point.z + height,
             point.x + length, point.y,point.z,
             point.x + length, point.y + width,point.z,
             point.x + length, point.y,point.z,
             point.x + length, point.y + width, point.z + height,
             point.x + length, point.y, point.z + height,
             point.x + length, point.y + width, point.z + height,
             point.x + length, point.y + width,point.z,
            point.x, point.y + width,point.z,
             point.x + length, point.y + width, point.z + height,
            point.x, point.y + width,point.z,
            point.x, point.y + width, point.z + height,
             point.x + length, point.y + width, point.z + height,
            point.x, point.y + width, point.z + height,
             point.x + length, point.y, point.z + height
        ];
    }

    fn init_color_array(color: glm::Vec3) -> Vec<GLfloat> {
        let mut resulting_vector: Vec<GLfloat> = Vec::new();
        for x in 0..36 {
            resulting_vector.push(color.x);
            resulting_vector.push(color.y);
            resulting_vector.push(color.z);
        }
        return resulting_vector;
        // return vec![
        //     0.583,  0.771,  0.014,
        //     0.609,  0.115,  0.436,
        //     0.327,  0.483,  0.844,
        //     0.822,  0.569,  0.201,
        //     0.435,  0.602,  0.223,
        //     0.310,  0.747,  0.185,
        //     0.597,  0.770,  0.761,
        //     0.559,  0.436,  0.730,
        //     0.359,  0.583,  0.152,
        //     0.483,  0.596,  0.789,
        //     0.559,  0.861,  0.639,
        //     0.195,  0.548,  0.859,
        //     0.014,  0.184,  0.576,
        //     0.771,  0.328,  0.970,
        //     0.406,  0.615,  0.116,
        //     0.676,  0.977,  0.133,
        //     0.971,  0.572,  0.833,
        //     0.111,  0.616,  0.489,
        //     0.997,  0.513,  0.064,
        //     0.945,  0.719,  0.592,
        //     0.543,  0.021,  0.978,
        //     0.279,  0.317,  0.505,
        //     0.167,  0.620,  0.077,
        //     0.347,  0.857,  0.137,
        //     0.055,  0.953,  0.042,
        //     0.714,  0.505,  0.345,
        //     0.783,  0.290,  0.734,
        //     0.722,  0.645,  0.174,
        //     0.302,  0.455,  0.848,
        //     0.225,  0.587,  0.040,
        //     0.517,  0.713,  0.338,
        //     0.053,  0.959,  0.111,
        //     0.393,  0.621,  0.362,
        //     0.673,  0.211,  0.457,
        //     0.820,  0.883,  0.371,
        //     0.982,  0.099,  0.879
        // ];
    }
}