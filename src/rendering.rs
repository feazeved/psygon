use crate::buttons::*;
use crate::game::Game;
use crate::game::state::State;
use macroquad::prelude::*;

pub struct Renderer {
    menu_background: Texture2D,
}

impl Renderer {
    pub async fn new() -> Self {
        Self {
            menu_background: load_texture("menu/background.png").await.unwrap(),
        }
    }

    pub fn draw(&self, game: &Game) {
        clear_background(LIGHTGRAY);

        match game.state() {
            State::Menu => self.draw_menu(),
        }
    }

    fn draw_menu(&self) {
        draw_texture_ex(
            &self.menu_background,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        let play_button = play_button_area();

        let settings_button = settings_button_area();
        let quit_button = quit_button_area();

        // maybe make this struct variables
        draw_button_rectangle("Play!", play_button, GRAY);
        draw_button_rectangle("Settings", settings_button, GRAY);
        draw_button_rectangle("quit", quit_button, GRAY);
    }
}
