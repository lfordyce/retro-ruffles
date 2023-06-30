use crate::clock::TimeSinceLevelStart;
use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub struct Counter;

#[derive(Component, Default, Clone, Debug)]
pub struct GameUiClock;

#[derive(Component, Default, Clone, Debug)]
pub struct GameUiScore;

#[derive(Resource, Default)]
pub struct Score(pub(crate) f32);

#[derive(Component)]
struct ScoreText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_clock, spawn_score).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    update_counter,
                    update_score_text,
                    // death_screen,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(cleanup.in_schedule(OnExit(GameState::Playing)));
    }
}

fn spawn_clock(mut commands: Commands, font_assets: Res<FontAssets>, assets: Res<TextureAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(75.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::Rgba {
                red: 0.098,
                green: 0.078,
                blue: 0.169,
                alpha: 1.,
            }),
            ..Default::default()
        })
        .insert(GameUiClock::default())
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        "00:00",
                        TextStyle {
                            font: font_assets.pico.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Counter);

            // Clock Counter
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(8. * 5.), Val::Px(8. * 5.)),
                    ..Default::default()
                },
                image: assets.clock.clone().into(),
                ..Default::default()
            });
        });
}

fn spawn_score(mut commands: Commands, font_assets: Res<FontAssets>, assets: Res<TextureAssets>) {
    commands.init_resource::<Score>();
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(75.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::Rgba {
                red: 0.098,
                green: 0.078,
                blue: 0.169,
                alpha: 1.,
            }),
            ..Default::default()
        })
        .insert(GameUiScore::default())
        .with_children(|parent| {
            // Heart icon score
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(8. * 5.), Val::Px(8. * 5.)),
                    ..Default::default()
                },
                image: assets.half_heart.clone().into(),
                ..Default::default()
            });
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "0/5".to_string(),
                            style: TextStyle {
                                font: font_assets.pico.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

fn update_score_text(score: Res<Score>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }
    score_text.single_mut().sections[0].value = format!("{:.0}/5", score.0);
}

fn update_counter(
    mut timer_ui: Query<&mut Text, With<Counter>>,
    time_since_level_start: Res<TimeSinceLevelStart>,
    asset_holder: Res<FontAssets>,
) {
    for mut text in timer_ui.iter_mut() {
        let remaining = 120. - time_since_level_start.0;
        let minutes = (remaining / 60.0) as u32;
        let seconds = (remaining % 60.0) as u32;
        let color = if remaining <= 31.0 {
            if seconds % 2 == 0 {
                Color::hex("FF004D").unwrap()
            } else {
                Color::WHITE
            }
        } else {
            Color::WHITE
        };

        *text = Text::from_section(
            format!("{:0>2}:{:0>2}", minutes, seconds),
            TextStyle {
                font: asset_holder.pico.clone(),
                font_size: 32.,
                color,
            },
        );
    }
}

fn cleanup(
    mut commands: Commands,
    clock_ui_entity: Query<Entity, With<GameUiClock>>,
    score_ui_entity: Query<Entity, With<GameUiScore>>,
) {
    for entity in clock_ui_entity.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in score_ui_entity.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
