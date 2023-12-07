use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use super::voxel_resources::*;

fn create_material_with_color(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        reflectance: 0.5,
        metallic: 0.5,
        perceptual_roughness: 0.5,
        emissive: Color::BLACK, // Default non-emissive state
        // Add other shared properties here
        ..default()
    }
}

impl VoxelAssets {
    pub fn new(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        VoxelAssets {
            tile_material: materials.add(create_material_with_color(Color::hex("808080").unwrap())), 
            wire_material: materials.add(create_material_with_color(Color::hex("800000").unwrap())),
            out_material: materials.add(create_material_with_color(Color::hex("FF4500").unwrap())),
            not_material: materials.add(create_material_with_color(Color::hex("FFA500").unwrap())),
            and_material: materials.add(create_material_with_color(Color::hex("FF69B4").unwrap())),
            or_material: materials.add(create_material_with_color(Color::hex("00FFFF").unwrap())),
            xor_material: materials.add(create_material_with_color(Color::hex("0000FF").unwrap())),
            switch_material: materials.add(create_material_with_color(Color::hex("32CD32").unwrap())),
            voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), 
        }
    }
}

pub fn debug_one(
    voxel_state: Res<VoxelState>,
    voxel_world: Res<VoxelWorld>,
) { 
    if let Some(voxel) = voxel_world.get_voxel(voxel_state.position) {
        println!("DEBUG_ONE is on:{:?} -- Voxel Type: {:?}", voxel.is_on, voxel.voxel_type);
    }
}

pub fn debug_two(
    mut query: Query<(&Voxel)>,
) { 
    for (voxel) in query.iter_mut() {
        println!("DEBUG_TWO is on:{:?} -- Voxel Type: {:?}", voxel.is_on, voxel.voxel_type);
    }
}

use bevy::prelude::*;
use super::voxel_resources::*;

// Define a system to update Voxel components based on VoxelWorld state

pub fn update_voxel_emissiveness(
    mut query: Query<(&Voxel, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (voxel, material_handle) in query.iter_mut() {
        // Dereference the handle to get the actual Handle object
        let material_handle = material_handle.clone();

        // Get mutable reference to the material using the handle
        if let Some(material) = materials.get_mut(&material_handle) {
            material.emissive = if voxel.is_on {
                Color::WHITE // Emissive color when voxel is on
            } else {
                Color::BLACK // Non-emissive when voxel is off
            };
        }
    }
}
