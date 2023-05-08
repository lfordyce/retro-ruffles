pub mod bundles;
mod components;
mod systems;

use crate::trigger::systems::activate_trigger;
use crate::GameState;
use bevy::prelude::{App, IntoSystemConfig, OnUpdate, Plugin};

pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(activate_trigger.in_set(OnUpdate(GameState::Playing)));
    }
}
