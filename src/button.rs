use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAction {
    Play,
    Settings,
    Quit,
    Start,
}

pub struct Button {
    pub label: &'static str,
    pub rect: Rect,
    pub action: ButtonAction,
    font_size: f32,
    text_x: f32,
    text_y: f32,
}

impl Button {
    pub fn new(label: &'static str, rect: Rect, action: ButtonAction) -> Self {
        Self {
            label,
            rect,
            action,
            font_size: (rect.h * 0.5).min(40.0),
            text_x: rect.x + (rect.w * 0.1),
            text_y: rect.y + (rect.h * 0.65),
        }
    }

    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        self.rect.contains(vec2(mx, my))
    }

    pub fn draw(&self) {
        let color = if self.is_hovered() { LIGHTGRAY } else { GRAY };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        draw_text(self.label, self.text_x, self.text_y, self.font_size, BLACK);
    }
}
