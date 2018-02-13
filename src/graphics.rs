use gfx_gl::*;
use gfx_gl::types::*;

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
}
