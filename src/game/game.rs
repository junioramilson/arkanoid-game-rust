use bevy::prelude::*;
use super::{Ball, BallPlugin, GameHudPlugin, PlayerPlugin, BALL_RADIUS, GameOverPlugin};

const BALL_INC_SPEED_FACTOR: f32 = 0.2;

#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum GameState {
    #[default]
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
        app.add_state::<GameState>()
            .add_event::<UpdateScore>()
            .add_startup_system(setup)
            .add_plugin(BallPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GameHudPlugin)
            .add_plugin(GameOverPlugin)
            .add_system(process_global_input)
            .add_system(game_over.in_set(OnUpdate(GameState::Playing)))
            .add_system(ball_hit_bottom.in_set(OnUpdate(GameState::Playing)))
            .add_system(reset.in_schedule(OnExit(GameState::Playing)));
    }
}

fn reset(mut score_query: Query<&mut Score>, mut update_score_event: EventWriter<UpdateScore>) {
    let mut score = score_query.get_single_mut().unwrap();
    
    score.0 = 0;

    update_score_event.send_default();
}

fn setup(mut commands: Commands, mut update_score_event: EventWriter<UpdateScore>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Score(0));

    update_score_event.send_default();
}

fn process_global_input(mut next_state: ResMut<NextState<GameState>>, current_state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    match current_state.0 {
        GameState::GameOver => {
            if keyboard_input.pressed(KeyCode::Escape) {
                next_state.set(GameState::Playing);
            }
        }
        GameState::Playing => {},
        GameState::PauseMenu => {},
    }
}

fn game_over(mut ball_query: Query<&Ball>, mut state: ResMut<NextState<GameState>>, score_query: Query<&Score>) {
    let ball = ball_query.single_mut();
    let score = score_query.get_single().unwrap();

    if ball.speed > 10. || score.0 < 0 {
        let _ = state.set(GameState::GameOver);
    }
}

fn ball_hit_bottom(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    mut score_query: Query<&mut Score>,
    mut update_score_event: EventWriter<UpdateScore>,
    window_query: Query<&Window>,
) {
    let (mut ball, ball_transform) = ball_query.get_single_mut().unwrap();
    let window = window_query.get_single().unwrap();
    let limit_y = (window.height() / 2.0) - BALL_RADIUS;
    let mut score = score_query.get_single_mut().unwrap();

    if ball_transform.translation.y <= -limit_y {
        update_score_event.send_default();
        score.0 -= 1;

        ball.speed += BALL_INC_SPEED_FACTOR;
    }
}
