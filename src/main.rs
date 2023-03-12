pub mod game;

use bevy::{prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     window: WindowDescriptor {
        //         title: "Arkanoid Game".to_string(),
        //         resizable: false,
        //         fit_canvas_to_parent: true,
        //         ..Default::default()
        //     },
        //     ..default()
        // }))
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(GamePlugin)
        .run();
}

