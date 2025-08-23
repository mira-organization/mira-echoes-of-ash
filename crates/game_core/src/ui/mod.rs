#![coverage(off)]

use bevy::prelude::*;

pub struct UiModule;

impl Plugin for UiModule {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<OpenUI>();
    }
}

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct OpenUI(pub UiType);

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum UiType {
    #[default]
    None,
    Inventory,
    Dialog,
    Pause,
    Settings
}