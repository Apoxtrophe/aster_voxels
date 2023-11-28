use bevy::prelude::*;
use bevy::render::mesh::Mesh;

#[derive(Resource)]
pub struct VoxelAssets {
    pub tile_material: Handle<StandardMaterial>,
    pub wire_material: Handle<StandardMaterial>,
    pub voxel_mesh: Handle<Mesh>,
    // Add more materials or meshes as needed
}

impl VoxelAssets {
    pub fn new(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        VoxelAssets {
            tile_material: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
            wire_material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), // Assuming a standard cube mesh for voxels
            // Initialize more materials or meshes here
        }
    }
}
