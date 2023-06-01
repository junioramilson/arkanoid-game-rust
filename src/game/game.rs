use std::path::Prefix;

use super::{
    Ball, BallPlugin, Brick, GameHudPlugin, GameOverPlugin, Player, PlayerPlugin, WallPlugin,
};
use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
    PauseMenu,
}

#[derive(Default)]
pub struct UpdateScore;

#[derive(Default)]
pub struct UpdateHealth;

#[derive(Component, Default)]
pub struct Score(pub i32);

#[derive(Component, Default)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Ground {
    pub entity: Entity,
    pub size: Vec2,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<UpdateScore>()
            .add_event::<UpdateHealth>()
            .add_startup_system(initialize)
            .add_plugin(BallPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GameHudPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(WallPlugin)
            .add_system(process_global_input)
            .add_system(game_over.in_set(OnUpdate(GameState::Playing)))
            .add_system(ball_hit_bottom.in_set(OnUpdate(GameState::Playing)))
            .add_system(reset_score.in_schedule(OnEnter(GameState::Playing)))
            .add_system(reset_health.in_schedule(OnExit(GameState::Playing)))
            .add_system(ball_block_collision.in_set(OnUpdate(GameState::Playing)))
            .add_system(player_ball_collision.in_set(OnUpdate(GameState::Playing)));
    }
}

const BALL_INC_SPEED_FACTOR: f32 = 0.2;
const SCORE_POINT_FACTOR: i32 = 10;

fn initialize(
    mut commands: Commands,
    mut update_score_event: EventWriter<UpdateScore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Score(0));
    commands.spawn(Health(100));

    let window = window_query.get_single().unwrap();
    let window_width = window.width();

    let ground_rect_size = Vec2 {
        x: window_width,
        y: 25.,
    };

    let ground_rect = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(ground_rect_size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
            transform: Transform::from_translation(Vec3::new(0., -350., 0.)),
            ..Default::default()
        })
        .id();

    commands.entity(ground_rect).insert(Ground {
        entity: ground_rect,
        size: ground_rect_size,
    });

    update_score_event.send_default();
}

fn reset_score(
    mut score_query: Query<&mut Score>,
    mut update_score_event: EventWriter<UpdateScore>,
) {
    let mut score = score_query.get_single_mut().unwrap();

    score.0 = 0;

    update_score_event.send_default();
}

fn reset_health(
    mut health_query: Query<&mut Health>,
    mut update_health_event: EventWriter<UpdateHealth>,
) {
    let mut health = health_query.get_single_mut().unwrap();

    health.0 = 100;

    update_health_event.send_default();
}

fn process_global_input(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    match current_state.0 {
        GameState::GameOver => {
            if keyboard_input.pressed(KeyCode::Escape) {
                next_state.set(GameState::Playing);
            }
        }
        GameState::Playing => {}
        GameState::PauseMenu => {}
    }
}

fn game_over(mut state: ResMut<NextState<GameState>>, health_query: Query<&Health>) {
    let health = health_query.get_single().unwrap();

    if health.0 <= 0 {
        let _ = state.set(GameState::GameOver);
    }
}

fn ball_hit_bottom(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    mut ground_query: Query<(&Ground, &Transform)>,
    mut health_query: Query<&mut Health>,
    mut update_health_event: EventWriter<UpdateHealth>,
) {
    let (mut ball, ball_transform) = ball_query.get_single_mut().unwrap();
    let (ground, ground_transform) = ground_query.get_single_mut().unwrap();

    let collision = collide(
        ball_transform.translation,
        Vec2 {
            x: ball.get_default_radius(),
            y: ball.get_default_radius(),
        },
        ground_transform.translation,
        ground.size,
    );

    let mut health = health_query.get_single_mut().unwrap();

    if collision.is_some() {
        match collision.unwrap() {
            Collision::Left => {
                println!("Left");
            }
            Collision::Right => {
                println!("Right");
            }
            Collision::Top => {
                update_health_event.send_default();

                health.0 -= 10;

                println!("Health: {}", health.0);

                ball.speed += BALL_INC_SPEED_FACTOR;
                ball.direction.1 = 1;
            }
            Collision::Bottom => {
                println!("Bottom");
            }
            Collision::Inside => {
                println!("Inside");
            }
        }
    }
}

fn ball_block_collision(
    mut bricks_query: Query<(&mut Brick, &Transform)>,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    mut score_query: Query<&mut Score>,
    mut update_score_event: EventWriter<UpdateScore>,
) {
    let (mut ball, ball_transform) = ball_query.get_single_mut().unwrap();
    for (mut brick, brick_transform) in bricks_query.iter_mut() {
        let collision = collide(
            ball_transform.translation,
            Vec2 {
                x: ball.get_default_radius(),
                y: ball.get_default_radius(),
            },
            brick_transform.translation,
            brick.get_brick_size(),
        );

        if collision.is_some() {
            let mut score = score_query.get_single_mut().unwrap();
            score.0 += SCORE_POINT_FACTOR;

            update_score_event.send_default();

            brick.apply_damage(100.);

            match collision.unwrap() {
                Collision::Left => ball.direction.0 = -1,
                Collision::Right => ball.direction.0 = 1,
                Collision::Top => ball.direction.1 = 1,
                Collision::Bottom => ball.direction.1 = -1,
                Collision::Inside => (),
            }
        }
    }
}

fn player_ball_collision(
    player_query: Query<(&Player, &Transform)>,
    mut ball_query: Query<(&mut Ball, &Transform)>,
) {
    let (player, player_transform) = player_query.get_single().unwrap();
    let (mut ball, ball_transform) = ball_query.get_single_mut().unwrap();

    let collision = collide(
        player_transform.translation,
        player.get_default_size(),
        ball_transform.translation,
        Vec2 {
            x: ball.get_default_radius() * 2.,
            y: ball.get_default_radius() * 2.,
        },
    );

    if collision.is_some() {
        match collision.unwrap() {
            Collision::Top => ball.direction.1 = -1,
            Collision::Bottom => ball.direction.1 = 1,
            Collision::Left => ball.direction.0 = 1,
            Collision::Right => ball.direction.0 = -1,
            _ => (),
        }
    }
}
