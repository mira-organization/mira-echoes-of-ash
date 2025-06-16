#![feature(coverage_attribute)]

use bevy::prelude::{App, AssetServer, OnEnter, Plugin, Res, ResMut};
use bevy_kira_audio::{AudioPlugin, DynamicAudioChannels};
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::audio::{AudioManager, AudioOption, AudioType};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioOption::new());
        app.insert_resource(AudioManager::new());
        app.add_plugins(AudioPlugin);
        app.add_systems(OnEnter(GameState::SplashScreen), load_up_audio_config);
        app.add_systems(OnEnter(GameState::InGame), test_music);
    }
}

#[coverage(off)]
fn load_up_audio_config(general_config: Res<ConfigService>, mut audio_option: ResMut<AudioOption>) {
    audio_option.initialize(&general_config);
}

#[coverage(off)]
fn test_music(
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
    mut audio_manager: ResMut<AudioManager>,
    option: Res<AudioOption>
) {
    audio_manager.add_audio("title_music", AudioType::Environment, "audio/test.ogg", &mut audio, &asset_server, &option);
}