use bevy::prelude::*;
use bevy_extended_ui::html::HtmlFunctionRegistry;
use bevy_extended_ui::observer::time_tick_trigger::TimeTick;
use bevy_extended_ui::widgets::ProgressBar;
use game_system::save_info::AssetLoadProgress;

pub struct LoadingScreenController;

impl Plugin for LoadingScreenController {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_functions);
    }
}

fn register_functions(mut functions: ResMut<HtmlFunctionRegistry>) {
    functions.update.insert("update_loading_bar".to_string(), update_loading_bar);
}

fn update_loading_bar(event: Trigger<TimeTick>, mut commands: Commands) {
    let target = event.target();

    commands.queue(move |world: &mut World| {
        let Some(progress_res) = world.get_resource::<AssetLoadProgress>() else {
            return;
        };

        let total = progress_res.total as f32;
        let loaded = progress_res.loaded as f32;
        let ratio = if total > 0.0 { loaded / total } else { 0.0 };
        
        let mut query = world.query_filtered::<(Entity, &mut ProgressBar), With<ProgressBar>>();

        for (entity, mut progress) in query.iter_mut(world) {
            if entity == target {
                progress.value = ratio * progress.max;
            }
        }
    });
}