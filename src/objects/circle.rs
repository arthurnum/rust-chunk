use cgmath::Point2;
use graphics::Gfx;
use gfx_gl::{Gl, LINE_STRIP};

pub struct Circle {
    pos: Point2<f32>,
    r: f32,
    gfx: Box<Gfx>
}

impl Circle {
    pub fn new(gl: &Gl, x: f32, y: f32, r: f32) -> Circle {
        Circle {
            pos: Point2 { x: x, y: y},
            r: r,
            gfx: Box::new(Gfx::build_circle_sample(gl, r, 24))
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gfx.gl.BindVertexArray(self.gfx.vao);
            self.gfx.gl.DrawArrays(LINE_STRIP, 0, 25);
        }
    }
}
