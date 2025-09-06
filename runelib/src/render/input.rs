use std::collections::VecDeque;
use skia_safe::Rect;

#[derive(Default, Debug)]
pub struct KeyData {
    pub key_code: i32,
    pub scan_mode: i32,
    pub modifiers: i32,
}

#[derive(Debug)]
pub enum KeyState {
    Pressed(KeyData),
    Released(KeyData)
}

#[derive(Default, Debug)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub type Delta = Position<f64>;

#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

#[derive(Default, Debug)]
pub struct Character {
    pub(crate) code_point: u16,
    pub(crate) modifiers: i32,
}

#[derive(Default, Debug)]
pub struct Input {
    pub mouse_position: Position<i32>,
    pub mouse_button_down: Option<MouseButton>,
    pub scroll_delta: Option<Delta>,
    pub key_state: VecDeque<KeyState>,
    pub typed_characters: VecDeque<Character>
}

impl Input {
    pub fn reset_scroll(&mut self) {
        self.scroll_delta = None;
    }

    pub fn reset_typed_characters(&mut self) {
        self.typed_characters.clear();
    }

    pub fn reset_key_state(&mut self) {
        self.key_state.clear()
    }

    pub fn reset_mouse_button(&mut self) {
        self.mouse_button_down = None;
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_button_down == Some(button)
    }

    pub fn is_mouse_hovering(&self, rect: Rect) -> bool {
        self.mouse_position.x >= rect.left as i32
            && self.mouse_position.x <= rect.right as i32
            && self.mouse_position.y >= rect.top as i32
            && self.mouse_position.y <= rect.bottom as i32
    }
}
