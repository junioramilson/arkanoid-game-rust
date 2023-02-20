use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::{GameState, UpdateScore, Score};

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct FpsText;

pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_hud)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_score_event)
                    .with_system(update_fps),
            );
    }
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
        ]) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
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
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("FPS: ", styled_text(20.)),
            TextSection::from_style(styled_text(20.)),
        ])
        .with_text_alignment(TextAlignment::TOP_CENTER)
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
    ));
}

fn update_score_event(
    mut event: EventReader<UpdateScore>,
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
