use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::{GameState, Health, Score, UpdateScore};

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct GameplayHud;

#[derive(Component)]
struct GameOverHud;

#[derive(Component)]
struct HealthText;

pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_systems((
                setup_hud.in_schedule(OnEnter(GameState::Playing)),
                update_score_event.in_set(OnUpdate(GameState::Playing)),
                update_fps.in_set(OnUpdate(GameState::Playing)),
                update_health.in_set(OnUpdate(GameState::Playing)),
            ))
            .add_system(despawn_screen::<GameplayHud>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameOverState {
    Show,
    #[default]
    Hide,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameOverState>()
            .add_system(setup_game_over.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(despawn_screen::<GameOverHud>.in_schedule(OnExit(GameState::GameOver)));
    }
}

fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query_score: Query<&Score>,
) {
    let score = query_score.get_single_mut();
    let styled_game_over_text = |font_size: f32| TextStyle {
        font: asset_server.load("fonts/AtariST8x16SystemFont.ttf"),
        font_size,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                format!("Game Over\nFinal score: {:?}\n\n", score.unwrap().0),
                styled_game_over_text(45.),
            ),
            TextSection::new("Press ESC to restart the game", styled_game_over_text(15.)),
            TextSection::from_style(styled_game_over_text(35.)),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            position: UiRect {
                left: Val::Percent(38.),
                ..Default::default()
            },
            padding: UiRect::all(Val::Px(35.)),
            ..default()
        }),
        GameOverText,
        GameOverHud,
    ));
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let styled_text = |font_size: f32| TextStyle {
        font: asset_server.load("fonts/AtariST8x16SystemFont.ttf"),
        font_size,
        color: Color::WHITE,
    };
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Score: ", styled_text(45.)),
            TextSection::from_style(styled_text(45.)),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreText,
        GameplayHud,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Health: ", styled_text(45.)),
            TextSection::from_style(styled_text(45.)),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(45.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        HealthText,
        GameplayHud,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("FPS: ", styled_text(20.)),
            TextSection::from_style(styled_text(20.)),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
        GameplayHud,
    ));
}

fn update_health(mut query: Query<&mut Text, With<HealthText>>, mut query_health: Query<&Health>) {
    let health = query_health.single_mut();
    for mut text in &mut query {
        text.sections[1].value = format!("{}", health.0);
    }
}

fn update_score_event(
    mut _event: EventReader<UpdateScore>,
    mut query: Query<&mut Text, With<ScoreText>>,
    mut query_score: Query<&Score>,
) {
    let score = query_score.single_mut();

    for mut text in &mut query {
        text.sections[1].value = format!("{}", score.0);
    }
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{:.2}", value);
            }
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
