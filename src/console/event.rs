use crate::console::ConsoleData;
use bevy::prelude::*;

pub struct PrintToConsoleEvent(pub String);

pub fn add_message_events_to_console(
    mut data: ResMut<ConsoleData>,
    mut ev_console_message: EventReader<PrintToConsoleEvent>,
) {
    for PrintToConsoleEvent(message) in ev_console_message.iter() {
        data.lines.push(message.clone());
    }
}
