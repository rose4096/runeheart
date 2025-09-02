#[derive(Default, Debug)]
pub struct KeyState {
    pub key_code: i32,
    pub scan_mode: i32,
    pub modifiers: i32,
}

#[derive(Default, Debug)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub type Delta = Position<f64>;

#[derive(Default, Debug)]
pub struct Input {
    pub mouse_position: Position<i32>,
    pub mouse_button_down: Option<i32>,
    pub scroll_delta: Option<Delta>,
    pub key_state: Option<KeyState>,
}

impl Input {
    pub fn reset(&mut self) {
        self.mouse_button_down = None;
        self.scroll_delta = None;
        self.key_state = None;
    }
}
