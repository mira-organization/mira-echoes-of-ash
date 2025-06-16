mod pre_load_service;
mod load_service;

use bevy::prelude::*;
use crate::service::load_service::LoadService;
use crate::service::pre_load_service::PreLoadService;

pub struct ServicePlugin;

impl Plugin for ServicePlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((PreLoadService, LoadService));
    }
}