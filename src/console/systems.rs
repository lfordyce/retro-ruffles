use crate::console::ConsoleData;
use crate::loading::{FontAssets, TextureAssets};
use crate::LevelState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ConsoleForeground;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct LinesArea;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct CommandInput;

pub fn console_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
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
            // size: Size::new(Val::Px(primary.width()), Val::Px(primary.height())),
            size: Size::new(Val::Percent(100.), Val::Percent(120.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
        ..Default::default()
    };

    // crt overlay
    let foreground_component = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(primary.width()), Val::Px(primary.height())),
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
    entities_query: Query<Entity, With<super::ConsoleStateEntity>>,
) {
    info!("[ConsolePlugin] Destroying state entities before exiting...");
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[ConsolePlugin] Exiting console state")
}
