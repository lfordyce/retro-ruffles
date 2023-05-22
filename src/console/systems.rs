use crate::console::ConsoleData;
use crate::loading::{FontAssets, Question, TextureAssets};
use crate::ui::Score;
use crate::LevelState;
use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use std::fmt::Display;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ConsoleForeground;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct LinesArea;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct CommandInput;

pub fn console_setup(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    info!("[ConsolePlugin] Building console UI");
    // let Ok(primary) = primary_query.get_single() else {
    //     return;
    // };

    let primary = primary_query.get_single().unwrap();
    info!("W: {} H: {}", primary.width(), primary.height());

    // root component
    let parent_component = NodeBundle {
        style: Style {
            // size: Size::new(Val::Px(1200.), Val::Px(600.)),
            size: Size::new(Val::Percent(90.), Val::Percent(90.)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: UiRect {
                left: Val::Px(10.),
                bottom: Val::Px(10.),
                // right: Val::Px(10.),
                top: Val::Px(95.),
                ..Default::default()
            },
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
        ..Default::default()
    };

    // crt overlay
    let foreground_component = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(800.), Val::Px(600.)),
            // size: Size::new(Val::Px(primary.width()), Val::Px(primary.height())),
            ..Default::default()
        },
        image: texture_assets.crt_overlay.clone().into(),
        ..Default::default()
    };

    // console root
    let console_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(primary.width() / 2.0),
                Val::Px(primary.height() / 1.5),
            ),
            padding: UiRect {
                left: Val::Percent(2.0),
                right: Val::Percent(2.0),
                top: Val::Percent(2.0),
                bottom: Val::Percent(2.0),
            },
            flex_direction: FlexDirection::ColumnReverse,
            overflow: Overflow::Hidden,
            // justify_content: JustifyContent::Center,
            // align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb_u8(5, 17, 0).into()),
        ..Default::default()
    };

    // lines area
    let lines_container_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba_u8(0, 0, 0, 0).into()),
        ..Default::default()
    };

    let lines_component = TextBundle {
        ..Default::default()
    };

    // command container
    let command_container_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba_u8(0, 0, 0, 0).into()),
        ..Default::default()
    };
    let command_component = TextBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        ..Default::default()
    };

    // ---------- UI TREE CONSTRUCTION ----------//
    commands
        .spawn(parent_component)
        .with_children(|parent| {
            // console
            parent.spawn(console_component).with_children(|parent| {
                // console lines
                parent
                    .spawn(lines_container_component)
                    .with_children(|parent| {
                        // placeholder to be populated with lines
                        parent.spawn(lines_component).insert(LinesArea);
                    });
                // console command input
                parent
                    .spawn(command_container_component)
                    .with_children(|parent| {
                        // placeholder to be populated with the command input
                        parent.spawn(command_component).insert(CommandInput);
                    });
            });
            // foreground
            parent.spawn(foreground_component).insert(ConsoleForeground);
        })
        .insert(super::ConsoleStateEntity);

    info!("[ConsolePlugin] UI constructed");
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Answers {
    One,
    Two,
    Three,
}

impl Display for Answers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "One"),
            Self::Two => write!(f, "Two"),
            Self::Three => write!(f, "Three"),
        }
    }
}

impl Answers {
    fn variant_from_index(idx: usize) -> Self {
        match idx {
            0 => Answers::One,
            1 => Answers::Two,
            3 => Answers::Three,
            _ => Answers::One,
        }
    }
}

#[derive(Component)]
pub struct UiRootNode;
#[derive(Component)]
pub struct Slot1Text;
#[derive(Component)]
pub struct Slot2Text;
#[derive(Component)]
pub struct CombinationText;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PotionMixSlot {
    pub index: usize,
}

#[derive(Reflect, Component, Default, PartialEq, Eq, Clone, Copy)]
#[reflect(Component)]
pub struct BtnGridPos {
    pub row: u8,
    pub col: u8,
}

impl BtnGridPos {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}

#[derive(Default, Resource)]
pub struct AbilityMenuState {
    pub selected_pos: BtnGridPos,
}

pub fn setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    mut input: ResMut<Input<KeyCode>>,
    mut questions: ResMut<Assets<Question>>,
) {
    input.clear(); // clear any `just_pressed` events that may be left over from previous state
    commands.insert_resource(AbilityMenuState::default());

    let button_style = Style {
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

    // let mut questions: Vec<(HandleId, &Question)> = questions.clone().iter().collect();
    // let picked = questions.iter().choose(&mut thread_rng()).unwrap();

    let (_id, picked) = questions.iter_mut().choose(&mut thread_rng()).unwrap();
    // let choices = out.clone();
    // (0..questions.len())

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Main box
            parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(1000.0), Val::Px(600.0)),
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
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Title text
                            parent.spawn(TextBundle::from_section(
                                picked.clone().description,
                                TextStyle {
                                    font: font_assets.pixel_font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
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
                            background_color: BackgroundColor(Color::NONE.into()),
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
                                    background_color: BackgroundColor(Color::NONE.into()),
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
                                                    background_color: BackgroundColor(
                                                        Color::rgb(0.15, 0.15, 0.15).into(),
                                                    ),
                                                    image: texture_assets.button.clone().into(),
                                                    ..Default::default()
                                                },
                                                Answers::variant_from_index(pos),
                                                BtnGridPos::new(pos as u8, 0),
                                                Name::new("Choice Slot"),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    choice,
                                                    TextStyle {
                                                        font: font_assets.pixel_font.clone(),
                                                        font_size: 20.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }

                                    // parent
                                    //     .spawn(ButtonBundle {
                                    //         style: button_style.clone(),
                                    //         background_color: BackgroundColor(
                                    //             Color::rgb(0.15, 0.15, 0.15).into(),
                                    //         ),
                                    //         image: texture_assets.button.clone().into(),
                                    //         ..Default::default()
                                    //     })
                                    //     .insert(Answers::One)
                                    //     .insert(BtnGridPos::new(0, 0))
                                    //     .with_children(|parent| {
                                    //         parent.spawn(TextBundle::from_section(
                                    //             "ANSWER 1",
                                    //             TextStyle {
                                    //                 font: font_assets.pixel_font.clone(),
                                    //                 font_size: 20.0,
                                    //                 color: Color::WHITE,
                                    //             },
                                    //         ));
                                    //     });
                                    //
                                    // parent
                                    //     .spawn(ButtonBundle {
                                    //         style: button_style.clone(),
                                    //         background_color: BackgroundColor(
                                    //             Color::rgb(0.15, 0.15, 0.15).into(),
                                    //         ),
                                    //         image: texture_assets.button.clone().into(),
                                    //         ..Default::default()
                                    //     })
                                    //     .insert(Answers::Two)
                                    //     .insert(BtnGridPos::new(1, 0))
                                    //     .with_children(|parent| {
                                    //         parent.spawn(TextBundle::from_section(
                                    //             "ANSWER 2",
                                    //             TextStyle {
                                    //                 font: font_assets.pixel_font.clone(),
                                    //                 font_size: 17.0,
                                    //                 color: Color::WHITE,
                                    //             },
                                    //         ));
                                    //     });
                                    //
                                    // parent
                                    //     .spawn(ButtonBundle {
                                    //         style: button_style.clone(),
                                    //         background_color: BackgroundColor(
                                    //             Color::rgb(0.15, 0.15, 0.15).into(),
                                    //         ),
                                    //         image: texture_assets.button.clone().into(),
                                    //         ..Default::default()
                                    //     })
                                    //     .insert(Answers::Three)
                                    //     .insert(BtnGridPos::new(2, 0))
                                    //     .with_children(|parent| {
                                    //         parent.spawn(TextBundle::from_section(
                                    //             "ANSWER 3",
                                    //             TextStyle {
                                    //                 font: font_assets.pixel_font.clone(),
                                    //                 font_size: 17.0,
                                    //                 color: Color::WHITE,
                                    //             },
                                    //         ));
                                    //     });
                                });
                        });
                    // Header wrapper
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
                            background_color: BackgroundColor(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Answers:",
                                TextStyle {
                                    font: font_assets.pixel_font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Combination text wrapper
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(
                                    "Some sort of description",
                                    TextStyle {
                                        font: font_assets.pixel_font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(CombinationText);
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
                                    bottom: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Use arrow keys & <z> to select. Press <x> when done",
                                TextStyle {
                                    font: font_assets.pixel_font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        })
        .insert(UiRootNode);
}

pub fn button_interaction_system(
    element_button_query: Query<
        (&Interaction, &Answers, &BtnGridPos),
        (With<Button>, Changed<Interaction>),
    >,
    mut state: ResMut<AbilityMenuState>,
) {
    for (interaction, element, grid_pos) in &element_button_query {
        match *interaction {
            Interaction::Clicked => {
                info!("clicked: {}", *element)
            }
            Interaction::Hovered => {
                state.selected_pos = *grid_pos;
            }
            _ => {}
        }
    }
}

pub fn button_mouse_select(
    mut element_button_query: Query<(&Answers, &BtnGridPos, &mut BackgroundColor), With<Button>>,
    state: ResMut<AbilityMenuState>,
) {
    for (_element, grid_pos, mut color) in &mut element_button_query {
        if state.selected_pos == *grid_pos {
            // info!("button selected {}", grid_pos.row);
            // *color = BackgroundColor(Color::rgb(0.25, 0.55, 0.25).into());
            *color = BackgroundColor(Color::BLUE)
        } else {
            // *color = BackgroundColor(Color::rgb(0.15, 0.45, 0.15).into());
            *color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15).into());
        }
    }
}

pub fn button_keyboard_select(
    element_button_query: Query<(&Answers, &BtnGridPos)>,
    mut state: ResMut<AbilityMenuState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Down) {
        state.selected_pos.row += 1;
        if state.selected_pos.row >= 3 {
            state.selected_pos.row = 0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        if state.selected_pos.row == 0 {
            state.selected_pos.row = 0;
        } else {
            state.selected_pos.row -= 1;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Z) {
        for (element, grid_pos) in element_button_query.iter() {
            if *grid_pos == state.selected_pos {
                info!("key code select: {}", *element);
                return;
            }
        }
    }
}

pub fn update_lines_area(
    data: Res<ConsoleData>,
    font_assets: Res<FontAssets>,
    mut lines_area_query: Query<&mut Text, With<LinesArea>>,
) {
    let sections_text = data.lines.join("\n");
    let sections = vec![TextSection {
        value: sections_text,
        style: TextStyle {
            font: font_assets.crt_font.clone(),
            font_size: 16.,
            color: Color::rgba_u8(76, 207, 76, 255),
        },
    }];
    let mut text = lines_area_query.single_mut();
    text.sections = sections;
}

pub fn close_console_handler(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        level_state.set(LevelState::OverWorld);
        keyboard.reset(KeyCode::Escape);
    }
}

pub fn destroy_console_state_entities(
    mut commands: Commands,
    // entities_query: Query<Entity, With<super::ConsoleStateEntity>>,
    entities_query: Query<Entity, With<UiRootNode>>,
    mut score: ResMut<Score>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    info!("[ConsolePlugin] Destroying state entities before exiting...");
    commands.remove_resource::<AbilityMenuState>();
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    score.0 += 1.;
    keyboard.clear();
    info!("[ConsolePlugin] Exiting console state")
}
