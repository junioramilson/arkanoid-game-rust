pub mod game;

use bevy::{prelude::*};
use game::GamePlugin;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Test".to_string(),
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}

