use std::fmt;

use macroquad::prelude::*;

pub struct Mana {
    position: Vec2,
    radius: f32,
}

impl Mana {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self { position, radius }
    }

    pub fn draw(&self, current: i32, max: i32) {
        draw_circle(self.position.x, self.position.y, self.radius, BLUE);
        draw_circle_lines(self.position.x, self.position.y, self.radius, 5.0, BLACK);
        draw_text(
            fmt::format(format_args!("{}/{}", current, max)),
            self.position.x - 30.0,
            self.position.y + 10.0,
            50.0,
            BLACK,
        );
    }
}
