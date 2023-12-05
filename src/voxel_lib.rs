use super::config;
use config::*;
use crate::VoxelAssets;

use bevy_mod_raycast::prelude::Raycast;
use bevy_mod_raycast::prelude::Ray3d;
use crate::VoxelWorld;
use crate::VoxelSelector;
use bevy::input::mouse::MouseWheel;
use super::voxel_resources::*;

use bevy::prelude::*;

impl VoxelState {
    pub fn new() -> Self {
        VoxelState {
            position: IVec3::ZERO,
            adjacent: IVec3::ZERO,
            voxel_type: None, 
            in_range: false,
            is_on: None,
            selected: None,
        }
    }
}

pub fn vox_raycast(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>,
    mut voxel_state: ResMut<VoxelState>,
    voxel_selector: ResMut<VoxelSelector>,
    voxel_world: Res<VoxelWorld>,
) {
    if let Ok(camera_transform ) = query.get_single() {
        let ray = Ray3d::new(camera_transform.translation, camera_transform.forward());

        if let Some((_, intersection_data)) = raycast.cast_ray(ray, &default()).iter().next() {
            let distance = intersection_data.distance();
            let normal = intersection_data.normal().round();
            let triangle = intersection_data.triangle().unwrap();
            let position = ((Vec3::from(triangle.v0) + Vec3::from(triangle.v1) + Vec3::from(triangle.v2)) / 3.0 - normal * 0.5).round().as_ivec3();
            let in_range = distance < INTERACTION_DISTANCE;
            let adjacent = position + normal.as_ivec3();
            if in_range {
                gizmos.cuboid(
                    Transform::from_translation(position.as_vec3()).with_scale(Vec3::splat(1.)),
                    Color::BLACK,
                );
            }
            println!("valid: {} position: {} adjacent: {}", in_range, position, adjacent);
            //return (valid, position, adjacent); 
            voxel_state.in_range = in_range;
            voxel_state.position = position;
            voxel_state.adjacent = adjacent;
            voxel_state.selected = Some(voxel_selector.current_voxel_type());
            if let Some(voxel) = voxel_world.get_voxel(position) {
                voxel_state.voxel_type = Some(voxel.voxel_type);
                voxel_state.is_on = Some(voxel.is_on);
            } else {
                voxel_state.voxel_type = None; // Replace VoxelType::Default with your default type
                voxel_state.is_on = None;
            }
       
        }
    }
}

pub fn vox_material(voxel_type: VoxelType, voxel_assets: &Res<VoxelAssets>) -> Handle<StandardMaterial> {
    match voxel_type {
        VoxelType::Tile => voxel_assets.tile_material.clone(),
        VoxelType::Wire => voxel_assets.wire_material.clone(),
        VoxelType::Out => voxel_assets.out_material.clone(),
        VoxelType::Not => voxel_assets.not_material.clone(),
        VoxelType::And => voxel_assets.and_material.clone(),
        VoxelType::Or => voxel_assets.or_material.clone(),
        VoxelType::Xor => voxel_assets.xor_material.clone(),
        VoxelType::Switch => voxel_assets.switch_material.clone(),
    }
}

pub fn vox_scroll_selection(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    voxel_selector: &mut ResMut<VoxelSelector>,
) {
    for event in mouse_wheel_events.read() {
        if event.y > 0.0 {
            voxel_selector.next();
        } else if event.y <0.0 {
            voxel_selector.previous();
        }
    }
}

pub fn vox_place(
    commands: &mut Commands,
    adjacent: IVec3,
    voxel_assets: &Res<VoxelAssets>,
    voxel_world: &mut ResMut<VoxelWorld>,
    voxel_selector: &ResMut<VoxelSelector>,
) {
    let current_voxel_type = voxel_selector.current_voxel_type();
    let material = vox_material(current_voxel_type, voxel_assets);
    voxel_world.set_voxel(
        commands,
        adjacent,
        Voxel { voxel_type: current_voxel_type, is_on: false },
        voxel_assets.voxel_mesh.clone(),
        material,
    );
}