use super::voxel_structure::*;


use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup_voxel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create materials for tiles and wires
    let tile_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into()); // Gray for tiles
    let wire_material = materials.add(Color::rgb(1.0, 0.0, 0.0).into()); // Red for wires

    // Initialize and populate the voxel world
    let mut voxel_world = VoxelWorld::new();
    // Example: Add some voxels to the world
    voxel_world.set_voxel(IVec3::new(5, 5, 5), Voxel { voxel_type: VoxelType::Tile, is_on: false });
    voxel_world.set_voxel(IVec3::new(5, 5, 4), Voxel { voxel_type: VoxelType::Wire, is_on: false });

    // Iterate over the voxel world and spawn entities
    for (position, voxel) in voxel_world.voxels.iter() {
        let material = match voxel.voxel_type {
            VoxelType::Tile => tile_material.clone(),
            VoxelType::Wire => wire_material.clone(),
            VoxelType::Air => continue, // Skip rendering air voxels
        };

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), // Assuming each voxel is a 1x1x1 cube
            material,
            transform: Transform::from_translation(position.as_vec3()), // Convert IVec3 to Vec3 for translation
            ..default()
        });
    }
    // ... rest of your setup code ...
}
