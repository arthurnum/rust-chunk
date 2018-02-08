use collision::{Aabb2, Contains};
use cgmath::Point2;

struct RoomUICollection {
    rooms: Vec<RoomUI>
}

pub struct RoomUI {
    aabb: Aabb2<u32>
}

impl RoomUI {
    pub fn contains(&self, x: u32, y: u32) -> bool {
        let p = Point2::new(x, y);
        self.aabb.contains(&p)
    }
}

pub fn new_room_ui(x: u32, y: u32, w: u32, h: u32) -> RoomUI {
    let min = Point2::new(x, y);
    let max = Point2::new(x + w, y - h);
    let aabb = Aabb2::new(min, max);
    RoomUI {
        aabb: aabb
    }
}
