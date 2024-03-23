use bevy::prelude::*;

use super::{despawn_screen, AppState};

pub struct SplashPlugin;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(AppState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(AppState::Splash), despawn_screen::<OnSplashScreen>);
    }
}
// Tag component used to tag entities added on the splash screen
#[derive(Component)]
pub struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn splash_setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>
) {
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    align_content: AlignContent::Center,
                    ..default()
                },
                text: Text::from_section(
                    "Splash",
                    TextStyle {
                        font_size: 64.0,
                        ..default()
                    }
                ),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
pub fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        debug!("Splash timer is done! Progressing to menu...");
        game_state.set(AppState::Menu);
    }
}
