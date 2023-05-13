mod actions;
mod animation;
mod audio;
mod clock;
mod console;
mod game_over;
mod levels;
mod loading;
mod menu;
mod player;
mod trigger;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use crate::animation::SpriteSheetAnimationPlugin;
use crate::clock::ClockPlugin;
use crate::console::ConsolePlugin;
use crate::game_over::GameOverPlugin;
use crate::levels::LevelsPlugin;
use crate::player::{alt::PlayerAltPlugin, AltGoalPlugin};
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Copy, Hash, Default, States, Reflect)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    GameOver,
    WinScreen,
}

#[derive(Clone, Eq, PartialEq, Debug, Copy, Hash, Default, States, Reflect)]
pub enum LevelState {
    #[default]
    None,
    OverWorld,
    Console,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<LevelState>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..Default::default()
            })
            .add_plugin(RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            })
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(LevelsPlugin { level_index: 0 })
            .add_plugin(ActionsPlugin)
            .add_plugin(SpriteSheetAnimationPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(ClockPlugin)
            .add_plugin(UiPlugin)
            // .add_plugin(PlayerPlugin)
            .add_plugin(PlayerAltPlugin)
            // .add_plugin(GoalPlugin)
            .add_plugin(AltGoalPlugin)
            .add_plugin(ConsolePlugin)
            .add_plugin(GameOverPlugin);
        // .add_plugin(EyePlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
