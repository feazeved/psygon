use macroquad::prelude::*;

use crate::button::{Button, ButtonAction};

const BUTTON_WIDTH_PCT: f32 = 0.20;
const BUTTON_HEIGHT_PCT: f32 = 0.10;
const BUTTON_X_PCT: f32 = 0.20;

const PLAY_Y_PCT: f32 = 0.20;
const SETTINGS_Y_PCT: f32 = 0.40;
const QUIT_Y_PCT: f32 = 0.60;

pub struct Menu {
    background: Texture2D,
    buttons: [Button; 3],
}

impl Menu {
    fn menu_rect(y_delta: f32) -> Rect {
        let sw = screen_width(); // maybe have these as consts in main?
        let sh = screen_height();

        Rect::new(
            sw * BUTTON_X_PCT,
            sh * y_delta,
            sw * BUTTON_WIDTH_PCT,
            sh * BUTTON_HEIGHT_PCT,
        )
    }

    pub async fn new() -> Self {
        let background = load_texture("menu/background.png")
            .await
            .expect("Failed to load menu  background texture");

        let buttons = [
            (Button::new("Play!", Menu::menu_rect(PLAY_Y_PCT), ButtonAction::Play)),
            (Button::new(
                "Settings",
                Menu::menu_rect(SETTINGS_Y_PCT),
                ButtonAction::Settings,
            )),
            (Button::new("Quit", Menu::menu_rect(QUIT_Y_PCT), ButtonAction::Quit)),
        ];
        Self {
            background,
            buttons,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.background,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        for button in &self.buttons {
            button.draw();
        }
    }

    pub fn update(&self) -> Option<ButtonAction> {
        if is_mouse_button_pressed(MouseButton::Left) {
            for button in &self.buttons {
                if button.is_hovered() {
                    return Some(button.action);
                }
            }
        }
        None
    }
}
