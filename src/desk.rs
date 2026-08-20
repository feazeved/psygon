use macroquad::prelude::*;

const DRAGOA_X_PCT: f32 = 0.217;
const DRAGOA_Y_PCT: f32 = 0.11;

pub struct Desk {
    background: Texture2D,
    dragoa: Texture2D,
}

impl Desk {
    pub async fn new() -> Self {
        let background = load_texture("desk/desk.png")
            .await
            .expect("Failed to load desk texture");

        let dragoa = load_texture("desk/dragoa2.png") // only dragoa2 is working atm because of hardcode...
            .await
            .expect("Failed to load dragoa image");

        Self { background, dragoa }
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

        let scale: f32 = 0.6;
        let sw = screen_width();
        let sh = screen_height();
        let dest_size = vec2(self.dragoa.width() * scale, self.dragoa.height() * scale);

        draw_texture_ex(
            &self.dragoa,
            sw * DRAGOA_X_PCT,
            sh * DRAGOA_Y_PCT,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                ..Default::default()
            },
        );
    }
}
