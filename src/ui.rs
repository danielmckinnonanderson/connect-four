use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings, render::color};

use crate::board::Board;

// Menu code taken from Bevy example github.com/bevyengine/bevy/blob/main/examples/games/game_menu.rs
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


// Listen for all of the relevant events in our game and draw
// their representations to the screen
pub struct ConnectFourUIPlugin;

#[derive(Component)]
pub struct MainCameraMarker;

#[derive(Component)]
pub enum MainMenuOption {
    BeginNewGame,
    Quit,
}

#[derive(Component)]
pub enum MenuButtonAction {
    BeginNewGame,
    Quit,
}

#[derive(Component)]
struct SelectedOption;

#[derive(Resource, Default)]
struct WorldCoords(Vec2);


/// Update the appearance of the button based on mouse interaction
fn main_menu_button_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
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

pub fn setup_ui(
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

#[derive(Component)]
pub struct MainMenuUIElement;

pub fn draw_main_menu(
    mut commands: Commands,
    _asset_handles: Res<AssetHandles>,
) {
    let button_style = Style {
        width: Val::Px(320.0),
        height: Val::Px(64.0),
        margin: UiRect::all(Val::Px(16.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    };

    commands.spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        }, MainMenuUIElement))
        .with_children(|main_menu_root| {
            main_menu_root.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|main_menu_root| {
                    main_menu_root.spawn(TextBundle::from_section(
                        "Connect Four",
                        TextStyle {
                            font_size: 80.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ).with_style(Style {
                            margin: UiRect::all(Val::Px(64.0)),
                            ..default()
                        })
                    );
                });

            // Main menu buttons
            main_menu_root.spawn((
                // Begin new game button
                ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                },
                MainMenuOption::BeginNewGame,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "New Game",
                    button_text_style.clone()
                ));
            });
        });
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

