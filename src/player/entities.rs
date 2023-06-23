use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, Sensor};

use crate::animation::{FromComponentPlugin, SpriteSheetAnimation};
use crate::loading::Question;
use crate::player::alt::PlayerAlt;
use crate::player::{ColliderBundle, Player, Vitality};
use crate::{GameState, LevelState};

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

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,

    goal: Goal,
    sensor: Sensor,
}

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<GoalBundle>("Goal")
            .add_plugin(FromComponentPlugin::<Goal, SpriteSheetAnimation>::new())
            .add_system(player_goal_collision.in_set(OnUpdate(GameState::Playing)));
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

fn player_goal_collision(
    mut commands: Commands,
    player_query: Query<&Vitality, With<Player>>,
    goal_query: Query<Entity, With<Goal>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    for collision in collision_events.iter() {
        if let CollisionEvent::Started(a, b, _) = collision {
            if player_query.contains(*a) && goal_query.contains(*b)
                || player_query.contains(*b) && goal_query.contains(*a)
            {
                info!("bumped into goal");
                if *player_query.single() == Vitality::Alive {
                    if goal_query.contains(*a) {
                        info!("goal reached... de-spawning goal entity");
                        commands.entity(*a).despawn_recursive();
                    } else if goal_query.contains(*b) {
                        info!("goal reached... de-spawning goal entity");
                        commands.entity(*b).despawn_recursive();
                    }
                    level_state.set(LevelState::Console);
                }
            }
        }
    }
}

// ALT GOAL USAGE FROM LDTK MAP
#[derive(Component, Default, Clone)]
pub struct AltGoal;

impl From<AltGoal> for SpriteSheetAnimation {
    fn from(_: AltGoal) -> Self {
        SpriteSheetAnimation {
            indices: 192..196,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            repeat: true,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct AltGoalBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[grid_coords]
    grid_coords: GridCoords,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,

    goal: AltGoal,
    sensor: Sensor,
}

pub struct AltGoalPlugin;

impl Plugin for AltGoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<AltGoalBundle>("Goal_Alt")
            // .add_plugin(FromComponentPlugin::<AltGoal, SpriteSheetAnimation>::new())
            .add_system(player_alt_goal_collision.in_set(OnUpdate(GameState::Playing)));
    }
}

fn player_alt_goal_collision(
    mut commands: Commands,
    player_query: Query<&Vitality, With<PlayerAlt>>,
    goal_query: Query<Entity, With<AltGoal>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut game_state: ResMut<NextState<GameState>>,
    questions: Res<Assets<Question>>,
) {
    for collision in collision_events.iter() {
        if let CollisionEvent::Started(a, b, _) = collision {
            if player_query.contains(*a) && goal_query.contains(*b)
                || player_query.contains(*b) && goal_query.contains(*a)
            {
                info!("bumped into goal");
                if *player_query.single() == Vitality::Alive {
                    if goal_query.contains(*a) {
                        info!("goal reached... de-spawning goal entity");
                        commands.entity(*a).despawn_recursive();
                    } else if goal_query.contains(*b) {
                        info!("goal reached... de-spawning goal entity");
                        commands.entity(*b).despawn_recursive();
                    }

                    if questions.iter().all(|(_idx, &ref q)| q.used) {
                        game_state.set(GameState::GameOver);
                    } else {
                        level_state.set(LevelState::Console);
                    }
                }
            }
        }
    }
}
