// Importing necessary modules
mod v_config;
mod v_graphics;
mod v_lib;
mod v_player;
mod v_simulation;
mod v_structure;
mod v_performance;


use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// Using structs and enums directly from their modules
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::window::{
    CursorGrabMode, CursorIcon, PresentMode, PrimaryWindow, Window, WindowMode, WindowResolution,
};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_egui::EguiPlugin;
use v_bench::benchmark;
use v_performance::{ PerformanceMetrics, performance_metrics_system};
use core::f32::consts::PI;

use std::time::Duration;
use v_config::*;
use v_lib::{update_info, VoxelInfo};
use v_player::*;
use v_simulation::{logic_operation_system, MyTimer};
use v_structure::Voxel;

use v_graphics::*;
mod v_debug;
use v_debug::*;
mod v_selector;
use v_selector::*;
mod v_bench;

#[derive(Component)]
pub struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AtmospherePlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_player)
        .add_systems(Update, update_info)
        .add_systems(Update, player_system)
        .add_systems(Update, voxel_interaction_system)
        .add_systems(Update, performance_metrics_system)
        .add_systems(Update, ui_debug)
        .add_systems(Update, update_voxel_emissive)
        .add_systems(Update, logic_operation_system)
        .add_systems(Update, benchmark)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.2, 0.1),
        perceptual_roughness: 0.95, // Adjust this value to make the ground appear rougher
        metallic: 0.0,
        ..Default::default()
    });

    // Spawn the ground entity with the rough material
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(200.).into()),
            material: ground_material,
            ..Default::default()
        },
        Ground,
    ));

    //SUN
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 1000.0,
            ..default()
        }
        .into(),
        ..default()
    });
    // Ambient lighting
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.3; // Adjust the brightness as needed

    // Window settings
    let mut window = windows.single_mut();
    window.title = "Logica".to_string();
    window.resolution = WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    window.present_mode = PresentMode::AutoVsync;
    window.cursor.icon = CursorIcon::Crosshair;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.mode = WindowMode::Windowed;
    window.cursor.visible = false;
    window.decorations = true;

    // Crosshair
    let texture_handle = asset_server.load("Crosshair.png");
    let cloned_handle = texture_handle.clone();
    commands.spawn(ImageBundle {
        image: UiImage {
            texture: (cloned_handle),
            flip_x: (false),
            flip_y: (false),
        },
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            left: Val::Px((SCREEN_WIDTH / 2.0) - 32.0),
            top: Val::Px((SCREEN_HEIGHT / 2.0) - 32.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Create materials for tiles and wires
    let voxel_assets = VoxelAssets::new(asset_server, &mut meshes, &mut materials);

    // Initialize the voxel world
    commands.insert_resource(Voxel::new());

    commands.insert_resource(voxel_assets);

    commands.insert_resource(VoxelSelector::new());

    commands.insert_resource(VoxelInfo::new());

    commands.insert_resource(MyTimer(Timer::new(
        Duration::from_millis(LOGIC_RATE),
        TimerMode::Repeating,
    )));

    commands.insert_resource(PerformanceMetrics::new());
}
