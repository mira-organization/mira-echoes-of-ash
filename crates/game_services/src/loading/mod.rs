pub mod pre_load;
mod load_state;
mod post_load;

use bevy::prelude::*;
use crate::loading::load_state::LoadStateService;
use crate::loading::post_load::PostLoadService;
use crate::loading::pre_load::PreLoadService;

pub struct LoadingService;

impl Plugin for LoadingService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((PreLoadService, LoadStateService, PostLoadService));
    }
}