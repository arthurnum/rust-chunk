#[derive(Debug)]
pub struct InputState {
    pub mouse_rbtn_pressed: bool,
    pub mouse_rbtn_was_pressed: bool,
    pub mouse_rbtn_was_released: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl InputState {
    pub fn default() -> InputState {
        InputState {
            mouse_rbtn_pressed: false,
            mouse_rbtn_was_pressed: false,
            mouse_rbtn_was_released: false,
            mouse_x: 0,
            mouse_y: 0,
        }
    }
}
