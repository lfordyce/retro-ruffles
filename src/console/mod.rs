mod event;
mod systems;
mod utils;

use crate::{GameState, LevelState};

use bevy::prelude::{
    in_state, Component, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
    Reflect, Resource,
};

#[derive(Resource)]
pub struct ConsoleData {
    input: String,
    history_index: usize,
    history: Vec<String>,
    lines: Vec<String>,
}

impl Default for ConsoleData {
    fn default() -> Self {
        ConsoleData {
            input: String::from(""),
            history_index: 0,
            history: Vec::new(),
            lines: utils::welcome_lines(),
        }
    }
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ConsoleStateEntity;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ConsoleData>()
            .add_event::<event::PrintToConsoleEvent>()
            .add_system(systems::console_setup.in_schedule(OnEnter(LevelState::Console)))
            .add_systems(
                (
                    systems::update_lines_area,
                    event::add_message_events_to_console,
                    systems::close_console_handler,
                )
                    .in_set(OnUpdate(LevelState::Console))
                    .distributive_run_if(in_state(GameState::Playing)),
            )
            .add_system(
                systems::destroy_console_state_entities.in_schedule(OnExit(LevelState::Console)),
            );
    }
}
