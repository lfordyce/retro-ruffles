use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<Background>()
            .add_system(start_background.in_schedule(OnEnter(GameState::Menu)));
    }
}

#[derive(Resource)]
pub struct Background;

pub fn start_background(audio: Res<AudioChannel<Background>>, audio_assets: Res<AudioAssets>) {
    audio.play(audio_assets.music.clone()).looped();
}
