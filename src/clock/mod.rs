mod event_scheduler;

use crate::clock::event_scheduler::EventSchedulerPlugin;
use crate::menu::LevelStart;
use crate::player::Vitality;
use crate::GameState;
use bevy::prelude::*;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EventSchedulerPlugin::<TimeEvent>::new())
            .insert_resource(TimeScale(1.))
            .insert_resource(TimeSinceLevelStart(0.))
            .add_system(update_time.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Moment<T> {
    pub timestamp: f32,
    pub data: T,
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct History<T> {
    pub moments: Vec<Moment<T>>,
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Resource)]
pub struct TimeSinceLevelStart(pub f32);

#[derive(Copy, Clone, PartialEq, Debug, Default, Resource)]
pub struct TimeScale(pub f32);

pub enum TimeEvent {
    Normal,
}

pub fn update_time(
    mut time_scale: ResMut<TimeScale>,
    mut time_since_level_start: ResMut<TimeSinceLevelStart>,
    bevy_time: Res<Time>,
    mut level_events: EventReader<LevelStart>,
    mut time_events: EventWriter<TimeEvent>,
    mut vitals: Query<&mut Vitality>,
    mut level_state: ResMut<NextState<GameState>>,
) {
    for _ in level_events.iter() {
        time_scale.0 = 1.;
        time_since_level_start.0 = 0.;
        time_events.send(TimeEvent::Normal);
        for mut vitality in vitals.iter_mut() {
            *vitality = Vitality::Alive;
        }
    }

    time_since_level_start.0 += bevy_time.delta_seconds() * time_scale.0;

    if time_since_level_start.0 < 0. {
        time_since_level_start.0 = 0.;
        time_scale.0 = 0.;
    }
    if time_since_level_start.0 > 180. {
        time_since_level_start.0 = 180.;
        time_scale.0 = 0.;
        for mut vitality in vitals.iter_mut() {
            *vitality = Vitality::Dead;
            level_state.set(GameState::GameOver)
        }
    }
}
