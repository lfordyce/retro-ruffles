mod systems;

use crate::game_over::systems::{cleanup, game_over_screen, game_over_start_new_game};
use crate::GameState;
use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            game_over_screen.in_schedule(OnEnter(GameState::GameOver)),
            game_over_start_new_game.in_set(OnUpdate(GameState::GameOver)),
            cleanup.in_schedule(OnExit(GameState::GameOver)),
        ));
    }
}
