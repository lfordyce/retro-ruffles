use crate::loading::FontAssets;
use crate::menu::LevelStart;
use crate::ui::Score;
use crate::{GameState, LevelState};
use bevy::prelude::{BuildChildren, JustifyContent, Val};
use bevy::text::Text;
use bevy::ui::{AlignContent, AlignItems, BackgroundColor};
use bevy::{
    prelude::{
        default, Color, Commands, Component, DespawnRecursiveExt, Entity, EventWriter, Input,
        KeyCode, NextState, NodeBundle, Query, Res, ResMut, TextBundle, With,
    },
    text::{TextAlignment, TextStyle},
    ui::{PositionType, Size, Style},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub struct GameOverScreen;

pub fn game_over_screen(
    mut commands: Commands,
    existing_game_over_screens: Query<Entity, With<GameOverScreen>>,
    asset_holder: Res<FontAssets>,
) {
    for entity in existing_game_over_screens.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0., 0., 0., 1.)),
            ..default()
        })
        .insert(GameOverScreen)
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    "GAME OVER\n\n[Press Space to Restart]",
                    TextStyle {
                        font: asset_holder.crt_font.clone(),
                        font_size: 128.,
                        color: Color::RED,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
        });
}

pub fn reset_score(mut score: ResMut<Score>) {
    *score = Score::default();
}

pub fn game_over_start_new_game(
    keyboard: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<GameState>>,
    mut next_story_state: ResMut<NextState<LevelState>>,
    mut level_start_events: EventWriter<LevelStart>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_app_state.set(GameState::Playing);
        next_story_state.set(LevelState::OverWorld);
        level_start_events.send(LevelStart);
    }
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
