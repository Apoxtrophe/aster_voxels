use super::config;
use config::*;
use crate::voxel_structure::VoxelType;
use crate::VoxelAssets;

use bevy_mod_raycast::prelude::Raycast;
use bevy_mod_raycast::prelude::Ray3d;
use crate::VoxelWorld;
use crate::VoxelSelector;
use crate::voxel_structure::Voxel;
use bevy::input::mouse::MouseWheel;

use bevy::prelude::*;


pub fn vox_raycast(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>,
) -> (bool, IVec3, IVec3) {
    if let Ok(camera_transform ) = query.get_single() {
        let ray = Ray3d::new(camera_transform.translation, camera_transform.forward());

        if let Some((_, intersection_data)) = raycast.cast_ray(ray, &default()).iter().next() {
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
            return (valid, position, adjacent);
        }
    }
    
    (false, IVec3::ZERO, IVec3::ZERO)
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

pub fn vox_delete(
    commands: &mut Commands,
    voxel_world: &mut ResMut<VoxelWorld>,
    position: IVec3,
) {
    voxel_world.remove_voxel(commands, &position)
}

pub fn vox_get<'a>(
    voxel_world: &'a mut ResMut<VoxelWorld>,
    position: IVec3,
) -> Option<&'a Voxel> {
    voxel_world.get_voxel(position)
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