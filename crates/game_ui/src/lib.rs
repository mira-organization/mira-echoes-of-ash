#![feature(coverage_attribute)]

use bevy::prelude::*;
use bevy_extended_ui::ExtendedUiPlugin;
use bevy_extended_ui::html::HtmlSource;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtendedUiPlugin).add_systems(Startup, tested);
    }
}

#[coverage(off)]
fn tested(mut commands: Commands) {
    commands.spawn(HtmlSource(String::from("assets/html/test.html")));
}