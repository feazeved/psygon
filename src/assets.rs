use macroquad::prelude::*;

use crate::combat::CardId;
use crate::sprite_sheet::SpriteSheet;

pub struct MenuAssets {
    pub background: Texture2D,
}

pub struct DeskAssets {
    pub background1: Texture2D,
    pub background2: Texture2D,
}

pub struct QuestsAssets {
    pub background: Texture2D,
    pub icons: SpriteSheet,
}

pub struct CardAssets {
    strike: Texture2D,
    shield: Texture2D,
}

pub struct LevelAssets {
    pub background: Texture2D,
    pub enemy_lhama: Texture2D,
    pub enemy_goblin: Texture2D,
    pub cards: CardAssets,
}

pub struct Assets {
    pub menu: MenuAssets,
    pub desk: DeskAssets,
    pub quests: QuestsAssets,
    pub level: LevelAssets,
}

impl MenuAssets {
    async fn new() -> Self {
        let background = load_texture("menu/background.png")
            .await
            .expect("Failed to load menu/background");

        Self { background }
    }
}

impl DeskAssets {
    async fn new() -> Self {
        let background1 = load_texture("desk/background1.png")
            .await
            .expect("Failed to load desk/background1");

        let background2 = load_texture("desk/background2.png")
            .await
            .expect("Failed to load desk/background2");

        Self {
            background1,
            background2,
        }
    }
}

impl QuestsAssets {
    async fn new() -> Self {
        let background = load_texture("quests/background.png")
            .await
            .expect("Failed to load quests/background.png");

        let icons_texture = load_texture("quests/all_levels.png")
            .await
            .expect("Failed to load quests/all_levels.png");

        const CELL_WIDHT: f32 = 256.0;
        const CELL_HEIGHT: f32 = 360.0;
        let columns = 5;
        let sprites = 10;

        Self {
            background,
            icons: SpriteSheet::new(
                icons_texture,
                vec2(CELL_WIDHT, CELL_HEIGHT),
                columns,
                sprites,
            ),
        }
    }
}

impl CardAssets {
    async fn new() -> Self {
        let strike = load_texture("cards/strike.png")
            .await
            .expect("Failed to load cards/strike");
        let shield = load_texture("cards/shield.png")
            .await
            .expect("Failed to load cards/shield");

        Self { strike, shield }
    }

    pub fn texture(&self, id: CardId) -> &Texture2D {
        match id {
            CardId::Strike => &self.strike,
            CardId::Shield => &self.shield,
        }
    }
}

impl LevelAssets {
    pub async fn new() -> Self {
        let background = load_texture("level/1/background.png")
            .await
            .expect("Failed to load level/1/background");

        let enemy_lhama = load_texture("level/1/enemy_lhama.png")
            .await
            .expect("Failed to load level/1/enemy_lhama");

        let enemy_goblin = load_texture("level/1/enemy_goblin.png")
            .await
            .expect("Failed to load level/1/enemy_goblin");

        let cards = CardAssets::new().await;

        Self {
            background,
            enemy_lhama,
            enemy_goblin,
            cards,
        }
    }
}

impl Assets {
    pub async fn new() -> Self {
        let menu = MenuAssets::new().await;
        let desk = DeskAssets::new().await;
        let quests = QuestsAssets::new().await;
        let level = LevelAssets::new().await;

        Self {
            menu,
            desk,
            quests,
            level,
        }
    }
}
