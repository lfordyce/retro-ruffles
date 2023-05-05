use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Sensor;

use crate::animation::{FromComponentPlugin, SpriteSheetAnimation};

#[derive(Component, Default, Clone)]
pub struct Goal;

impl From<Goal> for SpriteSheetAnimation {
    fn from(_: Goal) -> Self {
        SpriteSheetAnimation {
            indices: 66..68,
            frame_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            repeat: true,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[grid_coords]
    grid_coords: GridCoords,

    goal: Goal,
    sensor: Sensor,
}

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<GoalBundle>("Goal")
            .add_plugin(FromComponentPlugin::<Goal, SpriteSheetAnimation>::new());
    }
}

#[derive(Component, Default, Clone)]
pub struct Eye;

impl From<Eye> for SpriteSheetAnimation {
    fn from(_: Eye) -> Self {
        SpriteSheetAnimation {
            indices: 7..9,
            frame_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            repeat: true,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct EyeBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[grid_coords]
    grid_coords: GridCoords,

    goal: Eye,
}

pub struct EyePlugin;

impl Plugin for EyePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<EyeBundle>("Eye")
            .add_plugin(FromComponentPlugin::<Eye, SpriteSheetAnimation>::new());
    }
}
