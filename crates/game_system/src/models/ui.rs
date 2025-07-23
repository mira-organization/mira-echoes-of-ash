use bevy::prelude::*;

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