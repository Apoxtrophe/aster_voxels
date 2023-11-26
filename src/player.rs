use super::config;
use config::*;
use bevy::input::mouse::MouseMotion;
use crate::voxel_structure::Voxel;
use crate::voxel_structure::VoxelType;

use crate::voxel_structure::VoxelWorld;
use crate::voxel_assets::VoxelAssets;

use bevy_mod_raycast::prelude::Raycast;
use bevy_mod_raycast::prelude::Ray3d;


use bevy::prelude::*;
use bevy_atmosphere::prelude::*;


pub fn create_player(mut commands: Commands) {
    
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })  
    .insert(CameraRotation { pitch: 0.0, yaw: 0.0 })
    .insert(AtmosphereCamera::default());
}


pub fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, _camera) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let camera_forward = transform.forward();
        let camera_right = transform.right();

        if keyboard_input.pressed(KeyCode::W) {
            direction += camera_forward;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction -= camera_forward;
        }

        // Left and right movement
        if keyboard_input.pressed(KeyCode::A) {
            direction -= camera_right;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += camera_right;
        }

        // Upward movement
        if keyboard_input.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }

        // Normalize the direction to ensure consistent movement speed
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }
        let speed:f32;
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed = PLAYER_SPRINT;
        } else {
            speed = PLAYER_SPEED;
        }
        transform.translation += time.delta_seconds() * speed * direction;
    }
}

#[derive(Component)]
pub struct CameraRotation {
    pitch: f32,
    yaw: f32,
}

pub fn camera_rotation_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraRotation), With<Camera>>,
) {
    for (mut transform, mut rotation) in query.iter_mut() {
        for event in mouse_motion_events.read() {
            rotation.yaw -= event.delta.x * MOUSE_SENSITIVITY; // Adjust sensitivity as needed
            rotation.pitch += event.delta.y * -MOUSE_SENSITIVITY; // Adjust sensitivity as needed

            rotation.pitch = rotation.pitch.clamp(-89.9, 89.9); // Prevent flipping

            let yaw_radians = rotation.yaw.to_radians();
            let pitch_radians = rotation.pitch.to_radians();

            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians) * Quat::from_axis_angle(Vec3::X, pitch_radians);
        }
    }
}

pub fn raycast(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>, // Query to get the camera's transform
) {
    if let Ok(camera_transform) = query.get_single() {
        let camera_position = camera_transform.translation;
        let camera_forward = camera_transform.forward();

        // Cast the ray from the camera's position in the direction it's facing
        let ray = Ray3d::new(camera_position, camera_forward);

        // Cast the ray
        let intersect = raycast.debug_cast_ray(ray, &default(), &mut gizmos);
        println!("{:?}", intersect)
    }
}