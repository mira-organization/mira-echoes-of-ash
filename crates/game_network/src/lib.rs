#![feature(coverage_attribute)]

mod channel;

use bevy::prelude::*;
use bevy_http_client::HttpClientPlugin;
use crate::channel::NetworkChannelModule;

pub struct GameNetworkPlugin;

impl Plugin for GameNetworkPlugin {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpClientPlugin);
        app.add_plugins(NetworkChannelModule);
    }
}