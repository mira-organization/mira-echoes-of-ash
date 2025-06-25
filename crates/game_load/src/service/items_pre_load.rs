use std::fs::{read_dir, read_to_string};
use std::path::Path;
use bevy::prelude::*;
use game_system::models::inventory::{GameItemList, ItemTable};

pub struct ItemsPreLoadService;

impl Plugin for ItemsPreLoadService {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_item_tables);
    }
}

#[coverage(off)]
fn load_item_tables(mut game_item_list: ResMut<GameItemList>) {
    debug!("Loading item tables...");
    let dir_path = Path::new("assets/item_tables");
    
    let entries = match read_dir(dir_path) { 
        Ok(entries) => entries,
        Err(err) => {
            error!("Failed to read item tables directory: {}", err);
            return;
        }
    };
    
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        
        let file_content = match read_to_string(&path) { 
            Ok(content) => content,
            Err(err) => {
                error!("Failed to read item table file: {}", err);
                continue;
            }
        };
        
        let item_tables: ItemTable = match serde_json::from_str(&file_content) { 
            Ok(item) => item,
            Err(err) => {
                error!("Failed to parse item table file: {}", err);
                continue;
            }
        };
        
        for item in item_tables.entries {
            game_item_list.0.insert(item.name.clone(), item);   
        }
    }
    
    debug!("Finished loading item tables. [ {} ]", game_item_list.0.len());
}