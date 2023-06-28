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
        app.init_resource::<ButtonColors>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
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

#[derive(Resource)]
pub struct ButtonColors {
    pub(crate) normal: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb_u8(148, 32, 106),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    // flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        ..Default::default()
                    },
                    image: textures.press_start.clone().into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(600.0), Val::Px(75.0)),
                                // margin: UiRect::all(Val::Auto),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    bottom: Val::Px(100.0),
                                    top: Val::Auto,
                                },
                                // position: UiRect {
                                //     left: Val::Auto,
                                //     right: Val::Auto,
                                //     top: Val::Auto,
                                //     bottom: Val::Px(10.0),
                                // },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "PRESS",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 72.0,
                                    color: Color::WHITE,
                                    // color: Color::rgb_u8(148, 32, 106),
                                },
                            ));
                            parent.spawn(TextBundle::from_section(
                                "V",
                                TextStyle {
                                    font: font_assets.gamepad_font.clone(),
                                    font_size: 72.0,
                                    color: Color::WHITE,
                                    // color: Color::rgb_u8(148, 32, 106),
                                },
                            ));
                            parent.spawn(TextBundle::from_section(
                                "TO PLAY",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 72.0,
                                    color: Color::WHITE,
                                    // color: Color::rgb_u8(148, 32, 106),
                                },
                            ));
                        });
                });
        });
}

fn setup_controls_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
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
                                    font: font_assets.crt_font.clone(),
                                    font_size: 100.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // header wrapper
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Travel the paths to the 5 Ammo Cans",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 48.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // D-PAD
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_self: AlignSelf::Center,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                },
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Y",
                                TextStyle {
                                    font: font_assets.gamepad_font.clone(),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn(TextBundle::from_section(
                                "MOVE PLAYER",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // <A> TO SELECT
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_self: AlignSelf::Center,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                },
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                ",",
                                TextStyle {
                                    font: font_assets.gamepad_font.clone(),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn(TextBundle::from_section(
                                "SELECT ANSWER",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // START BUTTON
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_self: AlignSelf::Center,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                },
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "PRESS",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn(TextBundle::from_section(
                                "V",
                                TextStyle {
                                    font: font_assets.gamepad_font.clone(),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn(TextBundle::from_section(
                                "TO PLAY GAME",
                                TextStyle {
                                    font: font_assets.crt_font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                            ));
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
