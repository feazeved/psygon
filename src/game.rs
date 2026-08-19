mod card;
mod combat;
mod enemy;
mod player;
pub mod state;

use state::State;

use crate::{
    button::{ButtonAction::Play, ButtonAction::Quit, ButtonAction::Settings},
    menu::Menu,
};

pub struct Game {
    pub main_menu: Menu,
    game_state: State,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            main_menu: Menu::new().await,
            game_state: State::Menu,
        }
    }

    pub fn state(&self) -> &State {
        &self.game_state
    }

    pub fn update(&mut self) {
        match self.game_state {
            State::Menu => self.update_menu(),
            State::Desk => {}
            State::Settings => {}
        }
    }

    fn update_menu(&mut self) {
        match self.main_menu.update() {
            Some(Play) => self.game_state = State::Desk,
            Some(Settings) => self.game_state = State::Settings,
            Some(Quit) => std::process::exit(0),
            None => (),
        }
    }
}
