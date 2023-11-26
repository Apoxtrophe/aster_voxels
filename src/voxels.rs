use super::voxel_structure::*;


use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup_voxel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create materials for tiles and wires
    let tile_material = materials.add(Color::rgb(0.5, 1.0, 0.5).into()); // Gray for tiles
    let wire_material = materials.add(Color::rgb(1.0, 0.0, 0.0).into()); // Red for wires

    // Initialize the voxel world
    let mut voxel_world = VoxelWorld::new();

    // Example: Add some voxels to the world
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 })); // Assuming each voxel is a 1x1x1 cube

    // Add a Tile voxel
    voxel_world.set_voxel(
        &mut commands,
        IVec3::new(5, 5, 5),
        Voxel { voxel_type: VoxelType::Tile, is_on: false },
        cube_mesh.clone(),
        tile_material.clone(),
    );

    // Add a Wire voxel
    voxel_world.set_voxel(
        &mut commands,
        IVec3::new(5, 5, 4),
        Voxel { voxel_type: VoxelType::Wire, is_on: false },
        cube_mesh.clone(),
        wire_material.clone(),
    );

    // ... rest of your setup code ...
}