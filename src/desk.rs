use crate::assets::DeskAssets;
use crate::input::Input;
use crate::layout::Layout;
use crate::renderer::draw_background;
use crate::state::Transition;
use crate::ui::*;

use macroquad::prelude::*;
use macroquad::rand::rand;

pub struct Desk {
    quest_board: TransparentButton,
    which_background: u32,
}

impl Desk {
    pub fn new(layout: &Layout) -> Self {
        let level_button_size = vec2(360.0, 270.0);
        Self {
            quest_board: TransparentButton::new(
                layout.top_right(level_button_size, 65.0),
                level_button_size,
                ButtonAction::Desk(DeskAction::Quests),
            ),
            which_background: rand(),
        }
    }

    pub fn draw(&self, assets: &DeskAssets) {
        let background = if self.which_background % 2 == 0 {
            &assets.background1
        } else {
            &assets.background2
        };

        draw_background(background);
    }

    pub fn update(&mut self, input: &Input) -> Option<Transition> {
        let action = self.quest_board.update(input);

        if let Some(action) = action {
            return Self::opt_transition_from_action(action);
        }

        if input.key_pressed(KeyCode::Escape) {
            return Some(Transition::Menu);
        }

        None
    }

    fn opt_transition_from_action(action: ButtonAction) -> Option<Transition> {
        use DeskAction::*;

        match action {
            ButtonAction::Desk(Quests) => Some(Transition::Quests),
            _ => None,
        }
    }
}
