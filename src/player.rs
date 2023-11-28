use super::config;
use bevy::ecs::entity;
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


#[derive(Component)]
pub struct CameraRotation {
    pitch: f32,
    yaw: f32,
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

        direction += camera_forward * (keyboard_input.pressed(KeyCode::W) as i32 - keyboard_input.pressed(KeyCode::S) as i32) as f32;
        direction += camera_right * (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32) as f32;
        direction += Vec3::Y * keyboard_input.pressed(KeyCode::Space) as i32 as f32;

        if direction.length_squared() > 0.0 {
            transform.translation += time.delta_seconds() * (if keyboard_input.pressed(KeyCode::ShiftLeft) { PLAYER_SPRINT } else { PLAYER_SPEED }) * direction.normalize();
        }
    }
}

pub fn camera_rotation_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraRotation), With<Camera>>,
) {
    for event in mouse_motion_events.read() {
        for (mut transform, mut rotation) in query.iter_mut() {
            rotation.yaw -= event.delta.x * MOUSE_SENSITIVITY;
            rotation.pitch = (rotation.pitch - event.delta.y * MOUSE_SENSITIVITY).clamp(-89.9, 89.9);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, rotation.yaw.to_radians()) * Quat::from_axis_angle(Vec3::X, rotation.pitch.to_radians());
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
    let (valid, position, adjacent) = raycaster(raycast, gizmos, query);

    if valid {
        if mouse_input.just_pressed(MouseButton::Left) {
            voxel_world.set_voxel(
                &mut commands,
                adjacent,
                Voxel { voxel_type: VoxelType::Wire, is_on: false },
                voxel_assets.voxel_mesh.clone(),
                voxel_assets.wire_material.clone(),
            );
        } else if mouse_input.just_pressed(MouseButton::Right) {
            voxel_world.remove_voxel(&mut commands, &position);
        }
    }
}

pub fn raycaster(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>,
) -> (bool, IVec3, IVec3) {
    if let Ok(camera_transform) = query.get_single() {
        let ray = Ray3d::new(camera_transform.translation, camera_transform.forward());

        if let Some((entity, intersection_data)) = raycast.cast_ray(ray, &default()).iter().next() {
            let distance = intersection_data.distance();
            let normal = intersection_data.normal().round();
            let triangle = intersection_data.triangle().unwrap();
            let position = ((Vec3::from(triangle.v0) + Vec3::from(triangle.v1) + Vec3::from(triangle.v2)) / 3.0 - normal * 0.5).round().as_ivec3();
            let valid = distance < INTERACTION_DISTANCE;
            let adjacent = position + normal.as_ivec3();
            if valid {
                gizmos.cuboid(
                    Transform::from_translation(position.as_vec3()).with_scale(Vec3::splat(1.)),
                    Color::BLACK,
                );
            }
            println!("Valid:{}, Position:{} Adjacent:{} Entity:{:?} Normal{}", valid, position, adjacent, entity, normal);
            return (valid, position, adjacent);
        }
    }
    
    (false, IVec3::ZERO, IVec3::ZERO)
}