use bevy::prelude::*;
use bevy::reflect::Reflect;
use leafwing_input_manager::prelude::ActionState;

use crate::actions::UiAction;
use crate::despawn::despawn_entity;
use crate::loading::{FontAssets, Question};
use crate::ui::Score;
use crate::{GameState, LevelState};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_lose.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(on_win.in_schedule(OnEnter(GameState::WinScreen)))
            .add_system(reset_state.in_set(OnUpdate(GameState::WinScreen)))
            .add_system(reset_state.in_set(OnUpdate(GameState::GameOver)))
            .add_system(despawn_entity::<EndScreen>.in_schedule(OnExit(GameState::WinScreen)))
            .add_system(despawn_entity::<EndScreen>.in_schedule(OnExit(GameState::GameOver)));
    }
}

// ------ SYSTEMS ------

fn reset_state(
    query: Query<&ActionState<UiAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_phase: ResMut<NextState<LevelState>>,
    mut score: ResMut<Score>,
    mut questions: ResMut<Assets<Question>>,
) {
    for action_state in &query {
        if action_state.just_pressed(UiAction::Start) {
            *score = Score::default();
            questions.iter_mut().for_each(|i| i.1.used = false);
            game_phase.set(LevelState::None);
            game_state.set(GameState::Controls);
        }
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct EndScreen;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ResetButton;

pub fn on_lose(
    mut commands: Commands,
    existing_end_screens: Query<Entity, With<EndScreen>>,
    asset_holder: Res<FontAssets>,
) {
    for entity in existing_end_screens.iter() {
        commands.entity(entity).despawn_recursive();
    }
    build_end_screen(&mut commands, &asset_holder, false);
}

fn on_win(
    mut commands: Commands,
    existing_end_screens: Query<Entity, With<EndScreen>>,
    asset_holder: Res<FontAssets>,
) {
    for entity in existing_end_screens.iter() {
        commands.entity(entity).despawn_recursive();
    }
    build_end_screen(&mut commands, &asset_holder, true);
}

fn build_end_screen(commands: &mut Commands, asset_holder: &Res<FontAssets>, win: bool) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    gap: Size::height(Val::Px(20.)),
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 1.)),
                ..default()
            },
            EndScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    if win {
                        "YOU WIN!\nCOLLECT A PATH TAG!"
                    } else {
                        "GAME OVER!"
                    },
                    TextStyle {
                        font: asset_holder.crt_font.clone(),
                        font_size: 96.,
                        color: if win { Color::PINK } else { Color::RED },
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
            if win {
                parent.spawn(
                    TextBundle::from_section(
                        "(Limit of 1 path tag per family please)",
                        TextStyle {
                            font: asset_holder.crt_font.clone(),
                            font_size: 48.,
                            color: Color::PINK,
                        },
                    )
                    .with_text_alignment(TextAlignment::Center),
                );
            }

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "PRESS",
                        TextStyle {
                            font: asset_holder.crt_font.clone(),
                            font_size: 48.,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        "V",
                        TextStyle {
                            font: asset_holder.gamepad_font.clone(),
                            font_size: 48.,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        "TO RESTART",
                        TextStyle {
                            font: asset_holder.crt_font.clone(),
                            font_size: 48.,
                            color: Color::WHITE,
                        },
                    ));
                });

            // parent
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 size: Size::new(Val::Px(250.0), Val::Px(65.0)),
            //                 margin: UiRect::all(Val::Px(20.0)),
            //                 justify_content: JustifyContent::Center,
            //                 align_items: AlignItems::Center,
            //                 ..default()
            //             },
            //             background_color: Color::hex("#ffecd6").unwrap().into(),
            //             ..default()
            //         },
            //         ResetButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle::from_section(
            //             "Play Again",
            //             TextStyle {
            //                 font: asset_holder.crt_font.clone(),
            //                 font_size: 48.,
            //                 color: Color::RED,
            //             },
            //         ));
            //     });
        });
}
