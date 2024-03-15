use bevy::prelude::*;

// Listen for all of the relevant events in our game and draw
// their representations to the screen

pub struct ConnectFourUIPlugin;

impl Plugin for ConnectFourUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
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
    commands.spawn(SpriteBundle {
        texture: asset_handles.pointer_blue.clone(),
        ..Default::default()
    });
    debug!("UI setup complete");
}


