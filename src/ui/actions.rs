#[derive(Clone, Copy)]
pub enum MenuAction {
    Play,
    Settings,
    Quit,
}

#[derive(Clone, Copy)]
pub enum DeskAction {
    Quests,
}

#[derive(Clone, Copy)]
pub enum QuestAction {
    Level(usize),
}

#[derive(Clone, Copy)]
pub enum LevelAction {
    Enemy,
    EndTurn,
}

#[derive(Clone, Copy)]
pub enum ButtonAction {
    Menu(MenuAction),
    Quest(QuestAction),
    Desk(DeskAction),
    Level(LevelAction),
}
