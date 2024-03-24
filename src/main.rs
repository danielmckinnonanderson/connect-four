use bevy::{prelude::*, log::{LogPlugin, Level}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
mod game;
mod menu;
mod splash;

pub const WINDOW_WIDTH: f32 = 1290.;
pub const WINDOW_HEIGHT: f32 = 720.;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    BeginNewGame,
    WaitForSelection,
    EvaluateBoard,
    Draw,
    Winner,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,connect_four=debug".into(),
                level: Level::DEBUG,
                update_subscriber: None,
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: [WINDOW_WIDTH, WINDOW_HEIGHT].into(),
                    title: "Connect Four".to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(WorldInspectorPlugin::new())
        // Insert as resource the initial value for the settings resources
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<AppState>()
        .init_resource::<board::Board>()
        .add_plugins(splash::SplashPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_systems(Startup, setup)
        // When a new game begins, perform initial setup
        .add_systems(OnEnter(AppState::BeginNewGame), game::game_setup)
        // TODO - Is this even right?
        // Draw the board
        .add_systems(OnEnter(AppState::WaitForSelection), board::draw_board_system)
        // While waiting for input, listen for user to left-click
        .add_systems(Update,
            (board::handle_leftclick_input_system)
                .run_if(in_state(AppState::WaitForSelection)))
        // After the user has made an input & game has moved into
        //  "evaluate board" state, evaluate the board to determine next state
        //  (winner / draw, or next turn)
        .add_systems(OnEnter(AppState::EvaluateBoard), game::evaluate_end_condition_system)
        // If the last move did not end the game, advance to the next turn
        .add_systems(OnTransition {
            from: AppState::EvaluateBoard,
            to:   AppState::WaitForSelection
        }, game::advance_to_next_turn_system)


        // .add_plugins((
        //     splash::splash_plugin,
        //     menu::menu_plugin,
        //     game::game_plugin
        // ))
        .run();
}

fn setup(mut commands: Commands) {
    debug!("Running setup...");
    commands.spawn(Camera2dBundle::default());
    debug!("Setup complete!");
}


// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    debug!("Despawning screen...");
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
    debug!("Screen de-spawned!");
}

