// For UI implementation see v_main_menu and v_in_game_menu

use std::{fs, slice::Windows};

use bevy::{app::{App, Startup}, ecs::{entity::Entity, query::With, system::{Commands, NonSend, Query, Res, ResMut, Resource}}, log::tracing_subscriber::Layer, transform::commands, utils::info, window::{MonitorSelection, PrimaryWindow, Window, WindowPosition}, winit::WinitWindows};
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub ui_scale: f32,
    pub screen_dimensions: (u32, u32),
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



pub fn print_monitor_size(
    winit_windows: NonSend<WinitWindows>, 
    window_query: Query<Entity, With<PrimaryWindow>>
) -> (u32,u32) {
    if let Some(monitor) = window_query
        .get_single()
        .ok()
        .and_then(|entity| winit_windows.get_window(entity))
        .and_then(|winit_window| winit_window.current_monitor())
    {
        monitor.size().into()
    } else {
        (1920, 1080)
    }

}


pub fn update_global_screen(
    winit_windows: NonSend<WinitWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,  
    mut commands: Commands,
) {
    if let Some(monitor) = window_query
        .get_single()
        .ok()
        .and_then(|entity| winit_windows.get_window(entity))
        .and_then(|winit_window| winit_window.current_monitor())
    {
        println!("Monitor Size: {:?}", monitor.size());

        let monitor_size = monitor.size();
        commands.insert_resource(GlobalSettings{
            screen_dimensions: (monitor_size.width, monitor_size.height),
            ..Default::default()
        });
    }
}