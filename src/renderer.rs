use crate::layout::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use macroquad::prelude::*;

pub fn draw_background(texture: &Texture2D) {
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(VIRTUAL_WIDTH, VIRTUAL_HEIGHT)),
            ..Default::default()
        },
    );
}

pub struct Renderer {
    target: RenderTarget,
    camera: Camera2D,
}

impl Renderer {
    pub fn new() -> Self {
        let target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        target.texture.set_filter(FilterMode::Linear);

        let camera = Camera2D {
            target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
            zoom: vec2(2.0 / VIRTUAL_WIDTH, 2.0 / VIRTUAL_HEIGHT),
            render_target: Some(target.clone()),
            ..Default::default()
        };

        Self { target, camera }
    }

    pub fn begin(&self) {
        set_camera(&self.camera);
    }

    pub fn end(&self) {
        set_default_camera();

        let scale = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );

        let width = VIRTUAL_WIDTH * scale;
        let height = VIRTUAL_HEIGHT * scale;

        let x = (screen_width() - width) / 2.0;
        let y = (screen_height() - height) / 2.0;

        draw_texture_ex(
            &self.target.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                flip_y: false,
                ..Default::default()
            },
        );
    }
}
