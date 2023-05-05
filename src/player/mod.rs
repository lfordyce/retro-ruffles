mod entities;
use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
pub use entities::{EyePlugin, GoalPlugin};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::animation::{FromComponentPlugin, SpriteSheetAnimation};
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Component, Default)]
pub enum PlayerAnimationState {
    #[default]
    Idle,
    Moving(MovementDirection),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub enum Vitality {
    #[default]
    Alive,
    Dead,
}

impl From<PlayerAnimationState> for SpriteSheetAnimation {
    fn from(animation_state: PlayerAnimationState) -> Self {
        let indices = match animation_state {
            PlayerAnimationState::Idle => 15..23,
            PlayerAnimationState::Moving(direction) => match direction {
                MovementDirection::Up => 60..66,
                MovementDirection::Down => 30..36,
                MovementDirection::Left => 45..51,
                MovementDirection::Right => 45..51,
            },
        };

        let frame_timer = Timer::from_seconds(0.1, TimerMode::Repeating);

        let repeat = true;

        SpriteSheetAnimation {
            indices,
            frame_timer,
            repeat,
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
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

    player: Player,
    animation: PlayerAnimationState,
    vitality: Vitality,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.1,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugin(FromComponentPlugin::<
                PlayerAnimationState,
                SpriteSheetAnimation,
            >::new())
            .add_system(apply_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

fn apply_actions(
    time: Res<Time>,
    actions: Res<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut PlayerAnimationState,
            &mut Velocity,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    mut x_velocity_contribution: Local<f32>,
    mut y_velocity_contribution: Local<f32>,
) {
    // if actions.player_movement.is_none() {
    //     return;
    // }
    //
    let mut direction = Vec2::default();
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.y = 1.;
    } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.y = -1.;
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x = 1.;
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x = -1.;
    }
    let speed = 100.;

    for (mut animation_state, mut velocity, mut sprite) in &mut player_query {
        let move_delta = direction.normalize_or_zero() * speed;
        velocity.linvel = move_delta;

        // let mut move_delta = actions.player_movement.unwrap();
        //
        // if move_delta != Vec2::ZERO {
        //     move_delta /= move_delta.length();
        // }
        //
        // velocity.linvel = move_delta * speed * time.delta_seconds();
        if velocity.linvel.x.abs() > 0. {
            sprite.flip_x = velocity.linvel.x < 0.;
            if *animation_state != PlayerAnimationState::Moving(MovementDirection::Left) {
                *animation_state = PlayerAnimationState::Moving(MovementDirection::Left)
            }
        }
        if velocity.linvel.y > 0. {
            if *animation_state != PlayerAnimationState::Moving(MovementDirection::Up) {
                *animation_state = PlayerAnimationState::Moving(MovementDirection::Up)
            }
        }
        if velocity.linvel.y < 0. {
            if *animation_state != PlayerAnimationState::Moving(MovementDirection::Down) {
                *animation_state = PlayerAnimationState::Moving(MovementDirection::Down)
            }
        }
    }
}
//
// fn animate_player(
//     time: Res<Time>,
//     mut query: Query<
//         (
//             &mut PlayerAnimationState,
//             &mut TextureAtlasSprite,
//             &Velocity,
//         ),
//         With<Player>,
//     >,
// ) {
//     for (mut animation_state, mut sprite, velocity) in &mut query {
//         if velocity.0.length() < f32::EPSILON || velocity.0.y.abs() > 0. {
//             sprite.index = 0;
//             timer.0.reset();
//             continue;
//         }
//         timer.0.tick(time.delta());
//         if timer.0.finished() {
//             sprite.index = (sprite.index + 1) % timer.1;
//         }
//     }
// }

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerAnimationState,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for (mut player_transform, mut animation_state, mut sprite) in &mut player_query {
        player_transform.translation += movement;
    }
}
