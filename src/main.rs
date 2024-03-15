use bevy::prelude::*;
use game::TurnBasedGameplayPlugin;
use ui::ConnectFourUIPlugin;

mod board;
mod game;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TurnBasedGameplayPlugin)
        .add_plugins(ConnectFourUIPlugin)
        .run();
}

