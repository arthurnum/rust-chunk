use gfx_gl::*;
use gfx_gl::types::*;
use std::f32::consts::PI;

pub struct Gfx {
    pub vao: u32,
    vbo: u32,
    pub gl: Box<Gl>
}

impl Gfx {
    pub fn build_rectangle_sample(gl: &Gl, vertices: &Vec<f32>) -> Gfx {
        unsafe {
            let mut vao: GLuint = 0;
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo: GLuint = 0;
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(ARRAY_BUFFER, vbo);
            gl.BufferData(ARRAY_BUFFER, 4*12, ::std::mem::transmute(&vertices[0]), STATIC_DRAW);

            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 3, FLOAT, FALSE, 0, ::std::ptr::null());

            Gfx {
                vao: vao,
                vbo: vbo,
                gl: Box::new(gl.clone())
            }
        }
    }

    pub fn build_circle_sample(gl: &Gl, r: f32, d: u32) -> Gfx {
        let mut counter = 2.0 * PI;
        let step = counter / d as f32;
        let mut data: Vec<f32> = Vec::new();
        loop {
            data.append(&mut vec![
                counter.sin() * r,
                counter.cos() * r,
                1.0
                ]);
            counter -= step;
            if counter < step {
                data.append(&mut vec![
                    f32::sin(0.0) * r,
                    f32::cos(0.0) * r,
                    1.0
                    ]);
                break;
            }
        }
        unsafe {
            let mut vao: GLuint = 0;
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo: GLuint = 0;
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(ARRAY_BUFFER, vbo);

            gl.BufferData(ARRAY_BUFFER, 4*data.len() as isize, ::std::mem::transmute(data.as_ptr()), STATIC_DRAW);

            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 3, FLOAT, FALSE, 0, ::std::ptr::null());

            Gfx {
                vao: vao,
                vbo: vbo,
                gl: Box::new(gl.clone())
            }
        }
    }
}
