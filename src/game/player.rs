use super::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const MOVEMENT_SPEED_BOOST: f32 = 2.;
const MOVEMENT_SPEED: f32 = 1.5;
const PLAYER_PADDLE_SIZE: Vec2 = Vec2 { x: 100., y: 20. };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize)
            .add_systems((
                process_player_input.in_set(OnUpdate(GameState::Playing)),
                update_player_movement.in_set(OnUpdate(GameState::Playing)),
                process_player_collision.in_set(OnUpdate(GameState::Playing)),
            ))
            .add_system(reset.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component, Clone)]
pub struct Player {
    direction: Option<PlayerDirection>,
    can_move_left: bool,
    can_move_right: bool,
    boosting: bool,
}

impl Player {
    pub fn get_default_size(&self) -> Vec2 {
        PLAYER_PADDLE_SIZE
    }
}

#[derive(Clone)]
enum PlayerDirection {
    LEFT,
    RIGHT,
}

fn reset(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut window_query: Query<&mut Window>,
) {
    let window = window_query.get_single_mut().unwrap();

    let (mut player, mut player_transform) = player_query.get_single_mut().unwrap();

    player.boosting = false;
    player.can_move_left = true;
    player.can_move_right = true;
    player.direction = None;

    let spawn_position = Vec3 {
        x: 0.,
        y: -(window.height() / 2.0) + 100.,
        z: 0.,
    };

    *player_transform = Transform::from_translation(spawn_position);
}

fn initialize(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window_query: Query<&mut Window>,
) {
    let window = window_query.get_single_mut().unwrap();

    let spawn_position = Vec3 {
        x: 0.,
        y: -(window.height() / 2.0) + 100.,
        z: 0.,
    };

    let paddle_mesh = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(PLAYER_PADDLE_SIZE).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(spawn_position),
            ..default()
        })
        .id();

    commands
        .entity(paddle_mesh)
        .insert(Player {
            direction: None,
            boosting: false,
            can_move_left: true,
            can_move_right: true,
        })
        .insert(Name::new("PlayerPaddle"));
}

fn process_player_input(mut player_query: Query<&mut Player>, keyboard_input: Res<Input<KeyCode>>) {
    let mut player = player_query.get_single_mut().unwrap();

    player.boosting = if keyboard_input.pressed(KeyCode::LShift) {
        true
    } else {
        false
    };

    if keyboard_input.pressed(KeyCode::A) && player.can_move_left {
        player.direction = Some(PlayerDirection::LEFT);
        player.can_move_right = true;
    }

    if keyboard_input.pressed(KeyCode::D) && player.can_move_right {
        player.direction = Some(PlayerDirection::RIGHT);
        player.can_move_left = true;
    }
}

fn update_player_movement(mut player_query: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = player_query.get_single_mut().unwrap();

    let final_speed = match player.boosting {
        true => MOVEMENT_SPEED + MOVEMENT_SPEED_BOOST,
        false => MOVEMENT_SPEED,
    };

    match player.direction {
        None => {}
        Some(PlayerDirection::LEFT) => transform.translation.x -= final_speed,
        Some(PlayerDirection::RIGHT) => transform.translation.x += final_speed,
    }
}

fn process_player_collision(
    mut player_query: Query<(&mut Player, &Transform)>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    let (mut player, transform) = player_query.get_single_mut().unwrap();

    let limit_x = (window.width() / 2.0) - (PLAYER_PADDLE_SIZE.x / 2.);

    if transform.translation.x + 10. > limit_x {
        player.direction = None;
        player.can_move_right = false;
        player.can_move_left = true;
    } else if transform.translation.x - 10. <= -limit_x {
        player.direction = None;
        player.can_move_left = false;
        player.can_move_right = true;
    }
}
