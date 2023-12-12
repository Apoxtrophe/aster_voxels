use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_mod_raycast::{immediate::Raycast, primitives::Ray3d};

use crate::{v_structure::{TypeVoxel, VoxelWorld}, v_selector::VoxelSelector, v_graphics::VoxelAssets};
use super::v_config::*;

#[derive(Resource)]
pub struct VoxelInfo {
    pub position: IVec3,
    pub adjacent: IVec3,
    pub voxel_type: Option<TypeVoxel>,
    pub in_range: bool,
    pub is_on: Option<bool>,
    pub selected: Option<TypeVoxel>,
}

impl VoxelInfo {
    pub fn new() -> Self {
        VoxelInfo {
            position: IVec3::ZERO,
            adjacent: IVec3::ZERO,
            in_range: false,
            voxel_type: None, 
            is_on: None,
            selected: None,
        }
    }
}

pub fn raycasting(
    mut raycast: Raycast,
    query: Query<&Transform, With<Camera>>,
) -> Option<(IVec3, IVec3, bool)> {
    if let Ok(camera_transform) = query.get_single() {
        let ray = Ray3d::new(camera_transform.translation, camera_transform.forward());
        if let Some((_, intersection_data)) = raycast.cast_ray(ray, &default()).iter().next() {
            let distance = intersection_data.distance();
            let normal = intersection_data.normal().round();
            let triangle = intersection_data.triangle().unwrap();
            let position = ((Vec3::from(triangle.v0) + Vec3::from(triangle.v1) + Vec3::from(triangle.v2)) / 3.0 - normal * 0.5).round().as_ivec3();
            let in_range = distance < INTERACTION_DISTANCE;
            let adjacent = position + normal.as_ivec3();
            return Some((position, adjacent, in_range));
        }
    }
    None
}

pub fn update_info(
    raycast: Raycast,
    query: Query<&Transform, With<Camera>>,
    mut voxel_info: ResMut<VoxelInfo>,
) {
    if let Some((position, adjacent, in_range)) = raycasting(raycast, query){
        voxel_info.position = position;
        voxel_info.adjacent = adjacent;
        voxel_info.in_range = in_range;
    }
}