use crate::assets::QuestsAssets;
use crate::input::Input;
use crate::layout::{Anchor, Layout};
use crate::renderer::draw_background;
use crate::state::Transition;
use crate::ui::*;

use macroquad::prelude::*;

pub struct Quests {
    icons: Vec<TexturedButton>,
}

impl Quests {
    pub fn new(layout: &Layout, assets: &QuestsAssets) -> Self {
        use QuestAction::*;
        const CELL_SIZE: Vec2 = vec2(256.0, 360.0);
        let mut icons = Vec::new();

        for i in 0..5 {
            let pos = layout.position(
                Anchor::TopLeft,
                CELL_SIZE,
                vec2(CELL_SIZE.x * i as f32, 0.0),
            );
            icons.push(TexturedButton::from_sprite_sheet(
                pos,
                CELL_SIZE,
                &assets.icons,
                i,
                ButtonAction::Quest(Level(i)),
            ));
        }
        for i in 0..5 {
            let pos = layout.position(
                Anchor::BottomLeft,
                CELL_SIZE,
                vec2(CELL_SIZE.x * i as f32, 0.0),
            );
            icons.push(TexturedButton::from_sprite_sheet(
                pos,
                CELL_SIZE,
                &assets.icons,
                i + 5,
                ButtonAction::Quest(Level(i + 5)),
            ));
        }
        Self { icons: icons }
    }

    pub fn draw(&self, assets: &QuestsAssets) {
        draw_background(&assets.background);

        for icon in &self.icons {
            icon.draw();
        }
    }

    pub fn update(&mut self, input: &Input) -> Option<Transition> {
        for icon in &mut self.icons {
            if let Some(ButtonAction::Quest(QuestAction::Level(n))) = icon.update(input) {
                return Some(Transition::Levels(n));
            }
        }

        if input.key_pressed(KeyCode::Escape) {
            return Some(Transition::Desk);
        }

        None
    }
}
