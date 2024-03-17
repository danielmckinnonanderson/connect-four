
#[derive(Event)]
pub enum AppCommand {
    StartGame,
    MakeMove(usize),
    PauseGame,
    ResumeGame,
    QuitGame,
}

