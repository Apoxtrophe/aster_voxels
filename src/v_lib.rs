use bevy::prelude::*;
use bevy_mod_raycast::immediate::Raycast;
use bevy_math::{Ray3d, Vec3A};
use super::v_config::*;
use crate::{
    v_components::{PositionVoxel, StateVoxel, TypeVoxel},
    v_structure::Voxel,
};

#[derive(Resource)]
pub struct VoxelInfo {
    pub position: IVec3,
    pub adjacent: IVec3,
    pub voxel_type: Option<TypeVoxel>,
    pub in_range: bool,
    pub is_on: Option<bool>,
}

impl VoxelInfo {
    pub fn new() -> Self {
        Self {
            position: IVec3::ZERO,
            adjacent: IVec3::ZERO,
            in_range: false,
            voxel_type: None,
            is_on: None,
        }
    }
}

pub enum RaycastingError {
    NoIntersection,
}

pub fn raycasting(
    mut raycast: Raycast,
    query: Query<&Transform, With<Camera>>,
    mut gizmos: Gizmos,
) -> Result<(IVec3, IVec3, bool), RaycastingError> {
    if let Ok(camera_transform) = query.get_single() {
        let ray = Ray3d::new(
            camera_transform.translation,
            camera_transform.forward().into(),
        );
        if let Some((_, intersection_data)) = raycast.cast_ray(ray, &default()).iter().next() {
            let distance = intersection_data.distance();
            let normal = intersection_data.normal().round();
            let triangle = intersection_data.triangle().unwrap();
            let position = calculate_voxel_position(triangle, normal);
            let is_in_range = distance < PLAYER_INTERACTION_MAX;
            let adjacent_position = position + normal.as_ivec3();

            if is_in_range {
                draw_voxel_gizmo(&mut gizmos, position);
            }

            return Ok((position, adjacent_position, is_in_range));
        }
    }
    Err(RaycastingError::NoIntersection)
}

fn calculate_voxel_position(triangle: [Vec3A; 3], normal: Vec3) -> IVec3 {
    ((Vec3::from(triangle[0]) + Vec3::from(triangle[1]) + Vec3::from(triangle[2])) / 3.0
        - normal * 0.5)
        .round()
        .as_ivec3()
}

fn draw_voxel_gizmo(gizmos: &mut Gizmos, position: IVec3) {
    gizmos.cuboid(
        Transform::from_translation(position.as_vec3()).with_scale(Vec3::splat(1.02)),
        Color::BLACK,
    );
}

pub fn update_info(
    raycast: Raycast,
    query: Query<&Transform, With<Camera>>,
    mut voxel_info: ResMut<VoxelInfo>,
    voxel: ResMut<Voxel>,
    get_query: Query<(&PositionVoxel, &TypeVoxel, &StateVoxel)>,
    gizmos: Gizmos,
) {
    match raycasting(raycast, query, gizmos) {
        Ok((position, adjacent_position, is_in_range)) => {
            voxel_info.position = position;
            voxel_info.adjacent = adjacent_position;
            voxel_info.in_range = is_in_range;

            if let Some((voxel_type, voxel_state)) = voxel.get(position, get_query) {
                voxel_info.is_on = Some(voxel_state.0);
                voxel_info.voxel_type = Some(voxel_type);
            } else {
                reset_voxel_info(&mut voxel_info);
            }
        }
        Err(RaycastingError::NoIntersection) => {
            reset_voxel_info(&mut voxel_info);
        }
    }
}

fn reset_voxel_info(voxel_info: &mut VoxelInfo) {
    voxel_info.is_on = None;
    voxel_info.voxel_type = None;
}
