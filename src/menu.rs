use crate::assets::MenuAssets;
use crate::input::Input;
use crate::layout::Layout;
use crate::renderer::draw_background;
use crate::state::Transition;
use crate::ui::*;
use macroquad::prelude::*;

pub struct Menu {
    play_button: LabeledButton,
    settings_button: LabeledButton,
    quit_button: LabeledButton,
}

impl Menu {
    pub fn new(layout: &Layout) -> Self {
        let button_size = vec2(240.0, 80.0);
        Self {
            play_button: LabeledButton::new(
                "PLAY!",
                layout.center_left(button_size, 150.0) + vec2(0.0, -150.0),
                button_size,
                ButtonAction::Menu(MenuAction::Play),
            ),

            settings_button: LabeledButton::new(
                "SETTINGS",
                layout.center_left(button_size, 150.0) + vec2(0.0, 0.0),
                button_size,
                ButtonAction::Menu(MenuAction::Settings),
            ),

            quit_button: LabeledButton::new(
                "QUIT",
                layout.center_left(button_size, 150.0) + vec2(0.0, 150.0),
                button_size,
                ButtonAction::Menu(MenuAction::Quit),
            ),
        }
    }

    pub fn draw(&self, assets: &MenuAssets) {
        draw_background(&assets.background);
        self.play_button.draw();
        self.settings_button.draw();
        self.quit_button.draw();
    }

    pub fn update(&mut self, input: &Input) -> Option<Transition> {
        let action = self
            .play_button
            .update(input)
            .or_else(|| self.settings_button.update(input))
            .or_else(|| self.quit_button.update(input));

        if let Some(action) = action {
            return Self::opt_transition_from_action(action);
        }

        if input.key_pressed(KeyCode::Escape) {
            return Some(Transition::Quit);
        }

        None
    }

    fn opt_transition_from_action(action: ButtonAction) -> Option<Transition> {
        match action {
            ButtonAction::Menu(MenuAction::Play) => Some(Transition::Desk),
            ButtonAction::Menu(MenuAction::Settings) => Some(Transition::Desk),
            ButtonAction::Menu(MenuAction::Quit) => Some(Transition::Quit),
            _ => None,
        }
    }
}
