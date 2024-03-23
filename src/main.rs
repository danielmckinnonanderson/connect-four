use bevy::{prelude::*, log::{LogPlugin, Level}};

mod board;
mod game;
mod menu;
mod splash;

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
        )
        // Insert as resource the initial value for the settings resources
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<AppState>()
        .init_resource::<board::Board>()
        .add_plugins(splash::SplashPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::BeginNewGame), game::game_setup)
        .add_systems(OnEnter(AppState::WaitForSelection), || {}) // TODO
        .add_systems(OnEnter(AppState::EvaluateBoard), game::evaluate_end_condition_system)
        .add_systems(OnTransition {
            from: AppState::EvaluateBoard,
            to:   AppState::WaitForSelection
        }, game::advance_to_next_turn_system)
        .add_systems(Update,
            (board::draw_board_system)
                .run_if(in_state(AppState::WaitForSelection))
        )

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

