use bevy::{ecs::system::{Commands, Res, ResMut, Resource}, pbr::{StandardMaterial, PbrBundle}, asset::Assets, render::mesh::Mesh, math::IVec3, transform::components::Transform};

use crate::{v_graphics::VoxelAssets, v_structure::{TypeVoxel, StateVoxel, PositionVoxel}, v_config::{BENCHMARKING, BENCHMARK_SIZE}};



#[derive(Resource)]
pub struct OneTime; 

pub fn benchmark(
    mut commands: Commands,
    voxel_assets: Res<VoxelAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    marker: Option<Res<OneTime>>,
) {

    if marker.is_none() && BENCHMARKING {
        let mut voxel_type = TypeVoxel::Xor;
        let voxel_mesh_handle = voxel_assets.create_voxel_mesh(voxel_type, &mut meshes);
        let atlas_material = voxel_assets.atlas_material(&mut materials);
        for j in 0..BENCHMARK_SIZE{
            for i in 0..BENCHMARK_SIZE {
                let position = IVec3::new(i, 1, j);
    
                commands
                    .spawn(PbrBundle {
                        mesh: voxel_mesh_handle.clone(),  // Use the UV mapped mesh
                        material: atlas_material.clone(), // Use the atlas material
                        transform: Transform::from_translation(position.as_vec3()),
                        ..Default::default()
                    })
                    .insert(PositionVoxel(position))
                    .insert(voxel_type)
                    .insert(StateVoxel(false));
            }
        }
        commands.insert_resource(OneTime);
    }

} 
    