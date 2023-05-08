use crate::animation::{FromComponentPlugin, SpriteSheetAnimation};
use crate::player::{ColliderBundle, Vitality};
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerAltPlugin;

#[derive(Component, Default, Clone)]
pub struct PlayerAlt;

impl From<PlayerAlt> for SpriteSheetAnimation {
    fn from(_: PlayerAlt) -> Self {
        SpriteSheetAnimation {
            indices: 0..4,
            frame_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            repeat: true,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
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
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerAltPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerAltBundle>("Crab")
            .add_plugin(FromComponentPlugin::<PlayerAlt, SpriteSheetAnimation>::new())
            .add_system(apply_alt_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

fn apply_alt_actions(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut TextureAtlasSprite), With<PlayerAlt>>,
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

    for (mut velocity, mut sprite) in &mut player_query {
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
        }
    }
}
