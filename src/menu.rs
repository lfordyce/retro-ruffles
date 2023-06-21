use crate::actions::UiAction;
use crate::loading::FontAssets;
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
            .add_event::<LevelStart>();
    }
}

pub struct LevelStart;

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(160.0), Val::Px(70.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        // .insert(ActionStateDriver {
        //     action: UiAction::Start,
        //     entity: (),
        // })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PLAY",
                TextStyle {
                    font: font_assets.pico.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn click_play_button(
    mut state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut query: Query<&ActionState<UiAction>>,
    mut level_start_events: EventWriter<LevelStart>,
) {
    for action_state in &mut query {
        if action_state.pressed(UiAction::Start) {
            state.set(GameState::Playing);
            level_state.set(LevelState::OverWorld);
            level_start_events.send(LevelStart);
        }
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
