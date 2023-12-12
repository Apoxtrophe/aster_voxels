pub fn vox_raycast(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Camera>>,
    mut voxel_info: ResMut<VoxelInfo>,
    voxel_selector: ResMut<VoxelSelector>,
    mut voxel_world: ResMut<VoxelWorld>,
    commands: &mut Commands,
    //mut get_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
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
            /* 
            //Updates all 6 values of the voxels
            voxel_info.in_range = in_range;
            voxel_info.position = position;
            voxel_info.adjacent = adjacent;
            voxel_info.selected = Some(voxel_selector.current_voxel_type());
            if let Some((voxel_type, state_voxel)) = voxel_world.get(commands, position, get_query) {
                voxel_info.voxel_type = Some(voxel_type);
                voxel_info.is_on = Some(state_voxel.0); // Access the bool inside StateVoxel
            }    
            */
        }
    }
}