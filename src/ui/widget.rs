use crate::input::Input;
use crate::ui::actions::ButtonAction;
use crate::ui::buttons::{LabeledButton, TransparentButton};
use crate::ui::clickable::Clickable;

pub enum Widget {
    Button(LabeledButton),
    Transparent(TransparentButton),
}

impl Widget {
    pub fn update(&mut self, input: &Input) -> Option<ButtonAction> {
        match self {
            Widget::Button(b) => b.update(input),
            Widget::Transparent(t) => t.update(input),
        }
    }

    pub fn draw(&self) {
        match self {
            Widget::Button(b) => b.draw(),
            Widget::Transparent(t) => t.draw(),
        }
    }
}
