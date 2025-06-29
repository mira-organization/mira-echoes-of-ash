use bevy::prelude::*;
use bevy_extended_ui::html::HtmlFunctionRegistry;
use bevy_extended_ui::observer::time_tick_trigger::TimeTick;
use bevy_extended_ui::observer::widget_init_trigger::WidgetInit;
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::widgets::{Headline, Slider};
use game_system::models::audio::{ActualAudioOption, AudioOption};
use game_system::models::ui::{OpenUI, UiType};

pub struct MenuScreenController;

impl Plugin for MenuScreenController {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_functions);
    }
}

#[coverage(off)]
fn register_functions(mut functions: ResMut<HtmlFunctionRegistry>) {
    functions.click.insert("open_settings".to_string(), open_settings);
    functions.load.insert("load_master".to_string(), load_master);
    functions.load.insert("load_sfx".to_string(), load_sfx);
    functions.load.insert("load_ui".to_string(), load_ui);    
    functions.load.insert("load_character_voice".to_string(), load_character_voice);
    functions.load.insert("load_music".to_string(), load_environment);
    
    functions.update.insert("update_master".to_string(), update_master);
    functions.update.insert("update_music".to_string(), update_music);
    functions.update.insert("update_ui".to_string(), update_ui);
    functions.update.insert("update_sfx".to_string(), update_sfx);
    functions.update.insert("update_character".to_string(), update_character);
}

#[coverage(off)]
fn open_settings(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.queue(move |world: &mut World| {
        let open_ui = world.resource::<OpenUI>();
        if !open_ui.0.eq(&UiType::Settings) {
            let mut ui_registry = world.resource_mut::<UiRegistry>();
            ui_registry.use_ui("settings_screen");
        }
    });
}

// ==================================
//                Audio
// ==================================

/* ======================================================================
                                    Init
   ====================================================================== */

#[coverage(off)]
fn load_master(event: Trigger<WidgetInit>, mut commands: Commands) {
    let entity = event.target;
    commands.queue(move |world: &mut World| {
        let value = {
            let audio_option = world.resource::<AudioOption>();
            (audio_option.master_volume * 100.0) as f32
        };
        
        let mut query = world.query_filtered::<&mut Slider, With<Slider>>();
        if let Ok(mut slider) = query.get_mut(world, entity) {
            slider.value = value;
        }
    });
    
    text_change(&mut commands, "master_volume".to_string(), String::from("master"));
}

#[coverage(off)]
fn load_sfx(event: Trigger<WidgetInit>, mut commands: Commands) {
    let entity = event.target;
    commands.queue(move |world: &mut World| {
        let value = {
            let audio_option = world.resource::<AudioOption>();
            (audio_option.volumes.get("sfx").unwrap_or(&1.0) * 100.0) as f32
        };

        let mut query = world.query_filtered::<&mut Slider, With<Slider>>();
        if let Ok(mut slider) = query.get_mut(world, entity) {
            slider.value = value;
        }
    });

    text_change(&mut commands, "sfx_volume".to_string(), String::from("ui"));
}

#[coverage(off)]
fn load_ui(event: Trigger<WidgetInit>, mut commands: Commands) {
    let entity = event.target;
    commands.queue(move |world: &mut World| {
        let value = {
            let audio_option = world.resource::<AudioOption>();
            (audio_option.volumes.get("ui").unwrap_or(&1.0) * 100.0) as f32
        };

        let mut query = world.query_filtered::<&mut Slider, With<Slider>>();
        if let Ok(mut slider) = query.get_mut(world, entity) {
            slider.value = value;
        }
    });

    text_change(&mut commands, "ui_volume".to_string(), String::from("ui"));
}

#[coverage(off)]
fn load_character_voice(event: Trigger<WidgetInit>, mut commands: Commands) {
    let entity = event.target;
    commands.queue(move |world: &mut World| {
        let value = {
            let audio_option = world.resource::<AudioOption>();
            (audio_option.volumes.get("character_voice").unwrap_or(&1.0) * 100.0) as f32
        };

        let mut query = world.query_filtered::<&mut Slider, With<Slider>>();
        if let Ok(mut slider) = query.get_mut(world, entity) {
            slider.value = value;
        }
    });

    text_change(&mut commands, "character_volume".to_string(), String::from("character_voice"));
}

#[coverage(off)]
fn load_environment(event: Trigger<WidgetInit>, mut commands: Commands) {
    let entity = event.target;
    commands.queue(move |world: &mut World| {
        let value = {
            let audio_option = world.resource::<AudioOption>();
            (audio_option.volumes.get("environment").unwrap_or(&1.0) * 100.0) as f32
        };

        let mut query = world.query_filtered::<&mut Slider, With<Slider>>();
        if let Ok(mut slider) = query.get_mut(world, entity) {
            slider.value = value;
        }
    });

    text_change(&mut commands, "music_volume".to_string(), String::from("environment"));
}

/* ======================================================================
                                    Update
   ====================================================================== */

#[coverage(off)]
fn update_master(event: Trigger<TimeTick>, mut commands: Commands) {
    let target = event.target;
    commands.queue(move |world: &mut World| {
        let mut query = world.query_filtered::<&Slider, Changed<Slider>>();
        if let Ok(slider) = query.get_mut(world, target) {
            let slider_value = slider.value;
            
            let mut audio_option = world.resource_mut::<ActualAudioOption>();
            audio_option.master_volume = (slider_value / 100.0) as f64;
        }
    });

    text_change(&mut commands, "master_volume".to_string(), String::from("master"));
}

#[coverage(off)]
fn update_music(event: Trigger<TimeTick>, mut commands: Commands) {
    let target = event.target;
    commands.queue(move |world: &mut World| {
        let mut query = world.query_filtered::<&Slider, Changed<Slider>>();
        if let Ok(slider) = query.get_mut(world, target) {
            let slider_value = slider.value;

            let mut audio_option = world.resource_mut::<ActualAudioOption>();
            if let Some(value) = audio_option.volumes.get_mut("environment") {
                *value = (slider_value / 100.0) as f64;
            }
        }
    });

    text_change(&mut commands, "music_volume".to_string(), String::from("environment"));
}

#[coverage(off)]
fn update_ui(event: Trigger<TimeTick>, mut commands: Commands) {
    let target = event.target;
    commands.queue(move |world: &mut World| {
        let mut query = world.query_filtered::<&Slider, Changed<Slider>>();
        if let Ok(slider) = query.get_mut(world, target) {
            let slider_value = slider.value;

            let mut audio_option = world.resource_mut::<ActualAudioOption>();
            if let Some(value) = audio_option.volumes.get_mut("ui") {
                *value = (slider_value / 100.0) as f64;
            }
        }
    });

    text_change(&mut commands, "ui_volume".to_string(), String::from("ui"));
}

#[coverage(off)]
fn update_sfx(event: Trigger<TimeTick>, mut commands: Commands) {
    let target = event.target;
    commands.queue(move |world: &mut World| {
        let mut query = world.query_filtered::<&Slider, Changed<Slider>>();
        if let Ok(slider) = query.get_mut(world, target) {
            let slider_value = slider.value;

            let mut audio_option = world.resource_mut::<ActualAudioOption>();
            if let Some(value) = audio_option.volumes.get_mut("sfx") {
                *value = (slider_value / 100.0) as f64;
            }
        }
    });

    text_change(&mut commands, "sfx_volume".to_string(), String::from("sfx"));
}

#[coverage(off)]
fn update_character(event: Trigger<TimeTick>, mut commands: Commands) {
    let target = event.target;
    commands.queue(move |world: &mut World| {
        let mut query = world.query_filtered::<&Slider, Changed<Slider>>();
        if let Ok(slider) = query.get_mut(world, target) {
            let slider_value = slider.value;

            let mut audio_option = world.resource_mut::<ActualAudioOption>();
            if let Some(value) = audio_option.volumes.get_mut("character_voice") {
                *value = (slider_value / 100.0) as f64;
            }
        }
    });

    text_change(&mut commands, "character_volume".to_string(), String::from("character_voice"));
}

// ==================================
//         Internal Audio Fn
// ==================================

#[coverage(off)]
fn text_change(commands: &mut Commands, id: String, option: String) {
    commands.queue(move |world: &mut World| {
        let value = {
            let audio_option = world.resource::<AudioOption>();

            if option == "master" {
                (audio_option.master_volume * 100.0) as f32
            } else {
                (audio_option.volumes.get(&option).unwrap_or(&1.0) * 100.0) as f32
            }
        };

        let mut query = world.query_filtered::<(&mut Headline, &CssID), With<Headline>>();
        for (mut headline, css_id) in query.iter_mut(world) {
            if css_id.0.eq(id.as_str()) {
                headline.text = format!("{}%", value);
            }
        }
    });
}