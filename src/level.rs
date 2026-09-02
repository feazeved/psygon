use crate::assets::LevelAssets;
use crate::combat::Combat;
use crate::combat::{SHIELD, STRIKE};
use crate::entities::Enemy;
use crate::input::Input;
use crate::layout::{Anchor, Layout};
use crate::level_data::{LEVELS, LevelData};
use crate::renderer::draw_background;
use crate::state::Transition;
use crate::ui::*;

use macroquad::prelude::*;

const CARD_SIZE: Vec2 = vec2(160.0, 220.0);
const END_TURN_SIZE: Vec2 = vec2(200.0, 70.0);

pub struct Level {
    index: usize,
    data: &'static LevelData,
    combat: Combat,
    enemy: Enemy,
    player_health_bar: HealthBar,
    player_mana: Mana,
    enemy_health_bar: HealthBar,
    card_position: Vec2,
    end_turn_button: LabeledButton,
}

impl Level {
    pub fn new(index: usize, layout: &Layout, assets: &LevelAssets) -> Self {
        const GOBLIN_SIZE: Vec2 = vec2(256.0, 256.0);
        const HEALTH_BAR_SIZE: Vec2 = vec2(200.0, 50.0);
        let enemy_pos: Vec2 = layout.position(Anchor::Center, GOBLIN_SIZE, vec2(0.0, -10.0));
        let health_bar_pos: Vec2 =
            layout.position(Anchor::BottomLeft, HEALTH_BAR_SIZE, vec2(128.0, -32.0));
        let card_position = layout.position(Anchor::BottomCenter, CARD_SIZE, vec2(0.0, -10.0));

        Self {
            index,
            data: &LEVELS[index],
            combat: Combat::new(
                vec![SHIELD, STRIKE, SHIELD, STRIKE, SHIELD, STRIKE],
                5, //player hp
                3, //player mana
                4, //enemy hp
                3, //enemy dmg
            ),
            enemy: Enemy::new(enemy_pos, GOBLIN_SIZE, &assets.enemy_goblin),
            player_health_bar: HealthBar::new(health_bar_pos, HEALTH_BAR_SIZE),
            player_mana: Mana::new(health_bar_pos + vec2(HEALTH_BAR_SIZE.x / 2.0, -70.0), 50.0),
            enemy_health_bar: HealthBar::new(enemy_pos + vec2(-10.0, -10.0), HEALTH_BAR_SIZE * 0.5),
            card_position,
            end_turn_button: LabeledButton::new(
                "END TURN",
                layout.bottom_right(END_TURN_SIZE, 40.0),
                END_TURN_SIZE,
                ButtonAction::Level(LevelAction::EndTurn),
            ),
        }
    }

    pub fn update(&mut self, input: &Input) -> Option<Transition> {
        if input.key_pressed(KeyCode::Escape) {
            return Some(Transition::Quests);
        }

        self.enemy.update(input);
        self.combat.draw_card();

        let card_rect = Rect::new(
            self.card_position.x,
            self.card_position.y,
            CARD_SIZE.x,
            CARD_SIZE.y,
        );

        if input.mouse_pressed() && card_rect.contains(input.mouse_position()) {
            self.combat.use_current_card();
        }
        if let Some(ButtonAction::Level(LevelAction::EndTurn)) = self.end_turn_button.update(input)
        {
            self.combat.end_round();
        }
        if self.combat.is_enemy_dead() || self.combat.is_player_dead() {
            return Some(Transition::Quests);
        }

        None
    }

    pub fn draw(&self, assets: &LevelAssets) {
        draw_background(&assets.background);
        self.enemy.draw();
        self.player_health_bar
            .draw(self.combat.player_hp, self.combat.player_max_hp);
        self.enemy_health_bar
            .draw(self.combat.enemy_hp, self.combat.enemy_max_hp);
        self.player_mana
            .draw(self.combat.mana, self.combat.max_mana);
        self.end_turn_button.draw();

        if let Some(card) = self.combat.current_card {
            draw_texture_ex(
                assets.cards.texture(card.id),
                self.card_position.x,
                self.card_position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(CARD_SIZE),
                    ..Default::default()
                },
            );
        }
    }
}
