use crate::player::Vitality;
use crate::GameState;
use bevy::prelude::{Changed, NextState, Query, ResMut, With};

use super::components::{Trigger, TriggerTargetLevel};

pub fn activate_trigger(
    trigger_query: Query<(&Vitality, &TriggerTargetLevel), With<Trigger>>,
    vitality_query: Query<&Vitality, Changed<Vitality>>,
    mut level_state: ResMut<NextState<GameState>>,
) {
    if vitality_query.is_empty() {
        return;
    }

    let changed_vitality = vitality_query
        .get_single()
        .expect("0 or more than 1 `Vitality` found.");

    for (trigger_vitality, trigger_target_level) in trigger_query.iter() {
        if *changed_vitality == *trigger_vitality {
            level_state.set(trigger_target_level.0);
        }
    }
}
