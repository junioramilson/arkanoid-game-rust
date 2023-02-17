use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(process_player_input)
            .add_system(process_player_movement)
            .add_system(process_border_collision);
    }
}

#[derive(Component)]
struct Player {
    direction: Option<PlayerDirection>,
    can_move_left: bool,
    can_move_right: bool,
    boosting: bool
}

enum PlayerDirection {
    LEFT,
    RIGHT
}

const MOVEMENT_SPEED_BOOST: f32 = 2.;
const MOVEMENT_SPEED: f32 = 1.5;
const PLAYER_PADDLE_SIZE: Vec2 = Vec2 { x: 100., y: 20. };

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, windows: ResMut<Windows>) {
    let window = windows.get_primary().unwrap();

    let spawn_position = Vec3 {
        x: 0.,
        y: -(window.height() / 2.0) + 100.,
        z: 0.
    };

    let paddle_mesh = commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(PLAYER_PADDLE_SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(spawn_position),
        ..default()
    }).id();

    commands
    .entity(paddle_mesh)
        .insert(Player {
            direction: None,
            boosting: false,
            can_move_left: true,
            can_move_right: true
        })
        .insert(Name::new("PlayerPaddle"));
}

fn process_player_input(mut player_query: Query<&mut Player>, keyboard_input: Res<Input<KeyCode>>) {
    let mut player = player_query.single_mut();

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

fn process_player_movement(mut player_query: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = player_query.single_mut();
    
    let final_speed = match player.boosting {
        true => MOVEMENT_SPEED + MOVEMENT_SPEED_BOOST,
        false => MOVEMENT_SPEED,
    };

    match player.direction {
        None => {},
        Some(PlayerDirection::LEFT) => {
            transform.translation.x -= final_speed
        },
        Some(PlayerDirection::RIGHT) => {
            transform.translation.x += final_speed
        },
    }
}

fn process_border_collision(mut player_query: Query<(&mut Player, &Transform)>, windows: ResMut<Windows>) {
    let window = windows.get_primary().unwrap();
    let (mut player, transform) = player_query.single_mut();

    let limit_x = (window.width() / 2.0) - (PLAYER_PADDLE_SIZE.x / 2.);

    if transform.translation.x + 10. > limit_x {
        player.direction = None;
        player.can_move_right = false;
    } else if transform.translation.x - 10. <= -limit_x {
        player.direction = None;
        player.can_move_left = false;
    }
}