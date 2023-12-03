// Bevy-related imports
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_mod_raycast::prelude::Raycast;
use bevy_atmosphere::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

// Voxel assets and configuration
use super::voxel_assets::*;
use super::config::*;
use super::voxel_lib::*;
use super::voxel_structure::*;

pub fn create_player(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })  
    .insert(CameraRotation { pitch: 0.0, yaw: 0.0 })
    .insert(AtmosphereCamera::default());
}

#[derive(Component)]
pub struct CameraRotation {
    pitch: f32,
    yaw: f32,
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
    raycast: Raycast, 
    gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>,
    mouse_input: Res<Input<MouseButton>>,
    voxel_assets: Res<VoxelAssets>,
    voxel_selector: ResMut<VoxelSelector>,
    mut commands: Commands,
    mut voxel_world: ResMut<VoxelWorld>,
    mut voxel_look: ResMut<VoxelLookedAt>,
) {
        // Voxel Interaction
    let (valid, position, adjacent) = vox_raycast(raycast, gizmos, query);

    if valid {
        let voxel = vox_get(&mut voxel_world, position);
        if let Some(voxel_info) = voxel {
            voxel_look.update(position, voxel_info.voxel_type);
        }
        if mouse_input.just_pressed(MouseButton::Left) {
            vox_place(&mut commands, adjacent, &voxel_assets, &mut voxel_world, &voxel_selector)
        } else if mouse_input.just_pressed(MouseButton::Right) {
            vox_delete(&mut commands, &mut voxel_world, position)
        }  
    } else {
        voxel_look.clear();
    }
}

#[derive(Resource, Debug)]
pub struct VoxelLookedAt {
    pub position: Option<IVec3>,
    pub voxel_type: Option<VoxelType>,
}

impl VoxelLookedAt {
    pub fn update(&mut self, position: IVec3, voxel_type: VoxelType) {
        self.position = Some(position);
        self.voxel_type = Some(voxel_type);
    }

    pub fn clear(&mut self) {
        self.position = None;
        self.voxel_type = None;
    }
}

pub fn ui_DEBUG(
    mut contexts: EguiContexts,
    voxel_look: Res<VoxelLookedAt>,
    voxel_selector: ResMut<VoxelSelector>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        let voxel_type = voxel_selector.current_voxel_type();
        ui.label(format!("Selected Voxel: {:?}", voxel_type));
        match (voxel_look.position, voxel_look.voxel_type) {
            (Some(position), Some(voxel_type)) => {
                ui.label(format!("Position: {:?}", position));
                ui.label(format!("Voxel Type: {:?}", voxel_type));
            }
            _ => {
                ui.label("No voxel currently looked at");
            }
        }
    });
}