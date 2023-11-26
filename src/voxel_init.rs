use super::voxel_structure::*;
use super::voxel_assets::*;
use super::config::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn voxel_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create materials for tiles and wires
    let voxel_assets = VoxelAssets::new(&mut materials, &mut meshes);

    // Initialize the voxel world
    let mut voxel_world = VoxelWorld::new();

    // Add a Tile voxel
    voxel_world.set_voxel(
        &mut commands,
        IVec3::new(1, 2, 3),
        Voxel { voxel_type: VoxelType::Tile, is_on: false },
        voxel_assets.voxel_mesh.clone(),
        voxel_assets.tile_material.clone(),
    );
}