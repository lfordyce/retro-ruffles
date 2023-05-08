use crate::GameState;
use bevy::prelude::Component;

#[derive(Component, Default, Debug, Clone)]
pub struct Trigger;

#[derive(Component, Default, Debug, Clone)]
pub struct TriggerTargetLevel(pub GameState);
