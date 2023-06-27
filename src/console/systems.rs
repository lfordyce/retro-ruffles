use crate::actions::UiAction;
use crate::loading::{FontAssets, Question, TextureAssets};
use crate::ui::Score;
use crate::{GameState, LevelState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::*;

#[derive(Component)]
pub struct UiRootNode;

#[derive(Component)]
pub struct WrongRootNode;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectedQuestion {
    question: Handle<Question>,
}

#[derive(Reflect, Component, Default, PartialEq, Eq, Clone)]
#[reflect(Component)]
pub struct BtnGridPos {
    pub row: usize,
    pub choice: String,
}

impl BtnGridPos {
    pub fn new(row: usize, choice: String) -> Self {
        Self { row, choice }
    }
}

#[derive(Default, Resource)]
pub struct AbilityMenuState {
    pub selected_pos: BtnGridPos,
}

pub fn setup_splash(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
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
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..Default::default()
            },
            WrongRootNode,
        ))
        .with_children(|parent| {
            // Main box
            parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(1100.0), Val::Px(600.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    image: texture_assets.menu_background.clone().into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title text wrapper
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(110.0)),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Title text
                            parent.spawn(
                                TextBundle::from_section(
                                    "WRONG",
                                    TextStyle {
                                        font: font_assets.pixel_font.clone(),
                                        font_size: 96.0,
                                        color: Color::RED,
                                    },
                                )
                                .with_text_alignment(TextAlignment::Center)
                                .with_style(Style {
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Px(240.0),
                                        ..default()
                                    },
                                    max_size: Size {
                                        width: Val::Px(1000.),
                                        height: Val::Undefined,
                                    },
                                    ..default()
                                }),
                            );
                        });
                });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

pub fn countdown(
    mut level_state: ResMut<NextState<LevelState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        level_state.set(LevelState::Console);
    }
}

pub fn setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    mut input: ResMut<Input<KeyCode>>,
    mut questions: ResMut<Assets<Question>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    input.clear(); // clear any `just_pressed` events that may be left over from previous state
    commands.insert_resource(AbilityMenuState::default());

    let button_style = Style {
        size: Size::new(Val::Px(800.0), Val::Px(75.0)),
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

    match questions
        .iter_mut()
        .filter(|(_idx, q)| !q.used)
        .choose(&mut thread_rng())
    {
        Some((id, picked)) => {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Main box
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(1100.0), Val::Px(600.0)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            image: texture_assets.menu_background.clone().into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Title text wrapper
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Px(110.0)),
                                        justify_content: JustifyContent::SpaceAround,
                                        align_items: AlignItems::Center,
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Title text
                                    parent.spawn(
                                        TextBundle::from_section(
                                            picked.clone().description,
                                            TextStyle {
                                                font: font_assets.pixel_font.clone(),
                                                font_size: 18.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_text_alignment(TextAlignment::Center)
                                        .with_style(
                                            Style {
                                                position_type: PositionType::Absolute,
                                                position: UiRect {
                                                    top: Val::Px(30.0),
                                                    left: Val::Px(30.0),
                                                    right: Val::Px(15.0),
                                                    ..default()
                                                },
                                                max_size: Size {
                                                    width: Val::Px(1000.),
                                                    height: Val::Undefined,
                                                },
                                                ..default()
                                            },
                                        ),
                                    );
                                });

                            // Buttons wrapper
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Auto),
                                        ..Default::default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Equipment buttons wrapper
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
                                                "CHOICES:",
                                                TextStyle {
                                                    font: font_assets.pixel_font.clone(),
                                                    font_size: 20.0,
                                                    color: Color::WHITE,
                                                },
                                            ));

                                            // Answer choices
                                            let mut options = picked.clone().options;
                                            options.shuffle(&mut thread_rng());

                                            for (pos, choice) in options.iter().enumerate() {
                                                parent
                                                    .spawn((
                                                        ButtonBundle {
                                                            style: button_style.clone(),
                                                            background_color: Color::rgb(
                                                                0.15, 0.15, 0.15,
                                                            )
                                                            .into(),
                                                            image: texture_assets
                                                                .button
                                                                .clone()
                                                                .into(),
                                                            ..Default::default()
                                                        },
                                                        BtnGridPos::new(pos, choice.clone()),
                                                        SelectedQuestion {
                                                            question: Handle::weak(id),
                                                        },
                                                        Name::new("Choice Slot"),
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            choice,
                                                            TextStyle {
                                                                font: font_assets
                                                                    .pixel_font
                                                                    .clone(),
                                                                font_size: 20.0,
                                                                color: Color::WHITE,
                                                            },
                                                        ));
                                                    });
                                            }
                                        });
                                });

                            // Buttons help text wrapper
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
                                            bottom: Val::Px(30.0),
                                        },
                                        ..Default::default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Use arrow keys & <A> to select",
                                        TextStyle {
                                            font: font_assets.pixel_font.clone(),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        });
                })
                .insert(UiRootNode);
        }
        None => {
            level_state.set(LevelState::OverWorld);
            game_state.set(GameState::GameOver);
        }
    }
}

pub fn button_mouse_select(
    mut element_button_query: Query<(&BtnGridPos, &mut BackgroundColor), With<Button>>,
    state: ResMut<AbilityMenuState>,
) {
    for (grid_pos, mut color) in &mut element_button_query {
        if state.selected_pos.row == grid_pos.row {
            // info!("button selected {}", grid_pos.row);
            // *color = BackgroundColor(Color::rgb(0.25, 0.55, 0.25).into());
            *color = BackgroundColor(Color::BLUE)
        } else {
            // *color = BackgroundColor(Color::rgb(0.15, 0.45, 0.15).into());
            *color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
        }
    }
}

pub fn button_keyboard_select(
    element_button_query: Query<(&BtnGridPos, &SelectedQuestion)>,
    mut state: ResMut<AbilityMenuState>,
    mut query: Query<&ActionState<UiAction>>,
    mut questions: ResMut<Assets<Question>>,
    mut score: ResMut<Score>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for action_state in &mut query {
        if action_state.just_pressed(UiAction::Down) {
            state.selected_pos.row += 1;
            if state.selected_pos.row >= 3 {
                state.selected_pos.row = 0;
            }
        }
        if action_state.just_pressed(UiAction::Up) {
            if state.selected_pos.row == 0 {
                state.selected_pos.row = 0;
            } else {
                state.selected_pos.row -= 1;
            }
        }
        if action_state.just_pressed(UiAction::Select) {
            for (grid_pos, selected_question) in element_button_query.iter() {
                if grid_pos.row == state.selected_pos.row {
                    // info!("key code select: {}", grid_pos.choice);
                    if let Some(handle) = questions.get_mut(&selected_question.question) {
                        handle.used = true;
                        if handle.answer == grid_pos.choice {
                            info!("CORRECT ANSWER: {}", grid_pos.choice);
                            score.0 += 1.;
                            if score.0 >= 5. {
                                game_state.set(GameState::WinScreen);
                                level_state.set(LevelState::OverWorld);
                            } else {
                                level_state.set(LevelState::OverWorld);
                            }
                        } else {
                            info!("WRONG!: {}", grid_pos.choice);
                            level_state.set(LevelState::Wrong);
                        }
                    }
                    return;
                }
            }
        }
    }
}

pub fn destroy_console_state_entities(
    mut commands: Commands,
    entities_query: Query<Entity, With<UiRootNode>>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    info!("[ConsolePlugin] Destroying state entities before exiting...");
    commands.remove_resource::<AbilityMenuState>();
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    keyboard.clear();
    info!("[ConsolePlugin] Exiting console state")
}

pub fn destroy_wrong_state_entities(
    mut commands: Commands,
    entities_query: Query<Entity, With<WrongRootNode>>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    info!("[ConsolePlugin] Destroying wrong state entities before exiting...");
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    keyboard.clear();
    info!("[ConsolePlugin] Exiting wrong console state")
}
