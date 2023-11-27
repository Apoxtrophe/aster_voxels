use super::config;
use config::*;
use bevy::input::mouse::MouseMotion;
use crate::voxel_structure::Voxel;
use crate::voxel_structure::VoxelType;

use crate::voxel_structure::VoxelWorld;


use bevy_mod_raycast::prelude::Raycast;
use bevy_mod_raycast::prelude::Ray3d;


use bevy::prelude::*;
use bevy_atmosphere::prelude::*;


use super::voxel_assets::*;




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

pub fn voxel_place_system(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    query: Query<&Transform, With<Camera>>,
    raycast: Raycast, 
    gizmos: Gizmos, 
    mut voxel_world: ResMut<VoxelWorld>,
    voxel_assets: Res<VoxelAssets>,
) {
    let trinity = raycaster(raycast, gizmos, query);
    let (valid, position, adjacent) = trinity;

    if mouse_input.just_pressed(MouseButton::Left) && valid {
        voxel_world.set_voxel(
            &mut commands,
            adjacent,
            Voxel { voxel_type: VoxelType::Wire, is_on: false },
            voxel_assets.voxel_mesh.clone(),
            voxel_assets.wire_material.clone(),
        );
    }
    if mouse_input.just_pressed(MouseButton::Right) && valid {
        voxel_world.remove_voxel(&mut commands, &position)
    }
}



pub fn raycaster(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>, // Query to get the camera's transform
)-> (bool, IVec3, IVec3) {
    if let Ok(camera_transform) = query.get_single() {
        let camera_position = camera_transform.translation;
        let camera_forward = camera_transform.forward();

        let ray = Ray3d::new(camera_position, camera_forward);

        let intersect = raycast.debug_cast_ray(ray, &default(), &mut gizmos); // Modify this line to include max_distance

        for (_, intersection_data) in intersect {
            let distance = intersection_data.distance();
            let normal = intersection_data.normal();
            let mut vertex1: Vec3 = intersection_data.triangle().unwrap().v0.into();
            let mut vertex2: Vec3  = intersection_data.triangle().unwrap().v1.into();
            let mut vertex3: Vec3  = intersection_data.triangle().unwrap().v2.into();
            vertex1 = vertex1 - normal*0.5;
            vertex2 = vertex2 - normal*0.5;
            vertex3 = vertex3 - normal*0.5;
            let position:IVec3 = ((vertex1 + vertex2 + vertex3) * 0.33333).round().as_ivec3();
            let valid: bool = distance < INTERACTION_DISTANCE;

            let adjacent = position + normal.as_ivec3();

            return (valid, position, adjacent);
            //return (valid, position, adjacent);
        }
    }
    (false, IVec3::ZERO, IVec3::ZERO)
}