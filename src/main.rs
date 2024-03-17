use bevy::{prelude::*, log::LogPlugin, render::camera::ScalingMode};
use board::ConnectFourBoardPlugin;
use game::{GameState, TurnBasedGameplayPlugin};
use ui::{ConnectFourUIPlugin, setup_ui, draw_main_menu, AssetHandles};

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
        .init_state::<AppState>()

        .add_plugins(
            DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,connect_four=debug".into(),
                level: bevy::log::Level::DEBUG,
                update_subscriber: None,
            })
        )

        // Entering loading state will cause us to load our assets
        .add_systems(
            OnEnter(AppState::InitialLoading),
            setup_ui,
        )

        // After loading, transition to MainMenu state exactly once
        .add_systems(
            OnEnter(AppState::MainMenu),
            enter_main_menu_state_system
            .run_if(
                resource_exists::<AssetHandles>
                .and_then(in_state(AppState::InitialLoading))
            )
        )

        // Entering Main Menu state will show the main menu on screen
        //  while the player chooses an option
        .add_systems(
            OnEnter(AppState::MainMenu),
            (
                draw_main_menu
            ).chain()
        )

        .run()
}

fn enter_main_menu_state_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>
) {
    debug!("Enter main menu state system: Setting next state to AppState::MainMenu");
    next_state.set(AppState::MainMenu);
}

