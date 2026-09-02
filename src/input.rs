use std::collections::HashSet;

use crate::layout::Layout;
use macroquad::prelude::*;

pub struct Input {
    mouse_position: Vec2,
    mouse_pressed: bool,
    mouse_down: bool,
    mouse_released: bool,
    keys_pressed: HashSet<KeyCode>,
    keys_down: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_position: Vec2::ZERO,
            mouse_pressed: false,
            mouse_down: false,
            mouse_released: false,
            keys_pressed: HashSet::new(),
            keys_down: HashSet::new(),
            keys_released: HashSet::new(),
        }
    }

    pub fn update(&mut self, layout: &Layout) {
        let position = vec2(mouse_position().0, mouse_position().1);

        self.mouse_position = layout.to_virtual_position(position);
        self.mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.mouse_down = is_mouse_button_down(MouseButton::Left);
        self.mouse_released = is_mouse_button_released(MouseButton::Left);

        self.keys_pressed = get_keys_pressed();
        self.keys_down = get_keys_down();
        self.keys_released = get_keys_released();
    }
}

impl Input {
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }
}
