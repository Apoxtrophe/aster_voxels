// For UI implementation see v_main_menu and v_in_game_menu

use std::fs;

use bevy::ecs::system::{Commands, Res, ResMut, Resource};
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub ui_scale: f32,
    pub screen_dimensions: (usize, usize),
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            ui_scale: 1.0,
            screen_dimensions: (1920, 1080),
        }
    }
}

pub fn save_settings(settings: ResMut<GlobalSettings>,) {
    match serde_json::to_string_pretty(&*settings) {
        Ok(serialized) => {
            if let Err(e) = fs::write("assets/Settings/settings.json", serialized) {
                eprintln!("Failed to save settings: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize settings: {}", e);
        }
    }
}

pub fn load_settings(mut settings: ResMut<GlobalSettings>) {
    match fs::read_to_string("assets/Settings/settings.json") {
        Ok(contents) => {
            match serde_json::from_str(&contents) {
                Ok(loaded_settings) => {
                    *settings = loaded_settings;
                }
                Err(e) => {
                    eprintln!("Failed to deserialize settings: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read settings file: {}", e);
        }
    }
}

pub fn reset_settings(
    mut commands: Commands,
) {
    commands.insert_resource(GlobalSettings::default());
}