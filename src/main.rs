// Importing necessary modules in consolidated format
mod v_config;
mod v_graphics;
mod v_lib;
mod v_performance;
mod v_player;
mod v_simulation;
mod v_structure;
mod a_loading;
mod v_debug;
mod v_selector;
mod v_bench;
mod b_voxel_setup;
mod v_components;

use bevy::
    prelude::*
;
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_egui::EguiPlugin;

// Using structs and enums directly from their modules
use a_loading::{voxel_loading, asset_check};
use b_voxel_setup::voxel_setup;
use v_bench::benchmark;
use v_performance::performance_metrics_system;
use v_lib::update_info;
use v_player::*;
use v_simulation::logic_operation_system;
use v_graphics::*;
use v_debug::*;





#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    AssetLoading, // Loading assets, meshes and initialzing resoruces
    GameSetup,    // Setting up the game world, creating lighting, the player etc. 
    InGame,       // The main game loop
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AtmospherePlugin)
        .add_plugins(EguiPlugin)

        .add_state::<AppState>()
        .add_systems(Startup, voxel_loading)
        .add_systems(Update, asset_check.run_if(in_state(AppState::AssetLoading)))
        .add_systems(OnEnter(AppState::GameSetup), voxel_setup)

        .add_systems(Update, update_info.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, voxel_interaction_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, performance_metrics_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, ui_debug.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_voxel_emissive.run_if(in_state(AppState::InGame)))
        .add_systems(Update, logic_operation_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, benchmark.run_if(in_state(AppState::InGame)))
        .run();
}