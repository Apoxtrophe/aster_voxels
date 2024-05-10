use bevy::{
    prelude::*,
    render::render_resource::{AddressMode, SamplerDescriptor},
};
use bevy_atmosphere::{model::AtmosphereModel, plugin::AtmospherePlugin};
use bevy_egui::EguiPlugin;
use bevy_fps_controller::controller::*;
use bevy_rapier3d::{plugin::RapierConfiguration, prelude::*};
mod a_loading;
mod b_voxel_setup;
mod v_components;
mod v_config;
mod v_graphics;
mod v_graphics_helper;
mod v_hotbar;
mod v_in_game_menu;
mod v_lib;
mod v_lighting;
mod v_main_menu;
mod v_player2;
mod v_pre_main_menu;
mod v_save;
mod v_selector;
mod v_simulation;
mod v_structure;
mod v_widgets;
use a_loading::{asset_check, voxel_loading};
use b_voxel_setup::voxel_setup;
use v_config::SUN_TIMER_RATE;
use v_graphics::update_voxel_emissive;
use v_hotbar::{hotbar_ui, timer_update_system, voxel_descriptor};
use v_in_game_menu::in_game_menu;
use v_lib::update_info;
use v_lighting::{daylight_cycle, CycleTimer};
use v_main_menu::{
    load_world_menu, main_menu_buttons, setup_main_menu, setup_world_naming, world_naming,
    SelectedWorld, WorldName,
};
use v_player2::{manage_cursor, player_setup, respawn, voxel_interaction_system};
use v_pre_main_menu::pre_main_menu_cleanup;
use v_save::{autosave_system, check_for_save_input, world_loader, SaveEvent};
use v_simulation::logic_operation_system;
use v_widgets::{simulation_speed_widget, SaveNotificationPlugin};

// Application state definitions
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    PreMainMenu, //Cleaning up entities and preparing clean slate for main menu
    MainMenu,      // Main menu handling
    WorldNaming,   // World creation
    LoadWorldMenu, // World Loading
    AssetLoading,  // Loading Assets
    GameSetup,     // Adding assets to world
    InGame,        // In game loop
}

fn main() {
    App::new()
        .insert_resource(WorldName::default())
        .insert_resource(SelectedWorld::default())
        .insert_resource(RapierConfiguration::default())
        .insert_resource(Msaa::Sample2)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(SUN_TIMER_RATE),
            TimerMode::Repeating,
        )))
        .add_plugins(
            DefaultPlugins.set(ImagePlugin {
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
                }
                .into(),
            }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FpsControllerPlugin)
        .add_plugins(AtmospherePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(SaveNotificationPlugin)
        .add_event::<SaveEvent>()
        .init_state::<AppState>()
        .add_systems(Startup, pre_main_menu_cleanup)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(
            Update,
            main_menu_buttons.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnEnter(AppState::WorldNaming), setup_world_naming)
        .add_systems(Update, world_naming.run_if(in_state(AppState::WorldNaming)))
        .add_systems(
            Update,
            load_world_menu.run_if(in_state(AppState::LoadWorldMenu)),
        )
        .add_systems(
            OnEnter(AppState::AssetLoading),
            (voxel_loading, player_setup),
        )
        .add_systems(Update, asset_check.run_if(in_state(AppState::AssetLoading)))
        .add_systems(
            OnEnter(AppState::GameSetup),
            (voxel_setup, hotbar_ui, voxel_descriptor),
        )
        .add_systems(OnEnter(AppState::InGame), world_loader)
        .add_systems(
            Update,
            (
                in_game_menu,
                simulation_speed_widget,
                update_info,
                manage_cursor,
                respawn,
                voxel_interaction_system,
                daylight_cycle,
                check_for_save_input,
                timer_update_system,
                update_voxel_emissive,
                logic_operation_system,
                autosave_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}
