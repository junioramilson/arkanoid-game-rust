use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::GameState;

pub struct BallPlugin;

const BALL_RADIUS: f32 = 10.;
const BALL_INIT_SPEED: f32 = 3.;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize).add_systems((
            update_ball_movement.in_set(OnUpdate(GameState::Playing)),
            update_ball_direction.in_set(OnUpdate(GameState::Playing)),
            reset.in_schedule(OnExit(GameState::Playing)),
        ));
    }
}

#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub direction: (i32, i32),
}

impl Ball {
    pub fn get_default_radius(&self) -> f32 {
        BALL_RADIUS
    }
}

fn reset(mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    let (mut ball, mut ball_transform) = ball_query.get_single_mut().unwrap();

    ball.speed = BALL_INIT_SPEED;
    ball.direction = (1, 1);

    *ball_transform = Transform::from_translation(Vec3::ZERO);
}

fn initialize(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball_mesh = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        })
        .id();

    commands
        .entity(ball_mesh)
        .insert(Ball {
            speed: BALL_INIT_SPEED,
            direction: (1, 1),
        })
        .insert(Name::new("BouncingBall"));
}

fn update_ball_direction(mut ball_query: Query<(&mut Ball, &Transform)>, windows: Query<&Window>) {
    let (mut ball, transform) = ball_query.single_mut();
    let window = windows.get_single().unwrap();

    let limit_x = (window.width() / 2.0) - BALL_RADIUS;
    let limit_y = (window.height() / 2.0) - BALL_RADIUS;

    if transform.translation.x >= limit_x {
        ball.direction.0 = -1;
    } else if transform.translation.x <= -limit_x {
        ball.direction.0 = 1;
    }

    if transform.translation.y >= limit_y {
        ball.direction.1 = -1;
    } else if transform.translation.y <= -limit_y {
        ball.direction.1 = 1;
    }
}

fn update_ball_movement(mut ball_query: Query<(&Ball, &mut Transform)>) {
    let (ball, mut transform) = ball_query.single_mut();

    match ball.direction.0 {
        1 => transform.translation.x = transform.translation.x + ball.speed,
        -1 => transform.translation.x = transform.translation.x - ball.speed,
        _ => ()
    }

    match ball.direction.1 {
        1 => transform.translation.y = transform.translation.y + ball.speed,
        -1 => transform.translation.y = transform.translation.y - ball.speed,
        _ => ()
    }
}
