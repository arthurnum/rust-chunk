use cgmath::Point2;
use graphics::Gfx;
use gfx_gl::{Gl, TRIANGLE_STRIP};
use collision::{Aabb2};

pub struct Rectangle {
    pos: Point2<f32>,
    aabb: Aabb2<f32>,
    gfx: Box<Gfx>
}

impl Rectangle {
    pub fn new(gl: &Gl, x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        let wd = w * 0.5;
        let hd = h * 0.5;
        let min = Point2::new(x - wd, y - hd);
        let max = Point2::new(x + wd, y + hd);
        let aabb = Aabb2::new(min, max);
        let vertices: Vec<f32> = aabb.to_corners().iter().flat_map( |&point|
            vec![point.x.clone() as f32, point.y.clone() as f32, 1.0]
        ).collect();

        Rectangle {
            pos: Point2 { x: x, y: y },
            aabb: aabb,
            gfx: Box::new(Gfx::build_rectangle_sample(gl, &vertices))
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gfx.gl.BindVertexArray(self.gfx.vao);
            self.gfx.gl.DrawArrays(TRIANGLE_STRIP, 0, 4);
        }
    }
}
