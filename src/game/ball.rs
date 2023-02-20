use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::GameState;

pub struct BallPlugin;

pub const BALL_RADIUS: f32 = 10.;
const BALL_INIT_SPEED: f32 = 3.;
const BALL_INC_SPEED_FACTOR: f32 = 0.2;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball).add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_ball_movement)
                .with_system(update_ball_direction),
        );
    }
}

#[derive(Component)]
pub struct Ball {
    speed: f32,
    pub direction: (i32, i32), // TODO: Change it to enum
}

fn spawn_ball(
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

fn update_ball_direction(mut ball_query: Query<(&mut Ball, &Transform)>, windows: ResMut<Windows>) {
    let (mut ball, transform) = ball_query.single_mut();
    let window = windows.get_primary().unwrap();

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
        increase_ball_speed(&mut ball, BALL_INC_SPEED_FACTOR);
        ball.direction.1 = 1;
    }
}

fn update_ball_movement(mut ball_query: Query<(&Ball, &mut Transform)>) {
    let (ball, mut transform) = ball_query.single_mut();

    if ball.direction.0 == 1 {
        transform.translation.x = transform.translation.x + ball.speed;
    }

    if ball.direction.0 == -1 {
        transform.translation.x = transform.translation.x - ball.speed;
    }

    if ball.direction.1 == 1 {
        transform.translation.y = transform.translation.y + ball.speed;
    }

    if ball.direction.1 == -1 {
        transform.translation.y = transform.translation.y - ball.speed;
    }
}

fn increase_ball_speed(mut ball: &mut Ball, factor: f32) {
    ball.speed += factor;
}
