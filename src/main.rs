mod config;
use bevy_atmosphere::plugin::AtmospherePlugin;
use config::*;

mod voxel_structure;
mod voxel_assets;

mod player;
use player::*;
use bevy::prelude::*;

use bevy::window::{Window, PresentMode, CursorIcon, CursorGrabMode, WindowResolution, WindowMode};
use bevy::window::PrimaryWindow;

use crate::voxel_assets::VoxelAssets;
use crate::voxel_structure::VoxelWorld;
use crate::voxel_structure::Voxel;
use crate::voxel_structure::VoxelType;
use core::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtmospherePlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_player)
        //.add_systems(Startup, voxel_startup)
        .add_systems(Update, camera_rotation_system)
        .add_systems(Update, camera_movement_system)
        .add_systems(Update, voxel_place_system)
        .run();
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    //SUN
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 2.0,0.0),
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

    // Window settings
    let mut window = windows.single_mut();
    window.title = "Logica".to_string();
    window.resolution = WindowResolution::new(SCREEN_WIDTH,SCREEN_HEIGHT);
    window.present_mode = PresentMode::AutoVsync;
    window.cursor.icon = CursorIcon::Crosshair;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.mode = WindowMode::Fullscreen;
    window.cursor.visible = false;

    // Crosshair
    let texture_handle = asset_server.load("Crosshair.png");
    let cloned_handle = texture_handle.clone();
    commands.spawn(ImageBundle {
        image: UiImage { texture: (cloned_handle), flip_x: (false), flip_y: (false)},
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            left: Val::Px((SCREEN_WIDTH / 2.0) - 250.0),
            top: Val::Px((SCREEN_HEIGHT / 2.0) - 250.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Create materials for tiles and wires
    let voxel_assets = VoxelAssets::new(&mut materials, &mut meshes);

    // Initialize the voxel world
    let mut voxel_world = VoxelWorld::new();


    voxel_world.set_voxel(
        &mut commands,
        IVec3::new(4, 6, 7),
        Voxel { voxel_type: VoxelType::Wire, is_on: false },
        voxel_assets.voxel_mesh.clone(),
        voxel_assets.wire_material.clone(),
    );

    commands.insert_resource(voxel_world);

    commands.insert_resource(voxel_assets);
}
