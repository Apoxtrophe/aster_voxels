
// External crate imports
use bevy::{
    prelude::*, render::render_resource::{AddressMode, SamplerDescriptor}
};
use bevy_atmosphere::{plugin::AtmospherePlugin, model::AtmosphereModel};
use bevy_egui::EguiPlugin;
use bevy_rapier3d::{prelude::*, plugin::RapierConfiguration};
use bevy_fps_controller::controller::*;

// Local module imports
mod v_config;
mod v_graphics;
mod v_lib;
mod v_simulation;
mod v_structure;
mod a_loading;
mod v_debug;
mod v_selector;
mod b_voxel_setup;
mod v_components;
mod v_player2;
mod v_lighting;
mod v_hotbar;
mod v_graphics_helper;
mod v_main_menu;
mod v_save; 

// Using structs and enums directly from their modules
use a_loading::{voxel_loading, asset_check};
use b_voxel_setup::voxel_setup;
use v_config::SUN_TIMER_RATE;
use v_graphics::update_voxel_emissive;
use v_hotbar::{hotbar_ui, timer_update_system, voxel_descriptor};
use v_lighting::{daylight_cycle, CycleTimer};
use v_lib::update_info;
use v_main_menu::{main_menu_buttons, setup_main_menu};
use v_player2::{player_setup, manage_cursor, respawn, voxel_interaction_system};
use v_save::{check_for_save_input, world_loader};
use v_simulation::logic_operation_system;

// Application state definitions
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    AssetLoading,
    GameSetup,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(RapierConfiguration::default())
        .insert_resource(Msaa::Sample2)
        .insert_resource(AtmosphereModel::default()) 
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(SUN_TIMER_RATE), 
            TimerMode::Repeating,
        )))
        .add_plugins(
            DefaultPlugins
              .set(ImagePlugin {
                
                default_sampler: SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: bevy::render::render_resource::FilterMode::Nearest,
                min_filter: bevy::render::render_resource::FilterMode::Linear,
                mipmap_filter: bevy::render::render_resource::FilterMode::Linear,
                lod_min_clamp: 0.0,
                lod_max_clamp: 0.01,
                ..default()
                }.into(),
              }),
            )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FpsControllerPlugin)
        .add_plugins(AtmospherePlugin)
        .add_plugins(EguiPlugin)

        .add_state::<AppState>()
            
        .add_systems(Startup, setup_main_menu)
        .add_systems(Update, (
            main_menu_buttons,
        ).run_if(in_state(AppState::MainMenu)))
        
        // Asset Loading Systems
        .add_systems(OnEnter(AppState::AssetLoading), (
            voxel_loading, 
            player_setup,
        ))


        .add_systems(Update, (
             asset_check
            ).run_if(in_state(AppState::AssetLoading)))

        // Game-Setup Systems
        .add_systems(OnEnter(AppState::GameSetup), (
            voxel_setup, hotbar_ui, voxel_descriptor,
        ))

        // In-Game Systems

        .add_systems(OnEnter(AppState::InGame), world_loader)

        .add_systems(Update, (
            update_info, 
            manage_cursor, respawn, voxel_interaction_system, //Player systems
            daylight_cycle,
            check_for_save_input,
            timer_update_system,
            update_voxel_emissive,
            logic_operation_system,
        ).run_if(in_state(AppState::InGame)))

        .run();
}