mod components;
mod systems;

use crate::levels::components::WallBundle;
use crate::levels::systems::{camera_fit_inside_current_level, spawn_wall_collision};
use crate::loading::LevelAssets;
use crate::trigger::bundles::LevelTriggerBundle;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LdtkEntityAppExt, LdtkIntCellAppExt},
    *,
};
use bevy_rapier2d::plugin::PhysicsSet;

pub struct LevelsPlugin {
    pub level_index: usize,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            // Required to prevent race conditions between bevy_ecs_ldtk's and bevy_rapier's systems
            .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(LevelSelection::Index(self.level_index))
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_system(spawn_level.in_schedule(OnEnter(GameState::Playing)))
            .add_system(camera_fit_inside_current_level)
            .add_system(spawn_wall_collision)
            .add_system(despawn_world.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct World;

fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    level_selection: Res<LevelSelection>,
) {
    info!("Spawning level: {:?}", level_selection);
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: level_assets.level.clone_weak(),
            ..default()
        })
        .insert(World);

    // commands
    //     .spawn(LevelTriggerBundle::new(GameState::GameOver))
    //     .insert(Name::from("LevelTrigger"));
}

fn despawn_world(mut commands: Commands, world: Query<Entity, With<World>>) {
    let Ok(world) = world.get_single() else { return };
    commands.entity(world).despawn_recursive();
}
