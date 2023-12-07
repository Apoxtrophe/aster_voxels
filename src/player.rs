// Bevy-related imports
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_atmosphere::prelude::*;
use super::voxel_resources::*;
// Voxel assets and configuration

use super::config::*;
use super::voxel_lib::*;

pub fn create_player(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })  
    .insert(CameraRotation { pitch: 0.0, yaw: 0.0 })
    .insert(AtmosphereCamera::default());
}

pub fn player_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraRotation), With<Camera>>,
    mut voxel_selector: ResMut<VoxelSelector>,
    mouse_wheel_events: EventReader<MouseWheel>,
) {
    for (mut transform, mut rotation) in query.iter_mut() {
        // Handle Camera Rotation
        for event in mouse_motion_events.read() {
            rotation.yaw -= event.delta.x * MOUSE_SENSITIVITY;
            rotation.pitch = (rotation.pitch - event.delta.y * MOUSE_SENSITIVITY).clamp(-89.9, 89.9);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, rotation.yaw.to_radians(), rotation.pitch.to_radians(), 0.0);
        }

        // Handle Camera Movement
        let direction = transform.forward() * (keyboard_input.pressed(KeyCode::W) as i32 as f32 - keyboard_input.pressed(KeyCode::S) as i32 as f32)
                      + transform.right() * (keyboard_input.pressed(KeyCode::D) as i32 as f32 - keyboard_input.pressed(KeyCode::A) as i32 as f32)
                      + Vec3::Y * keyboard_input.pressed(KeyCode::Space) as i32 as f32;

        if direction.length_squared() > 0.0 {
            transform.translation += time.delta_seconds() * ((if keyboard_input.pressed(KeyCode::ShiftLeft) { PLAYER_SPRINT } else { PLAYER_SPEED }) * direction.normalize());
        }
    }
    // Selection of current voxel type
    vox_scroll_selection(mouse_wheel_events, &mut voxel_selector);
}

pub fn voxel_interaction_system(
    mouse_input: Res<Input<MouseButton>>,
    voxel_assets: Res<VoxelAssets>,
    voxel_selector: ResMut<VoxelSelector>,
    mut commands: Commands,
    mut voxel_world: ResMut<VoxelWorld>,
    voxel_state: ResMut<VoxelState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if voxel_state.in_range{
        let is_control = keyboard_input.pressed(KeyCode::ControlLeft);
        if is_control && mouse_input.just_pressed(MouseButton::Left){
            if let Some(voxel) = voxel_world.get_voxel_mut(voxel_state.position){
                if voxel.voxel_type == VoxelType::Switch{
                    voxel.is_on = !voxel.is_on;
                }
            }
        } else if mouse_input.just_pressed(MouseButton::Left) {
            vox_place(&mut commands, voxel_state.adjacent, &voxel_assets, &mut voxel_world, &voxel_selector)
        } else if mouse_input.just_pressed(MouseButton::Right) {
            voxel_world.remove_voxel(&mut commands, &voxel_state.position)
        }  
    }
}