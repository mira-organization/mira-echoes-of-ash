use std::collections::HashMap;
use std::time::Duration;
use bevy::asset::AssetServer;
use bevy::log::{debug, warn};
use bevy::prelude::{Handle, Resource};
use bevy_kira_audio::prelude::*;
use crate::config::ConfigService;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AudioType {
    Environment,
    Sfx,
    Ui,
    Character,
    #[default]
    Other
}

impl AudioType {

    #[coverage(off)]
    pub fn from_string(string: &str) -> Self {
        match string.to_lowercase().as_str() {
            "environment" => Self::Environment,
            "sfx" => Self::Sfx,
            "ui" => Self::Ui,
            "character" => Self::Character,
            _ => Self::Other
        }
    }
    
}

#[derive(Resource)]
pub struct AudioManager {
    pub muted: bool,
    pub audio: HashMap<String, AudioType>,
    pub audio_handles: HashMap<String, Handle<AudioSource>>
}

impl AudioManager {

    #[coverage(off)]
    pub fn new() -> Self {
        Self {
            muted: false,
            audio: HashMap::new(),
            audio_handles: HashMap::new()
        }
    }

    #[coverage(off)]
    pub fn add_audio(
        &mut self,
        channel_name: &str,
        audio_type: AudioType,
        path: &str,
        audio_dyn_channels: &mut DynamicAudioChannels,
        asset_server: &AssetServer,
        option: &AudioOption
    ) {
        if self.audio.contains_key(channel_name) {
            warn!("Audio channel {} already exists", channel_name);
            return;
        }
        
        let looped = self.looped_time(audio_type.clone());
        let volume = self.load_correct_volume(audio_type.clone(), option);
        
        let handle = asset_server.load::<AudioSource>(path);

        let mut binding = audio_dyn_channels.create_channel(channel_name)
            .play(handle.clone());

        let build = binding
            .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear))
            .with_volume(volume);

        if looped { build.looped(); }
        self.audio.insert(channel_name.to_string(), audio_type);
        self.audio_handles.insert(channel_name.to_string(), handle.clone());
        debug!("Audio channel {} added", channel_name);
    }

    #[coverage(off)]
    pub fn remove_audio(
        &mut self,
        channel_name: &str,
        audio_dyn_channels: &mut DynamicAudioChannels,
    ) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio channel {} does not exist", channel_name);
            return;
        }
        
        audio_dyn_channels.remove_channel(channel_name);
        self.audio_handles.remove(channel_name);
        self.audio.remove(channel_name);
        debug!("Audio channel {} removed", channel_name);
    }

    #[coverage(off)]
    pub fn stop_audio(
        &mut self, 
        channel_name: &str, 
        audio_dyn_channels: &mut DynamicAudioChannels
    ) {
        if !self.audio.contains_key(channel_name) || audio_dyn_channels.get_channel(channel_name).is_none() {
            warn!("Audio channel {} does not exist", channel_name);
            return;
        }
        
        audio_dyn_channels.channel(channel_name).stop().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
        debug!("Audio channel {} stopped", channel_name);
    }

    #[coverage(off)]
    pub fn play_audio(
        &mut self,
        channel_name: &str,
        audio_dyn_channels: &mut DynamicAudioChannels,
        option: &AudioOption
    ) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio channel {} does not exist", channel_name);
            return;
        }
        
        if let Some(handle) = self.audio_handles.get(channel_name) {
            let audio_type = self.audio.get(channel_name).unwrap();
            let looped = self.looped_time(audio_type.clone());
            let volume = self.load_correct_volume(audio_type.clone(), option);
            
            let mut binding = audio_dyn_channels.channel(channel_name)
                .play(handle.clone());
            
            let build = binding
                .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear))
                .with_volume(volume);
            
            if looped { build.looped(); }
            debug!("Audio channel {} played", channel_name);
        }
    }

    #[coverage(off)]
    pub fn pause_audio(
        &mut self,
        channel_name: &str,
        audio_dyn_channels: &mut DynamicAudioChannels
    ) {
        if !self.audio.contains_key(channel_name) || audio_dyn_channels.get_channel(channel_name).is_none() {
            warn!("Audio channel {} does not exist", channel_name);
            return;
        }

        audio_dyn_channels.channel(channel_name).pause().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
        debug!("Audio channel {} paused", channel_name);
    }

    #[coverage(off)]
    pub fn resume_audio(
        &mut self,
        channel_name: &str,
        audio_dyn_channels: &mut DynamicAudioChannels
    ) {
        if !self.audio.contains_key(channel_name) || audio_dyn_channels.get_channel(channel_name).is_none() {
            warn!("Audio channel {} does not exist", channel_name);
            return;
        }
        
        audio_dyn_channels.channel(channel_name).resume().fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
        debug!("Audio channel {} resumed", channel_name);
    }

    #[coverage(off)]
    pub fn looped_time(&self, audio_type: AudioType) -> bool {
        match audio_type {
            AudioType::Environment => true,
            _ => false
        }
    }

    #[coverage(off)]
    pub fn contains_channel(&self, channel_name: &str) -> bool {
        self.audio.contains_key(channel_name)
    }

    #[coverage(off)]
    fn load_correct_volume(&self, audio_type: AudioType, audio_option: &AudioOption) -> f64 {
        match audio_type {
            AudioType::Environment => {
                (*audio_option.volumes.get("environment")
                    .unwrap_or(&1.0) * audio_option.master_volume).clamp(0.0, 1.0)
            },
            AudioType::Sfx => {
                (*audio_option.volumes.get("sfx").unwrap_or(&1.0)
                    * audio_option.master_volume).clamp(0.0, 1.0)
            },
            AudioType::Ui => {
                (*audio_option.volumes.get("ui").unwrap_or(&1.0)
                    * audio_option.master_volume).clamp(0.0, 1.0)
            },
            AudioType::Character => {
                (*audio_option.volumes.get("character_voice").unwrap_or(&1.0)
                    * audio_option.master_volume).clamp(0.0, 1.0)
            },
            AudioType::Other => 0.1,
        }
    }
}

#[derive(Resource)]
pub struct ActualAudioOption {
    pub master_volume: f64,
    pub volumes: HashMap<String, f64>
}

impl Default for ActualAudioOption {
    fn default() -> Self {
        Self {
            master_volume: -1.0,
            volumes: HashMap::new()
        }
    }
}

#[derive(Resource)]
pub struct AudioOption {
    pub master_volume: f64,
    pub volumes: HashMap<String, f64>
}

impl Default for AudioOption {

    #[coverage(off)]
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOption {

    #[coverage(off)]
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            volumes: HashMap::new()
        }
    }

    /// Initializes the `AudioOption` struct with values from a configuration service.
    /// The method sets the `master_volume` and individual category volumes based on the provided config.
    /// All values are clamped between 0.0 and 1.0 to ensure valid audio levels.
    ///
    /// # Parameters:
    /// - `config`: A reference to the `ConfigService` that provides audio configuration values.
    #[coverage(off)]
    pub fn initialize(&mut self, config: &ConfigService) {
        self.master_volume = config.audio_config.master_volume.clamp(0.0, 1.0);

        self.volumes.insert("environment".to_string(), config.audio_config.environment_volume.clamp(0.0, 1.0));
        self.volumes.insert("character_voice".to_string(), config.audio_config.character_voice_volume.clamp(0.0, 1.0));
        self.volumes.insert("sfx".to_string(), config.audio_config.sfx_volume.clamp(0.0, 1.0));
        self.volumes.insert("ui".to_string(), config.audio_config.ui_volume.clamp(0.0, 1.0));
    }

    /// Sets the primary volume and applies the updated volume settings to the audio channels.
    /// This method will ensure the primary volume is applied across all audio categories.
    ///
    /// # Parameters:
    /// - `volume`: The new primary volume value to set.
    /// - `audio_kira_handle`: A mutable reference to `DynamicAudioChannels` to manage dynamic audio channels.
    /// - `audio_manager`: A reference to the `AudioManager` that handles the audio channels.
    #[coverage(off)]
    pub fn set_master_volume(&mut self, volume: f64, audio_dyn_channels: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        self.master_volume = (volume.clamp(0.0, 1.0) * 100.0).round() / 100.0;
        self.apply_volumes(audio_dyn_channels, audio_manager);
    }

    /// Sets the volume for a specific audio category and applies the updated volume settings.
    ///
    /// # Parameters:
    /// - `category`: The name of the audio category (e.g., "environment", "sfx", etc.).
    /// - `volume`: The volume to set for the given category.
    /// - `audio_kira_handle`: A mutable reference to `DynamicAudioChannels` to manage dynamic audio channels.
    /// - `audio_manager`: A reference to the `AudioManager` that handles the audio channels.
    #[coverage(off)]
    pub fn set_category_volume(&mut self, category: &str, volume: f64, audio_dyn_channels: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        if let Some(ch) = self.volumes.get_mut(category) {
            *ch = (volume.clamp(0.0, 1.0) * 100.0).round() / 100.0;
            self.apply_volumes(audio_dyn_channels, audio_manager);
        }
    }

    /// Applies the volume settings to all audio channels by adjusting each channel's volume according to the individual category volumes.
    /// The volume for each channel is calculated by multiplying the category volume by the primary volume.
    /// This method ensures all channels are updated with the new volume settings.
    ///
    /// # Parameters:
    /// - `audio_kira_handle`: A mutable reference to `DynamicAudioChannels` to manage dynamic audio channels.
    /// - `audio_manager`: A reference to the `AudioManager` that handles the audio channels.
    #[coverage(off)]
    fn apply_volumes(&self, audio_dyn_channels: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        for (audio_type, volume) in &self.volumes {
            for (channel, audio_type_enum) in audio_manager.audio.clone() {
                if AudioType::from_string(audio_type) == audio_type_enum {
                    if let Some(audio) = audio_dyn_channels.get_channel(channel.as_str()) {
                        let insert = (volume * self.master_volume * 100.0).round() / 100.0;

                        debug!("Value: {}, channel: {:?}", insert, channel);

                        audio.set_volume(insert)
                            .fade_in(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
                    }
                }
            }
        }
    }
}