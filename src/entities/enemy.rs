use crate::input::Input;
use crate::ui::*;

use macroquad::prelude::*;

pub struct Enemy {
    hitbox: TexturedButton,
}

impl Enemy {
    pub fn new(position: Vec2, size: Vec2, texture: &Texture2D) -> Self {
        use LevelAction::*;

        Self {
            hitbox: TexturedButton::from_texture(
                position,
                size,
                texture,
                ButtonAction::Level(Enemy),
            ),
        }
    }

    pub fn update(&mut self, input: &Input) -> bool {
        self.hitbox.update(input).is_some()
    }

    pub fn draw(&self) {
        self.hitbox.draw();
    }
}
