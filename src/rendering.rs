use crate::game::Game;
use crate::game::state::State;
use macroquad::prelude::*;

pub struct Renderer {}

impl Renderer {
    pub fn draw(&self, game: &Game) {
        clear_background(LIGHTGRAY);

        match game.state() {
            State::Menu => game.main_menu.draw(),
            State::Desk => {}
            State::Settings => {}
        }
    }
}
