use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,

    #[asset(path = "fonts/VT323-Regular.ttf")]
    pub crt_font: Handle<Font>,

    #[asset(path = "fonts/monogram.ttf")]
    pub monogram: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/full_clock.png")]
    pub clock: Handle<Image>,

    #[asset(path = "textures/crt.png")]
    pub crt_overlay: Handle<Image>,

    #[asset(path = "icons/heart.png")]
    pub heart: Handle<Image>,

    #[asset(path = "icons/empty_heart_container.png")]
    pub empty_heart: Handle<Image>,

    #[asset(path = "icons/potion.png")]
    pub potion: Handle<Image>,

    #[asset(path = "icons/coin.png")]
    pub coin: Handle<Image>,

    #[asset(path = "text/tile-0.png")]
    pub text0: Handle<Image>,

    #[asset(path = "text/tile-1.png")]
    pub text1: Handle<Image>,

    #[asset(path = "text/tile-2.png")]
    pub text2: Handle<Image>,

    #[asset(path = "text/tile-3.png")]
    pub text3: Handle<Image>,

    #[asset(path = "text/tile-4.png")]
    pub text4: Handle<Image>,

    #[asset(path = "text/tile-5.png")]
    pub text5: Handle<Image>,

    #[asset(path = "text/tile-6.png")]
    pub text6: Handle<Image>,

    #[asset(path = "text/tile-7.png")]
    pub text7: Handle<Image>,

    #[asset(path = "text/tile-8.png")]
    pub text8: Handle<Image>,

    #[asset(path = "text/tile-9.png")]
    pub text9: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    // #[asset(path = "textures/level.ldtk")]
    #[asset(path = "level/level.ldtk")]
    pub level: Handle<LdtkAsset>,
}
