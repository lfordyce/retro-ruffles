mod systems;

use crate::game_over::systems::{cleanup, game_over_screen, game_over_start_new_game, reset_score};
use crate::GameState;
use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfig, OnEnter, OnExit, OnUpdate,
    Plugin,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((cleanup, reset_score).in_schedule(OnExit(GameState::GameOver)))
            .add_system(game_over_screen.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(game_over_start_new_game.in_set(OnUpdate(GameState::GameOver)));
    }
}
