use crate::GameState;
use bevy::prelude::Bundle;

use super::components::{Trigger, TriggerTargetLevel};

#[derive(Bundle)]
pub struct LevelTriggerBundle {
    tag: Trigger,
    target: TriggerTargetLevel,
}

impl LevelTriggerBundle {
    pub fn new(target: GameState) -> Self {
        LevelTriggerBundle {
            tag: Trigger,
            target: TriggerTargetLevel(target),
        }
    }
}
