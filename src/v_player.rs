// Bevy-related imports
use bevy::prelude::*;

use crate::v_components::CameraRotation;
use crate::v_components::PositionVoxel;
use crate::v_components::StateVoxel;
use crate::v_components::TypeVoxel;
use crate::v_graphics::VoxelAssets;
use crate::v_selector::vox_scroll_selection;
use crate::v_selector::VoxelSelector;
use crate::v_structure::Voxel;
use bevy::input::mouse::{MouseMotion, MouseWheel};
// Voxel assets and configuration

use super::v_config::*;
use super::v_lib::*;

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
            rotation.pitch =
                (rotation.pitch - event.delta.y * MOUSE_SENSITIVITY).clamp(-89.9, 89.9);
            transform.rotation = Quat::from_euler(
                EulerRot::YXZ,
                rotation.yaw.to_radians(),
                rotation.pitch.to_radians(),
                0.0,
            );
        }

        // Handle Camera Movement
        let direction = transform.forward()
            * (keyboard_input.pressed(KeyCode::W) as i32 as f32
                - keyboard_input.pressed(KeyCode::S) as i32 as f32)
            + transform.right()
                * (keyboard_input.pressed(KeyCode::D) as i32 as f32
                    - keyboard_input.pressed(KeyCode::A) as i32 as f32)
            + Vec3::Y * keyboard_input.pressed(KeyCode::Space) as i32 as f32;

        if direction.length_squared() > 0.0 {
            transform.translation += time.delta_seconds()
                * ((if keyboard_input.pressed(KeyCode::ShiftLeft) {
                    PLAYER_SPRINT
                } else {
                    PLAYER_SPEED
                }) * direction.normalize());
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
    mut voxel: ResMut<Voxel>,
    voxel_info: Res<VoxelInfo>,
    keyboard_input: Res<Input<KeyCode>>,
    remove_query: Query<(Entity, &PositionVoxel)>,
    state_query: Query<(Entity, &PositionVoxel, &mut StateVoxel)>,
    materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    //Placing, removing, and altering state on mouse click
    if voxel_info.in_range {
        if mouse_input.just_pressed(MouseButton::Left) {
            if keyboard_input.pressed(KeyCode::ControlLeft) {
                if let Some(state) = voxel_info.is_on {
                    if let Some(voxel_type) = voxel_info.voxel_type {
                        if voxel_type == TypeVoxel::Switch {
                            voxel.set_state(
                                &mut commands,
                                voxel_info.position,
                                !state,
                                state_query,
                            );
                        }
                    }
                }
            } else {
                voxel.place(
                    &mut commands,
                    voxel_info.adjacent,
                    &voxel_selector,
                    &voxel_assets,
                    materials,
                    meshes,
                )
            }
        } else if mouse_input.just_pressed(MouseButton::Right) {
            voxel.remove(&mut commands, voxel_info.position, remove_query);
        }
    }
}
