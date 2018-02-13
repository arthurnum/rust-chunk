use collision::{Aabb2, Contains};
use cgmath::Point2;
use graphics::Gfx;
use gfx_gl::*;

pub struct RoomUICollection {
    rooms: Vec<RoomUI>
}

impl RoomUICollection {
    pub fn new(count: u8) -> Box<RoomUICollection> {
        let mut rooms: Vec<RoomUI> = Vec::with_capacity(4);
        for i in 0..count {
            let x: u32 = 10 + 100 * i as u32 + 10 * i as u32;
            let y: u32 = 10;
            let room = new_room_ui(i + 1, x, y, 100, 100);
            rooms.push(room);
        }
        Box::new(RoomUICollection {
            rooms: rooms
        })
    }

    pub fn each(&self) -> ::std::slice::Iter<RoomUI> {
        self.rooms.iter()
    }

    pub fn each_mut(&mut self) -> ::std::slice::IterMut<RoomUI> {
        self.rooms.iter_mut()
    }
}

pub struct RoomUI {
    number: u8,
    is_active: bool,
    pub aabb: Aabb2<u32>,
    pub gfx: Option<Gfx>
}

impl RoomUI {
    pub fn contains(&self, x: u32, y: u32) -> bool {
        let p = Point2::new(x, y);
        self.aabb.contains(&p)
    }

    pub fn calc_vertices(&self) -> Vec<f32> {
        let vertices: Vec<f32> = self.aabb.to_corners().iter().flat_map( |&point|
            vec![point.x.clone() as f32, point.y.clone() as f32, 1.0]
        ).collect();
        vertices
    }

    pub fn draw(&self) {
        match self.gfx {
            Some(ref gfx) => {
                unsafe {
                    gfx.gl.BindVertexArray(gfx.vao);
                    gfx.gl.DrawArrays(TRIANGLE_STRIP, 0, 4);
                }
            }

            None => ()
        }
    }

    pub fn is_active(&self) -> bool { self.is_active }
    pub fn number(&self) -> &u8 { &self.number }
    pub fn activate(&mut self) { self.is_active = true; }
    pub fn deactivate(&mut self) { self.is_active = false; }
}

pub fn new_room_ui(number: u8, x: u32, y: u32, w: u32, h: u32) -> RoomUI {
    let min = Point2::new(x, y);
    let max = Point2::new(x + w, y + h);
    let aabb = Aabb2::new(min, max);
    RoomUI {
        number: number,
        is_active: false,
        aabb: aabb,
        gfx: None
    }
}
