use bevy::{prelude::*, input::mouse::MouseWheel, transform::commands};
use bevy_mod_raycast::{immediate::Raycast, primitives::Ray3d};

use crate::{v_structure::{TypeVoxel, Voxel, PositionVoxel, StateVoxel}, v_selector::VoxelSelector, v_graphics::VoxelAssets};
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
    mut gizmos: Gizmos, 
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

            //Neat lil gizmo
            if in_range{
                gizmos.cuboid(
                    Transform::from_translation(position.as_vec3()).with_scale(Vec3::splat(1.02)),
                    Color::BLACK,
                );
            }
            return Some((position, adjacent, in_range));
        }
    }
    None
}

pub fn update_info(
    raycast: Raycast,
    query: Query<&Transform, With<Camera>>,
    mut voxel_info: ResMut<VoxelInfo>,
    voxel: ResMut<Voxel>,
    get_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    voxel_selector: ResMut<VoxelSelector>,
    gizmos: Gizmos, 
) {
    voxel_info.selected = Some(voxel_selector.current_voxel_type());
    if let Some((position, adjacent, in_range)) = raycasting(raycast, query, gizmos){
        voxel_info.position = position;
        voxel_info.adjacent = adjacent;
        voxel_info.in_range = in_range;
        if let Some((voxel_type, voxel_state)) = voxel.get(position, get_query){
            voxel_info.is_on = Some(voxel_state.0);
            voxel_info.voxel_type = Some(voxel_type);
        } else {
            // Handle the case where no voxel is found at the position
            voxel_info.is_on = None;
            voxel_info.voxel_type = None;
        }
    }

}