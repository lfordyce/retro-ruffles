use crate::clock::TimeSinceLevelStart;
use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub struct Counter;

const MAX_UI_DIGITS: usize = 4;

#[derive(Component, Default, Clone, Debug)]
pub struct GameUi;

#[derive(Component, Default, Clone, Debug)]
pub struct UiElementIndex(usize);

#[derive(Component, Default, Clone, Debug)]
pub struct HealthContainerImage;

#[derive(Component, Default, Clone, Debug)]
pub struct BorkPointNumber;

#[derive(Component, Default, Clone, Debug)]
pub struct CoinNumber;

pub const WINDOW_SCALE: f32 = 4.;
pub const PLAYER_MAX_HEALTH: u32 = 3;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_ui, spawn_counter).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    update_counter,
                    update_health_containers,
                    update_potion_counter,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(cleanup.in_schedule(OnExit(GameState::Playing)));
    }
}

fn spawn_counter(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(2.),
                    left: Val::Percent(50.),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                "Counter",
                TextStyle {
                    font: font_assets.fira_sans.clone(),
                    font_size: 64.,
                    color: Color::BLACK,
                },
            ),
            ..default()
        })
        .insert(Counter);
}

fn spawn_ui(mut commands: Commands, assets: Res<TextureAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameUi::default())
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Px(10. * WINDOW_SCALE)),
                        padding: UiRect {
                            left: Val::Px(1. * WINDOW_SCALE),
                            right: Val::Px(1. * WINDOW_SCALE),
                            top: Val::Px(1. * WINDOW_SCALE),
                            bottom: Val::Px(1. * WINDOW_SCALE),
                        },
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.098, 0.078, 0.169).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Hearth Points Counter
                    for index in 1..=PLAYER_MAX_HEALTH {
                        parent
                            .spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(8. * WINDOW_SCALE),
                                        Val::Px(8. * WINDOW_SCALE),
                                    ),
                                    ..Default::default()
                                },
                                image: assets.heart.clone().into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(index as usize))
                            .insert(HealthContainerImage::default());
                    }

                    // Bork Points Counter
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(8. * WINDOW_SCALE), Val::Px(8. * WINDOW_SCALE)),
                            ..Default::default()
                        },
                        image: assets.potion.clone().into(),
                        ..Default::default()
                    });
                    for index in 1..=MAX_UI_DIGITS {
                        parent
                            .spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(8. * WINDOW_SCALE),
                                        Val::Px(8. * WINDOW_SCALE),
                                    ),
                                    ..Default::default()
                                },
                                image: assets.text0.clone().into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(MAX_UI_DIGITS + 1 - index))
                            .insert(BorkPointNumber::default());
                    }

                    // Coin Counter
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(8. * WINDOW_SCALE), Val::Px(8. * WINDOW_SCALE)),
                            ..Default::default()
                        },
                        image: assets.coin.clone().into(),
                        ..Default::default()
                    });
                    for index in 1..=MAX_UI_DIGITS {
                        parent
                            .spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(8. * WINDOW_SCALE),
                                        Val::Px(8. * WINDOW_SCALE),
                                    ),
                                    ..Default::default()
                                },
                                image: assets.text0.clone().into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(MAX_UI_DIGITS + 1 - index))
                            .insert(CoinNumber::default());
                    }
                });
        });
}

fn update_counter(
    mut query: Query<&mut Text, With<Counter>>,
    time_since_level_start: Res<TimeSinceLevelStart>,
    asset_holder: Res<FontAssets>,
) {
    for mut text in query.iter_mut() {
        *text = Text::from_section(
            format!("{:.2}", 60. - time_since_level_start.0),
            TextStyle {
                font: asset_holder.fira_sans.clone(),
                font_size: 64.,
                color: Color::BLACK,
            },
        );
    }
}

fn update_health_containers() {}

fn update_potion_counter() {}

fn number_to_image(image_assets: TextureAssets, num: Option<&u32>) -> Handle<Image> {
    let default: u32 = 0;
    let num = num.unwrap_or(&default);
    match num {
        0 => image_assets.text0.clone(),
        1 => image_assets.text1.clone(),
        2 => image_assets.text2.clone(),
        3 => image_assets.text3.clone(),
        4 => image_assets.text4.clone(),
        5 => image_assets.text5.clone(),
        6 => image_assets.text6.clone(),
        7 => image_assets.text7.clone(),
        8 => image_assets.text8.clone(),
        9 => image_assets.text9.clone(),
        _ => image_assets.text0.clone(),
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
