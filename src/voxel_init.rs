use super::voxel_structure::*;
use super::voxel_assets::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup_voxel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create materials for tiles and wires
    let voxel_assets = VoxelAssets::new(&mut materials, &mut meshes);

    // Initialize the voxel world
    let mut voxel_world = VoxelWorld::new();

    // Example: Add some voxels to the world
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 })); // Assuming each voxel is a 1x1x1 cube

    // Add a Tile voxel
    voxel_world.set_voxel(
        &mut commands,
        IVec3::new(5, 5, 5),
        Voxel { voxel_type: VoxelType::Tile, is_on: false },
        voxel_assets.voxel_mesh.clone(),
        voxel_assets.tile_material.clone(),
    );
    // ... rest of your setup code ...
}