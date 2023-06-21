mod systems;

use crate::{GameState, LevelState};

use crate::console::systems::SelectedQuestion;
use bevy::prelude::{
    in_state, Component, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
    Reflect,
};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ConsoleStateEntity;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SelectedQuestion>()
            .add_system(systems::setup.in_schedule(OnEnter(LevelState::Console)))
            .add_systems(
                (
                    systems::button_mouse_select,
                    systems::button_keyboard_select,
                )
                    .in_set(OnUpdate(LevelState::Console))
                    .distributive_run_if(in_state(GameState::Playing)),
            )
            .add_system(
                systems::destroy_console_state_entities.in_schedule(OnExit(LevelState::Console)),
            );
    }
}
