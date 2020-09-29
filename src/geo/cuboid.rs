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
    pub normal_array: Vec<GLfloat>,
    vertex_buffer: GLuint,
    color_buffer: GLuint,
    normal_buffer: GLuint
}

impl Cuboid {

    pub fn new(point: glm::Vec3, color: glm::Vec3, length: GLfloat, width: GLfloat, height: GLfloat) -> Self {
        let mut vertex_buffer: GLuint = 0;
        let mut color_buffer: GLuint = 0;
        let mut normal_buffer: GLuint = 0;
        let vertex_array = Self::init_vertex_array(point, length, width, height);
        let color_array = Self::init_color_array(color);
        let normal_array = Self::init_normal_array();
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
        unsafe {
            gl::GenBuffers(2, &mut normal_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (normal_array.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                normal_array.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW);
        }
        return Cuboid {
            point: point,
            length: length,
            width: width,
            height: height,
            vertex_array: vertex_array,
            color_array: color_array,
            normal_array: normal_array,
            vertex_buffer: vertex_buffer,
            color_buffer: color_buffer,
            normal_buffer: normal_buffer
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

            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null_mut());

            gl::DrawArrays(gl::TRIANGLES, 0, 12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    fn init_vertex_array(point: glm::Vec3, length: GLfloat, width: GLfloat, height: GLfloat) -> Vec<GLfloat> {
        return vec![

        //1
            point.x, point.y,point.z,
            point.x, point.y, point.z + height,
            point.x, point.y + width, point.z + height,
        //2
            point.x + length, point.y + width,point.z,
            point.x, point.y,point.z,
            point.x, point.y + width,point.z,
        //3
             point.x + length, point.y, point.z + height,
            point.x, point.y,point.z,
             point.x + length, point.y,point.z,
        //4
             point.x + length, point.y + width,point.z,
             point.x + length, point.y,point.z,
            point.x, point.y,point.z,
        //5
            point.x, point.y,point.z,
            point.x, point.y + width, point.z + height,
            point.x, point.y + width,point.z,
        //6
             point.x + length, point.y, point.z + height,
            point.x, point.y, point.z + height,
            point.x, point.y,point.z,
        //7
            point.x, point.y + width, point.z + height,
            point.x, point.y, point.z + height,
             point.x + length, point.y, point.z + height,
        //8
             point.x + length, point.y + width, point.z + height,
             point.x + length, point.y,point.z,
             point.x + length, point.y + width,point.z,
        //9
             point.x + length, point.y,point.z,
             point.x + length, point.y + width, point.z + height,
             point.x + length, point.y, point.z + height,
        //10
             point.x + length, point.y + width, point.z + height,
             point.x + length, point.y + width,point.z,
             point.x, point.y + width,point.z,
        //11
             point.x + length, point.y + width, point.z + height,
            point.x, point.y + width,point.z,
            point.x, point.y + width, point.z + height,
        //12
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
    }

    fn init_normal_array() -> Vec<GLfloat> {
        return vec![
            //1
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,

            //2
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,

            //3
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,

            //4
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,
            0.0,  0.0,  -1.0,

            //5
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,
            -1.0,  0.0,  0.0,

            //6
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,
            0.0, -1.0,  0.0,
            
            //7
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,

            //8
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,
            
            //9
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,
            1.0,  0.0,  0.0,

            //10
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,

            //11
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,
            0.0, 1.0,  0.0,

            //12
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,
            0.0,  0.0,  1.0,
        ];
    }
}