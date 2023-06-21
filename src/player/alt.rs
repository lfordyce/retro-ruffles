use crate::animation::{FromComponentPlugin, SpriteSheetAnimation};
use crate::player::{ColliderBundle, PlayerAction, Vitality};
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerAltPlugin;

#[derive(Component, Default, Clone)]
pub struct PlayerAlt;

// impl From<PlayerAlt> for SpriteSheetAnimation {
//     fn from(_: PlayerAlt) -> Self {
//         SpriteSheetAnimation {
//             indices: 0..4,
//             frame_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
//             repeat: true,
//         }
//     }
// }

impl From<PlayerAlt> for SpriteSheetAnimation {
    fn from(_: PlayerAlt) -> Self {
        SpriteSheetAnimation {
            indices: 65..71,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            repeat: true,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerInput {
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        use PlayerAction::*;
        let mut input_map = InputMap::default();
        // QwertyScanCode::W
        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        Self {
            input_manager: InputManagerBundle::<PlayerAction> {
                input_map,
                ..Default::default()
            },
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerAltBundle {
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

    player: PlayerAlt,
    vitality: Vitality,

    #[bundle]
    pub input: PlayerInput,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerAltPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .register_ldtk_entity::<PlayerAltBundle>("Runner")
            .add_plugin(FromComponentPlugin::<PlayerAlt, SpriteSheetAnimation>::new())
            .add_system(apply_alt_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

fn apply_alt_actions(
    mut player_query: Query<
        (
            &ActionState<PlayerAction>,
            &mut Velocity,
            &mut TextureAtlasSprite,
        ),
        With<PlayerAlt>,
    >,
) {
    let speed = 100.;

    for (action_state, mut velocity, mut sprite) in &mut player_query {
        let mut direction = Vec2::default();
        if action_state.pressed(PlayerAction::Up) {
            direction.y = 1.;
        } else if action_state.pressed(PlayerAction::Down) {
            direction.y = -1.;
        }

        if action_state.pressed(PlayerAction::Right) {
            direction.x = 1.;
        } else if action_state.pressed(PlayerAction::Left) {
            direction.x = -1.;
        }

        let move_delta = direction.normalize_or_zero() * speed;
        velocity.linvel = move_delta;

        if velocity.linvel.x.abs() > 0. {
            sprite.flip_x = velocity.linvel.x < 0.;
        }
    }
}
