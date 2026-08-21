mod card;
mod combat;
mod enemy;
mod player;
pub mod state;

use macroquad::rand::rand;
use state::State;

use crate::{
    button::{ButtonAction::Play, ButtonAction::Quit, ButtonAction::Settings},
    desk::{Desk, DeskAction::Main, DeskAction::Start},
    menu::Menu,
};

pub struct Game {
    pub main_menu: Menu,
    pub desk: Desk,
    game_state: State,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            main_menu: Menu::new().await,
            desk: Desk::new().await,
            game_state: State::Menu,
        }
    }

    pub fn state(&self) -> &State {
        &self.game_state
    }

    pub fn update(&mut self) {
        match self.game_state {
            State::Menu => self.update_menu(),
            State::Desk => self.update_desk(),
            State::Quests => (),
            State::Settings => {}
        }
    }

    fn update_desk(&mut self) {
        match self.desk.update() {
            Some(Main) => self.game_state = State::Menu,
            Some(Start) => self.game_state = State::Quests,
            None => (),
        }
    }

    fn update_menu(&mut self) {
        match self.main_menu.update() {
            Some(Play) => {
                self.game_state = State::Desk;
                self.desk.which = rand();
            }
            Some(Settings) => self.game_state = State::Settings,
            Some(Quit) => std::process::exit(0),
            _ => (),
        }
    }
}
