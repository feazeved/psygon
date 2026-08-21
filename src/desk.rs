use crate::button::{Button, ButtonAction};
use macroquad::prelude::*;

const BUTTON_WIDTH_PCT: f32 = 0.285;
const BUTTON_HEIGHT_PCT: f32 = 0.40;
const BUTTON_X_PCT: f32 = 0.67;

const PLAY_Y_PCT: f32 = 0.10;

pub struct Desk {
    background: Texture2D,
    background2: Texture2D,
    buttons: [Button; 1],
    pub which: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeskAction {
    Main,
    Start,
}

impl Desk {
    fn desk_rect(y_delta: f32) -> Rect {
        let sw = screen_width();
        let sh = screen_height();

        Rect::new(
            sw * BUTTON_X_PCT,
            sh * y_delta,
            sw * BUTTON_WIDTH_PCT,
            sh * BUTTON_HEIGHT_PCT,
        )
    }

    pub async fn new() -> Self {
        let background = load_texture("desk/desk2.png")
            .await
            .expect("Failed to load desk2 texture");
        let background2 = load_texture("desk/desk3.png")
            .await
            .expect("Failed to load desk3 texture");

        let buttons = [(Button::new("", Desk::desk_rect(PLAY_Y_PCT), ButtonAction::Start))];

        Self {
            background,
            background2,
            which: 0,
            buttons,
        }
    }

    pub fn draw(&self) {
        let background = if self.which % 2 == 0 {
            &self.background
        } else {
            &self.background2
        };

        draw_texture_ex(
            &background,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    // Maybe have each screen context be derived from an interface. So that its easy to call
    // context -> update for input and context -> draw for rendering.
    pub fn update(&self) -> Option<DeskAction> {
        if is_mouse_button_pressed(MouseButton::Left) {
            if self.buttons[0].is_hovered() {
                return Some(DeskAction::Start);
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            return Some(DeskAction::Main);
        }
        None
    }
}
