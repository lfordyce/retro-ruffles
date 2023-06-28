use crate::actions::UiAction;
use crate::loading::{FontAssets, TextureAssets};
use crate::{GameState, LevelState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::Menu)))
            .add_system(setup_controls_menu.in_schedule(OnEnter(GameState::Controls)))
            .add_system(click_control_play_button.in_set(OnUpdate(GameState::Controls)))
            .add_system(cleanup_control_menu.in_schedule(OnExit(GameState::Controls)))
            .add_event::<LevelStart>();
    }
}

pub struct LevelStart;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct ControlMenu;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                image: textures.press_start.clone().into(),
                ..default()
            });
        });
}

fn setup_controls_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
    let txt_style = Style {
        size: Size::new(Val::Px(195.0), Val::Px(65.0)),
        // center button
        margin: UiRect {
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..Default::default()
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::rgb_u8(22, 23, 26).into(),
                ..Default::default()
            },
            ControlMenu,
        ))
        .with_children(|parent| {
            // Main box
            parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(1000.0), Val::Px(500.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    image: textures.menu_background.clone().into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title text wrapper
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(110.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Title text
                            parent.spawn(TextBundle::from_section(
                                "CONTROLS",
                                TextStyle {
                                    font: font_assets.pixel_font.clone(),
                                    font_size: 48.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Control wrapper
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(65.0), Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Auto),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Button type wrapper
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Px(0.0),
                                            bottom: Val::Auto,
                                        },
                                        flex_direction: FlexDirection::Column,
                                        ..Default::default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Buttons:",
                                        TextStyle {
                                            font: font_assets.pixel_font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                    // D-PAD button
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Y",
                                            TextStyle {
                                                font: font_assets.gamepad_font.clone(),
                                                font_size: 64.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(txt_style.clone()),
                                    );
                                    // A button
                                    parent.spawn(
                                        TextBundle::from_section(
                                            ",",
                                            TextStyle {
                                                font: font_assets.gamepad_font.clone(),
                                                font_size: 64.0,
                                                color: Color::rgb(0.25, 0.55, 0.25).into(),
                                            },
                                        )
                                        .with_style(txt_style.clone()),
                                    );
                                    // Start button
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "V",
                                            TextStyle {
                                                font: font_assets.gamepad_font.clone(),
                                                font_size: 64.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(txt_style.clone()),
                                    );
                                });

                            //
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Px(0.0),
                                            bottom: Val::Auto,
                                        },
                                        flex_direction: FlexDirection::Column,
                                        ..Default::default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Actions:",
                                        TextStyle {
                                            font: font_assets.pixel_font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                    // D-PAD button
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "MOVE PLAYER",
                                            TextStyle {
                                                font: font_assets.pixel_font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(txt_style.clone()),
                                    );
                                    // A button
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "SELECT YOUR ANSWER",
                                            TextStyle {
                                                font: font_assets.pixel_font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(txt_style.clone()),
                                    );
                                    // Start button
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "TO CONTINUE...",
                                            TextStyle {
                                                font: font_assets.pixel_font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(txt_style.clone()),
                                    );
                                });
                        });
                });
        });
}

fn click_play_button(
    mut state: ResMut<NextState<GameState>>,
    input: Query<&ActionState<UiAction>>,
) {
    let action_state = input.single();
    if action_state.just_pressed(UiAction::Start) {
        state.set(GameState::Controls);
    }

    // for action_state in &mut query {
    //     if action_state.pressed(UiAction::Start) {
    //         state.set(GameState::Controls);
    //     }
    // }
}

fn click_control_play_button(
    mut state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    input: Query<&ActionState<UiAction>>,
    mut level_start_events: EventWriter<LevelStart>,
) {
    let action_state = input.single();
    if action_state.just_pressed(UiAction::Start) {
        state.set(GameState::Playing);
        level_state.set(LevelState::OverWorld);
        level_start_events.send(LevelStart);
    }

    // for action_state in &mut query {
    //     if action_state.pressed(UiAction::Start) {
    //         state.set(GameState::Playing);
    //         level_state.set(LevelState::OverWorld);
    //         level_start_events.send(LevelStart);
    //     }
    // }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_control_menu(mut commands: Commands, query: Query<Entity, With<ControlMenu>>) {
    info!("cleaning up controls menu");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
