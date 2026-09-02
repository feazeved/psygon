use crate::input::Input;
use crate::ui::actions::ButtonAction;
use macroquad::prelude::*;

pub trait Clickable {
    fn rect(&self) -> Rect;
    fn action(&self) -> ButtonAction;
    fn set_hovered(&mut self, hovered: bool);

    fn update(&mut self, input: &Input) -> Option<ButtonAction> {
        let hovered = self.rect().contains(input.mouse_position());
        self.set_hovered(hovered);
        (hovered && input.mouse_pressed()).then(|| self.action())
    }

    fn draw(&self) {}
}
