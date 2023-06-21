use crate::GameState;
use bevy::asset::Error;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde;
use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;
use std::collections::HashMap;
use std::fmt;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<GameDataAssetDynamicCollection>::new(&[
            "game-data.ron",
        ]))
        .add_asset::<Question>()
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu))
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading)
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetDynamicCollection>(
            GameState::Loading,
            "data/trivia.game-data.ron",
        )
        .add_collection_to_loading_state::<_, QuestionAssets>(GameState::Loading);
    }
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "2df00c92-cf7b-42c1-a989-dccbad659c13"]
pub struct GameDataAssetDynamicCollection(HashMap<String, GameDataAsset>);

impl DynamicAssetCollection for GameDataAssetDynamicCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, asset) in self.0.iter() {
            dynamic_assets.register_asset(key, Box::new(asset.clone()))
        }
    }
}

impl DynamicAsset for GameDataAsset {
    fn load(&self, _asset_server: &AssetServer) -> Vec<HandleUntyped> {
        match self {
            GameDataAsset::Question { .. } => vec![],
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, Error> {
        let cell = world.cell();

        match self {
            GameDataAsset::Question {
                description,
                options,
                answer,
            } => {
                let mut questions = cell
                    .get_resource_mut::<Assets<Question>>()
                    .expect("Failed to get question asset");

                let handle = questions
                    .add(Question {
                        description: description.clone(),
                        options: options.clone(),
                        answer: answer.clone(),
                        used: false,
                    })
                    .clone_untyped();
                Ok(DynamicAssetType::Single(handle))
            }
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
enum GameDataAsset {
    Question {
        description: String,
        options: Vec<String>,
        answer: String,
    },
}

#[derive(TypeUuid, Clone, PartialEq, Eq, Hash, Debug)]
#[uuid = "2a897eae-c084-4fcd-bfb4-f54c64d2895e"]
pub struct Question {
    pub description: String,
    pub options: Vec<String>,
    pub answer: String,
    pub used: bool,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n{:?}", self.answer, self.description)
    }
}

#[derive(AssetCollection, Resource)]
pub struct QuestionAssets {
    #[asset(key = "pac_man")]
    pub pac_man: Handle<Question>,
    #[asset(key = "frogger")]
    pub frogger: Handle<Question>,
    #[asset(key = "yoshi")]
    pub yoshi: Handle<Question>,
    #[asset(key = "pong")]
    pub pong: Handle<Question>,
    #[asset(key = "nintendo_wii")]
    pub nintendo_wii: Handle<Question>,
    #[asset(key = "pokemon_go")]
    pub pokemon_go: Handle<Question>,
    #[asset(key = "duck_hunt")]
    pub duck_hunt: Handle<Question>,
    #[asset(key = "clyde")]
    pub clyde: Handle<Question>,
    #[asset(key = "chuck_e_cheese")]
    pub chuck_e_cheese: Handle<Question>,
    #[asset(key = "the_sims")]
    pub the_sims: Handle<Question>,
    #[asset(key = "bulbasaur")]
    pub bulbasaur: Handle<Question>,
    #[asset(key = "first_bash")]
    pub first_bash: Handle<Question>,
    #[asset(key = "mega_event")]
    pub mega_event: Handle<Question>,
    #[asset(key = "golf_ball")]
    pub golf_ball: Handle<Question>,
    #[asset(key = "area_51")]
    pub area_51: Handle<Question>,
    #[asset(key = "pennsylvania")]
    pub pennsylvania: Handle<Question>,
    #[asset(key = "wauseon")]
    pub wauseon: Handle<Question>,
    #[asset(key = "tin_foil_hat")]
    pub tin_foil_hat: Handle<Question>,
    #[asset(key = "hat_on")]
    pub hat_on: Handle<Question>,
    #[asset(key = "swag")]
    pub swag: Handle<Question>,
    #[asset(key = "favorited")]
    pub favorited: Handle<Question>,
    #[asset(key = "orginial")]
    pub orginial: Handle<Question>,
    #[asset(key = "no_geobash")]
    pub no_geobash: Handle<Question>,
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

    #[asset(path = "fonts/PICO-8 mono.ttf")]
    pub pico: Handle<Font>,

    #[asset(path = "fonts/prstartk.ttf")]
    pub pixel_font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/menu_background.png")]
    pub menu_background: Handle<Image>,

    #[asset(path = "textures/button.png")]
    pub button: Handle<Image>,

    #[asset(path = "textures/crt.png")]
    pub crt_overlay: Handle<Image>,

    #[asset(path = "icons/full_clock_alt.png")]
    pub clock: Handle<Image>,

    #[asset(path = "icons/half_heart.png")]
    pub half_heart: Handle<Image>,

    #[asset(path = "icons/heart.png")]
    pub heart: Handle<Image>,

    #[asset(path = "icons/empty_heart_container.png")]
    pub empty_heart: Handle<Image>,

    #[asset(path = "icons/potion.png")]
    pub potion: Handle<Image>,

    #[asset(path = "icons/coin.png")]
    pub coin: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    // #[asset(path = "textures/level.ldtk")]
    #[asset(path = "textures/level_alt.ldtk")]
    pub level: Handle<LdtkAsset>,
}
