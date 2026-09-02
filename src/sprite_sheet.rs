use macroquad::prelude::*;

pub struct SpriteSheet {
    texture: Texture2D,
    sprite_size: Vec2,
    columns: usize,
    sprites: usize,
}

impl SpriteSheet {
    pub fn new(texture: Texture2D, sprite_size: Vec2, columns: usize, sprites: usize) -> Self {
        Self {
            texture,
            sprite_size,
            columns,
            sprites,
        }
    }

    pub fn texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn source(&self, index: usize) -> Rect {
        assert!(index < self.sprites);

        let column = index % self.columns;
        let row = index / self.columns;

        Rect::new(
            column as f32 * self.sprite_size.x,
            row as f32 * self.sprite_size.y,
            self.sprite_size.x,
            self.sprite_size.y,
        )
    }
}
