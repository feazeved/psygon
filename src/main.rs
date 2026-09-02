mod assets;
mod combat;
mod desk;
mod entities;
mod game;
mod input;
mod layout;
mod level;
mod level_data;
mod menu;
mod quests;
mod renderer;
mod sprite_sheet;
mod state;
mod ui;

use assets::Assets;
use game::Game;
use input::Input;
use layout::Layout;
use macroquad::{prelude::*, rand::srand};
use renderer::Renderer;

fn conf() -> Conf {
    Conf {
        window_title: "Psygon!".to_string(),
        fullscreen: false,
        window_resizable: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    macroquad::file::set_pc_assets_folder("assets");

    srand(2147483647);
    let renderer = Renderer::new();
    let assets = Assets::new().await;
    let layout = Layout::new();
    let mut game = Game::new(assets, layout);
    let mut input = Input::new();

    loop {
        input.update(game.layout());
        game.update(&input);

        if game.should_stop() {
            break;
        }

        renderer.begin();
        clear_background(BLACK);
        game.draw();
        renderer.end();

        next_frame().await;
    }
}
