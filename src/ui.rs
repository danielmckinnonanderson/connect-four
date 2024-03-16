use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};

use crate::board::Board;

// Listen for all of the relevant events in our game and draw
// their representations to the screen
pub struct ConnectFourUIPlugin;

#[derive(Component)]
pub struct MainCameraMarker;

#[derive(Resource, Default)]
struct WorldCoords(Vec2);

impl Plugin for ConnectFourUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            // Mouse coords in world space
            .init_resource::<WorldCoords>()

            // Only run app when there is user input
            .insert_resource(WinitSettings::desktop_app())
            .add_systems(
                Update,
                draw_board
                    .run_if(resource_exists_and_changed::<Board>
                        .and_then(resource_exists::<AssetHandles>)
                    )
            )
            .add_systems(
                Update,
                track_cursor_in_worldspace_system
                .run_if(resource_exists_and_changed::<WorldCoords>)
            );
    }
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash)]
pub struct AssetHandles {
    pub tile_grey: Handle<Image>,

    pub token_red: Handle<Image>,
    pub pointer_red: Handle<Image>,

    pub token_blue: Handle<Image>,
    pub pointer_blue: Handle<Image>,
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    debug!("Setting up UI");
    let asset_handles = AssetHandles {
        tile_grey: asset_server.load("tile_grey.png"),

        token_red: asset_server.load("red_body_rhombus.png"),
        pointer_red: asset_server.load("red_hand_point.png"),

        token_blue: asset_server.load("blue_body_squircle.png"),
        pointer_blue: asset_server.load("blue_hand_point.png"),
    };

    commands.insert_resource(asset_handles.clone());
    commands.spawn(Camera2dBundle::default());
    debug!("UI setup complete");
}

fn draw_board(
    mut commands: Commands,
    board_res: Res<Board>,
    asset_handles: Res<AssetHandles>
) {
    debug!("Drawing board...");

    for (position, _) in &board_res.board {
        commands.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::from_array([-500.0, 0.0, 0.0]),
                ..default()
            },
            texture: asset_handles.tile_grey.clone(),
            ..default()
        });
    }

    debug!("Board drawn");
}

// Tracking cursor in worldspace example from
//  https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games
fn track_cursor_in_worldspace_system(
    mut coords: ResMut<WorldCoords>,
    // Get window to read cursor position
    q_window: Query<&Window, With<PrimaryWindow>>,
    // Get camera's transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCameraMarker>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // Check if cursor is inside the window and get its coords
    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_pos;
        // TODO - Do something with this information
        debug!("World coords: {}/{}", world_pos.x, world_pos.y);
    }
}

