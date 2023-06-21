use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;

#[derive(Actionlike, Debug, PartialEq, Clone, Copy)]
pub enum UiAction {
    Up,
    Down,
    Select,
    Start,
}

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<UiAction>::default())
            .add_startup_system(spawn_action_system);
    }
}

fn spawn_action_system(mut commands: Commands) {
    let mut input_map = InputMap::new([
        (KeyCode::Space, UiAction::Start),
        (KeyCode::Space, UiAction::Select),
        (KeyCode::Up, UiAction::Up),
        (KeyCode::Down, UiAction::Down),
    ]);
    input_map.insert(GamepadButtonType::DPadUp, UiAction::Up);
    input_map.insert(GamepadButtonType::DPadDown, UiAction::Down);
    input_map.insert(GamepadButtonType::South, UiAction::Select);
    input_map.insert(GamepadButtonType::Select, UiAction::Select);
    input_map.insert(GamepadButtonType::Start, UiAction::Start);
    commands.spawn(InputManagerBundle::<UiAction> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map,
    });
}
