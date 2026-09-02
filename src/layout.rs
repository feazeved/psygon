use macroquad::prelude::*;

pub const VIRTUAL_WIDTH: f32 = 1280.0;
pub const VIRTUAL_HEIGHT: f32 = 720.0;

#[derive(Clone, Copy)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

pub struct Layout {
    scale: f32,
    offset: Vec2,
}

impl Layout {
    pub fn new() -> Self {
        let scale = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );

        let offset = vec2(
            (screen_width() - VIRTUAL_WIDTH * scale) * 0.5,
            (screen_height() - VIRTUAL_HEIGHT * scale) * 0.5,
        );

        Self { scale, offset }
    }

    pub fn position(&self, anchor: Anchor, size: Vec2, offset: Vec2) -> Vec2 {
        let base = match anchor {
            Anchor::TopLeft => vec2(0.0, 0.0),
            Anchor::TopCenter => vec2(VIRTUAL_WIDTH / 2.0 - size.x / 2.0, 0.0),
            Anchor::TopRight => vec2(VIRTUAL_WIDTH - size.x, 0.0),
            Anchor::CenterLeft => vec2(0.0, VIRTUAL_HEIGHT / 2.0 - size.y / 2.0),
            Anchor::Center => vec2(
                VIRTUAL_WIDTH / 2.0 - size.x / 2.0,
                VIRTUAL_HEIGHT / 2.0 - size.y / 2.0,
            ),
            Anchor::CenterRight => {
                vec2(VIRTUAL_WIDTH - size.x, VIRTUAL_HEIGHT / 2.0 - size.y / 2.0)
            }
            Anchor::BottomLeft => vec2(0.0, VIRTUAL_HEIGHT - size.y),
            Anchor::BottomCenter => {
                vec2(VIRTUAL_WIDTH / 2.0 - size.x / 2.0, VIRTUAL_HEIGHT - size.y)
            }
            Anchor::BottomRight => vec2(VIRTUAL_WIDTH - size.x, VIRTUAL_HEIGHT - size.y),
        };

        base + offset
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    pub fn top_left(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::TopLeft, size, vec2(margin, margin))
    }

    pub fn top_center(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::TopCenter, size, vec2(0.0, margin))
    }

    pub fn top_right(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::TopRight, size, vec2(-margin, margin))
    }

    pub fn center_left(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::CenterLeft, size, vec2(margin, 0.0))
    }

    pub fn center(&self, size: Vec2) -> Vec2 {
        self.position(Anchor::Center, size, Vec2::ZERO)
    }

    pub fn center_right(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::CenterRight, size, vec2(-margin, 0.0))
    }

    pub fn bottom_left(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::BottomLeft, size, vec2(margin, -margin))
    }

    pub fn bottom_center(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::BottomCenter, size, vec2(0.0, -margin))
    }

    pub fn bottom_right(&self, size: Vec2, margin: f32) -> Vec2 {
        self.position(Anchor::BottomRight, size, vec2(-margin, -margin))
    }

    pub fn to_virtual_position(&self, position: Vec2) -> Vec2 {
        vec2(
            (position.x - self.offset.x) / self.scale,
            (position.y - self.offset.y) / self.scale,
        )
    }
}
