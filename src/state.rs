use crate::assets::Assets;
use crate::desk::Desk;
use crate::input::Input;
use crate::level::Level;
use crate::menu::Menu;
use crate::quests::Quests;

pub enum Transition {
    Quit,
    Menu,
    Desk,
    Quests,
    Levels(usize),
}

pub enum State {
    Menu(Menu),
    Desk(Desk),
    Quests(Quests),
    Playing(Level),
}

impl State {
    pub fn draw(&self, assets: &Assets) {
        match self {
            State::Menu(menu) => menu.draw(&assets.menu),
            State::Desk(desk) => desk.draw(&assets.desk),
            State::Quests(quests) => quests.draw(&assets.quests),
            State::Playing(level) => level.draw(&assets.level),
        }
    }

    pub fn update(&mut self, input: &Input) -> Option<Transition> {
        match self {
            State::Menu(menu) => menu.update(input),
            State::Desk(desk) => desk.update(input),
            State::Quests(quests) => quests.update(input),
            State::Playing(level) => level.update(input),
        }
    }
}
