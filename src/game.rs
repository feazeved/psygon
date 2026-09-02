use crate::assets::Assets;
use crate::desk::Desk;
use crate::input::Input;
use crate::layout::Layout;
use crate::level::Level;
use crate::menu::Menu;
use crate::quests::Quests;
use crate::state::{State, Transition};

pub struct Game {
    state: State,
    assets: Assets,
    layout: Layout,

    stop: bool,
}

impl Game {
    pub fn new(assets: Assets, layout: Layout) -> Self {
        Self {
            state: State::Menu(Menu::new(&layout)),
            assets: assets,
            layout: layout,
            stop: false,
        }
    }

    pub fn update(&mut self, input: &Input) {
        if let Some(transition) = self.state.update(input) {
            self.transition(transition);
        }
    }

    pub fn draw(&self) {
        self.state.draw(&self.assets);
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn should_stop(&self) -> bool {
        self.stop
    }

    fn transition(&mut self, transition: Transition) {
        match transition {
            Transition::Quit => self.stop = true,
            Transition::Desk => self.state = State::Desk(Desk::new(&self.layout)),
            Transition::Menu => self.state = State::Menu(Menu::new(&self.layout)),
            Transition::Quests => {
                self.state = State::Quests(Quests::new(&self.layout, &self.assets.quests))
            }
            Transition::Levels(n) => {
                self.state = State::Playing(Level::new(n, &self.layout, &self.assets.level))
            }
        }
    }
}
