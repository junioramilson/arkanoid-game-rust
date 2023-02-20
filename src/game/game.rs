use bevy::prelude::*;

use super::{BallPlugin, GameHudPlugin, PlayerPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
    PauseMenu,
}

#[derive(Default)]
pub struct UpdateScore;

#[derive(Component, Default)]
pub struct Score(pub i32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Playing)
            .add_event::<UpdateScore>()
            .add_startup_system(setup)
            .add_plugin(BallPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GameHudPlugin)
            .add_system(process_global_input);
    }
}

fn setup(mut commands: Commands, mut update_score_event: EventWriter<UpdateScore>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Score(0));

    update_score_event.send_default();
}

fn process_global_input(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let _ = state.overwrite_set(GameState::PauseMenu);
    }
}
