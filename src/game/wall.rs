use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::GameState;

const DEFAULT_BRICK_SIZE: Vec2 = Vec2 { x: 50., y: 20. };

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(initialize.in_schedule(OnEnter(GameState::Playing)))
            .add_system(reset.in_schedule(OnExit(GameState::Playing)))
            .add_system(bricks_health_check.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Brick {
    health: f32,
    pub entity: Entity,
}

impl Brick {
    pub fn apply_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    pub fn get_brick_size(&self) -> Vec2 {
        DEFAULT_BRICK_SIZE
    }
}

fn bricks_health_check(bricks_query: Query<&Brick>, mut commands: Commands,) {
    for brick in bricks_query.iter() {
        if brick.health <= 0. {
            commands.entity(brick.entity).despawn_recursive();
        }
    }
}

fn reset(
    bricks_query: Query<&Brick>,
    mut commands: Commands,
) {
    for brick in bricks_query.iter() {
        commands.entity(brick.entity).despawn_recursive();
    }
}

fn initialize(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    let bricks_matrix = vec![vec![1; 19]; 5];

    let first_brick_pos = Vec3 {
        x: -(window.width() / 2.) + 55.,
        y: (window.height() / 2.) - 40.,
        ..Default::default()
    };

    let mut prev_brick_pos = first_brick_pos;
    for row in bricks_matrix.iter() {
        prev_brick_pos = Vec3 {
            y: prev_brick_pos.y + (DEFAULT_BRICK_SIZE.y / 2.) - 45.,
            ..first_brick_pos
        };
        for (index, _elem) in row.iter().enumerate() {
            let brick_pos = Vec3 {
                x: if index == 0 {
                    first_brick_pos.x
                } else {
                    prev_brick_pos = Vec3 {
                        x: prev_brick_pos.x + (DEFAULT_BRICK_SIZE.x / 2.) + 40.,
                        ..prev_brick_pos
                    };
                    prev_brick_pos.x
                },
                y: prev_brick_pos.y,
                z: 0.,
            };

            let brick_mesh = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(DEFAULT_BRICK_SIZE).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(brick_pos),
                    ..Default::default()
                })
                .id();

            commands
                .entity(brick_mesh)
                .insert(Brick {
                    health: 100.,
                    entity: brick_mesh,
                })
                .insert(Name::new(format!("Brick-{:?}", index)));
        }
    }
}
