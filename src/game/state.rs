/// The main enum present in Game. Responsible to determine
/// the context of the game. If another State is added, game.update
/// and rendering.draw() should be changed!
pub enum State {
    Menu,
    Desk,
    Settings,
}
