mod config;
mod voxels;
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_flycam::NoCameraPlayerPlugin;
use config::*;

mod voxel_structure;
use voxel_structure::*;

mod player;
use player::*;
use bevy::prelude::*;

use bevy::window::{Window, PresentMode, CursorIcon, CursorGrabMode, WindowResolution};
use bevy::window::PrimaryWindow;
use voxels::setup_voxel;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtmospherePlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_player)
        .add_systems(Startup, setup_voxel)
        .add_systems(Update, camera_rotation_system)
        .add_systems(Update, camera_movement_system)
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
    // VOXEL

    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.).into()),
            material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            ..default()
        },
        Ground,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Window settings
    let mut window = windows.single_mut();
    window.title = "Logica".to_string();
    window.resolution = WindowResolution::new(SCREEN_WIDTH,SCREEN_HEIGHT);
    window.present_mode = PresentMode::AutoVsync;
    window.cursor.icon = CursorIcon::Crosshair;
    window.cursor.grab_mode = CursorGrabMode::Locked;
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
}
