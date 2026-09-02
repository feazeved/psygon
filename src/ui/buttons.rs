use crate::sprite_sheet::SpriteSheet;
use crate::ui::*;
use macroquad::prelude::*;

pub struct LabeledButton {
    rect: Rect,
    label: &'static str,
    action: ButtonAction,
    hovered: bool,
}

pub struct TransparentButton {
    rect: Rect,
    action: ButtonAction,
    hovered: bool,
}

pub struct TexturedButton {
    rect: Rect,
    texture: Texture2D,
    source: Option<Rect>,
    action: ButtonAction,
    hovered: bool,
}

impl LabeledButton {
    pub fn new(label: &'static str, pos: Vec2, size: Vec2, action: ButtonAction) -> Self {
        Self {
            label,
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            action,
            hovered: false,
        }
    }
}

impl Clickable for LabeledButton {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn action(&self) -> ButtonAction {
        self.action
    }

    fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    fn draw(&self) {
        let color = if self.hovered { LIGHTGRAY } else { GRAY };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);

        let dim = measure_text(self.label, None, 50, 1.0);
        draw_text(
            self.label,
            self.rect.x + (self.rect.w - dim.width) / 2.0,
            self.rect.y + (self.rect.h - dim.height),
            50.0,
            WHITE,
        );
    }
}

impl TransparentButton {
    pub fn new(pos: Vec2, size: Vec2, action: ButtonAction) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            action,
            hovered: false,
        }
    }
}

impl Clickable for TransparentButton {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn action(&self) -> ButtonAction {
        self.action
    }

    fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }
}

impl TexturedButton {
    pub fn from_sprite_sheet(
        position: Vec2,
        size: Vec2,
        sheet: &SpriteSheet,
        index: usize,
        action: ButtonAction,
    ) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, size.x, size.y),
            texture: sheet.texture().clone(),
            source: Some(sheet.source(index)),
            action,
            hovered: false,
        }
    }

    pub fn from_texture(
        position: Vec2,
        size: Vec2,
        texture: &Texture2D,
        action: ButtonAction,
    ) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, size.x, size.y),
            texture: texture.clone(),
            source: None,
            action,
            hovered: false,
        }
    }
}

impl Clickable for TexturedButton {
    fn rect(&self) -> Rect {
        self.rect
    }
    fn action(&self) -> ButtonAction {
        self.action
    }
    fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    fn draw(&self) {
        let tint = if self.hovered { LIGHTGRAY } else { WHITE };
        draw_texture_ex(
            &self.texture,
            self.rect.x,
            self.rect.y,
            tint,
            DrawTextureParams {
                dest_size: Some(self.rect.size()),
                source: self.source,
                ..Default::default()
            },
        );
    }
}
