use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct OpenUI(pub KnownUi);

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum KnownUi {
    #[default]
    None,
    Inventory,
    Pause
}