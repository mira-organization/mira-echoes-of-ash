use bevy::prelude::*;

pub mod client_conf;

pub struct ConfigModule;

impl Plugin for ConfigModule {
    #[coverage(off)]
    fn build(&self, _app: &mut App) {
    }
    
}