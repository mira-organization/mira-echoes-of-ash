use bevy::prelude::*;
use bevy_kira_audio::DynamicAudioChannels;
use game_system::config::ConfigService;
use game_system::models::audio::{ActualAudioOption, AudioManager, AudioOption};

pub struct UpdateService;

impl Plugin for UpdateService {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_actual_option);
        app.add_systems(Update, update_master_volume.run_if(resource_changed::<ActualAudioOption>));
        app.add_systems(Update, update_category_volume.run_if(resource_changed::<ActualAudioOption>));
    }
}

#[coverage(off)]
fn setup_actual_option(mut actual_option: ResMut<ActualAudioOption>) {
    actual_option.master_volume = -1.0;
    actual_option.volumes.insert("environment".to_string(), -1.0);
    actual_option.volumes.insert("character_voice".to_string(), -1.0);
    actual_option.volumes.insert("sfx".to_string(), -1.0);
    actual_option.volumes.insert("ui".to_string(), -1.0);
}

#[coverage(off)]
fn update_master_volume(
    mut audio_option: ResMut<AudioOption>,
    actual_option: Res<ActualAudioOption>,
    audio_manager: Res<AudioManager>,
    mut audio_channels: ResMut<DynamicAudioChannels>,
    mut general_config: ResMut<ConfigService>
) {
    if actual_option.master_volume != -1.0 {
        audio_option.set_master_volume(actual_option.master_volume, &mut *audio_channels, &*audio_manager);
        general_config.audio_config.master_volume = actual_option.master_volume;
    }
}

#[coverage(off)]
fn update_category_volume(
    mut audio_option: ResMut<AudioOption>,
    actual_option: Res<ActualAudioOption>,
    audio_manager: Res<AudioManager>,
    mut audio_channels: ResMut<DynamicAudioChannels>,
    mut general_config: ResMut<ConfigService>
) {
    if actual_option.volumes.is_empty() {
        return;
    }

    let categories = ["environment", "character_voice", "sfx", "ui"];

    for category in categories {

        let value = *actual_option.volumes.get(category).unwrap_or_else(|| &-1.0);
        if value != -1.0 {
            audio_option.set_category_volume(
                category,
                value,
                &mut *audio_channels,
                &audio_manager,
            );
            match category {
                "environment" => general_config.audio_config.environment_volume = value,
                "character_voice" => general_config.audio_config.character_voice_volume = value,
                "sfx" => general_config.audio_config.sfx_volume = value,
                "ui" => general_config.audio_config.ui_volume = value,
                _ => { error!("No config found for category: {}", category) }
            }
        }
    }
}