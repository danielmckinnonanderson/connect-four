use bevy::{prelude::*, log::LogPlugin, render::camera::ScalingMode};
use board::ConnectFourBoardPlugin;
use game::{GameState, TurnBasedGameplayPlugin};
use ui::ConnectFourUIPlugin;

mod board;
mod game;
mod ui;


#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    InitialLoading,
    MainMenu,
    RunningGame(GameState),
    Paused(GameState),
    GameOver,
    PostGameMenu,
    Exiting,
}

fn main() {
    App::new()
        .insert_state(AppState::InitialLoading)

        .add_plugins(
            DefaultPlugins
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,connect_four=debug".into(),
                level: bevy::log::Level::DEBUG,
                update_subscriber: None,
            })
        )
        .add_plugins(TurnBasedGameplayPlugin)
        .add_plugins(ConnectFourBoardPlugin)
        .add_plugins(ConnectFourUIPlugin)

        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }.into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );
}

