mod card;
mod combat;
mod enemy;
mod player;
pub mod state;

use state::State;

pub struct Game {
    game_state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: State::Menu,
        }
    }

    pub fn update(&mut self) {
        match self.game_state {
            State::Menu => {}
        }
    }

    pub fn state(&self) -> &State {
        &self.game_state
    }
}
