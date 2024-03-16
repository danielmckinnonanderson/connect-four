use bevy::{prelude::*, log::LogPlugin};
use board::ConnectFourBoardPlugin;
use game::GameState;
use ui::ConnectFourUIPlugin;

mod board;
mod game;
mod ui;


#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    RunningGame(GameState),
    Paused(GameState),
    GameOver,
    PostGameMenu,
    Exiting,
}

fn main() {
    App::new()
        .insert_state(AppState::Loading)

        .add_plugins(
            DefaultPlugins
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,connect_four=debug".into(),
                level: bevy::log::Level::DEBUG,
                update_subscriber: None,
            })
        )
        .add_plugins(ConnectFourBoardPlugin)
        .add_plugins(ConnectFourUIPlugin)

        .run();
}

