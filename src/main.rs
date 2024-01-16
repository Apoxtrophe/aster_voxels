// Importing necessary modules in consolidated format
mod v_config;
mod v_graphics;
mod v_lib;
mod v_performance;
mod v_simulation;
mod v_structure;
mod a_loading;
mod v_debug;
mod v_selector;
mod v_bench;
mod b_voxel_setup;
mod v_components;
mod v_player2;
mod v_lighting;
use std::env;

use bevy::{
    prelude::*, render::render_resource::{SamplerDescriptor, AddressMode}}
;
use bevy_atmosphere::{plugin::AtmospherePlugin, model::AtmosphereModel};
use bevy_egui::EguiPlugin;

// Using structs and enums directly from their modules
use a_loading::{voxel_loading, asset_check};
use b_voxel_setup::voxel_setup;
use bevy_rapier3d::plugin::RapierConfiguration;
use v_bench::benchmark;
use v_lighting::{daylight_cycle, CycleTimer};
use v_performance::performance_metrics_system;
use v_lib::update_info;
use v_player2::{player_setup, manage_cursor, display_text, respawn, voxel_interaction_system};
use v_simulation::logic_operation_system;
use v_graphics::*;
use v_debug::*;




use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    AssetLoading, // Loading assets, meshes and initialzing resoruces
    GameSetup,    // Setting up the game world, creating lighting, the player etc. 
    InGame,       // The main game loop
}

fn main() {

    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(RapierConfiguration::default())
        .insert_resource(Msaa::Sample4)
        .insert_resource(AtmosphereModel::default()) 
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(50), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            TimerMode::Repeating,
        )))
        .add_plugins(
            DefaultPlugins
              .set(ImagePlugin {

                  default_sampler: SamplerDescriptor {
                  address_mode_u: AddressMode::Repeat,
                  address_mode_v: AddressMode::Repeat,
                  address_mode_w: AddressMode::Repeat,
                  ..default()
                }.into(),
              }),
            )

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FpsControllerPlugin)
        .add_plugins(AtmospherePlugin)
        .add_plugins(EguiPlugin)

        .add_state::<AppState>()
        .add_systems(Startup, voxel_loading)
        .add_systems(Startup, player_setup)


        // Asset Loading Systems
        .add_systems(Update, asset_check.run_if(in_state(AppState::AssetLoading)))

        // Game-Setup Systems
        .add_systems(OnEnter(AppState::GameSetup), voxel_setup)

        // In-Game Systems
        .add_systems(Update, (
            performance_metrics_system, ui_debug, update_info, benchmark, //Optional systems
            manage_cursor, display_text, respawn, voxel_interaction_system, //Player systems
            daylight_cycle,
            update_voxel_emissive,
            logic_operation_system,
        ).run_if(in_state(AppState::InGame)))
        .run();
}