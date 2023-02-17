use bevy::{prelude::*};

mod ball;
mod player;

pub use player::PlayerPlugin;
pub use ball::BallPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Test".to_string(),
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(BallPlugin)
        .add_plugin(PlayerPlugin)
        .add_system(show_mouse_pos_info)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn show_mouse_pos_info(mut cursor_moved_events: EventReader<CursorMoved>,) {
    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }
}


