#![feature(coverage_attribute)]

mod save_data_service;

use bevy::prelude::*;
use bevy_http_client::HttpClientPlugin;
use crate::save_data_service::NetworkGetController;

pub struct GameNetworkPlugin;

impl Plugin for GameNetworkPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpClientPlugin);
        app.add_plugins(NetworkGetController);
    }
}