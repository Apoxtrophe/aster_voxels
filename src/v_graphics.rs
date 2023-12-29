use bevy::prelude::*;
use bevy::render::mesh::Mesh;

use crate::v_structure::StateVoxel;

fn create_material_with_color(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        reflectance: 0.0,
        metallic: 0.99,
        perceptual_roughness: 0.99,
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
            wire_material: materials.add(create_material_with_color(Color::hex("36454F").unwrap())),
            out_material: materials.add(create_material_with_color(Color::hex("CC5500").unwrap())),
            not_material: materials.add(create_material_with_color(Color::hex("654321").unwrap())),
            and_material: materials.add(create_material_with_color(Color::hex("800020").unwrap())),
            or_material: materials.add(create_material_with_color(Color::hex("008080").unwrap())),
            xor_material: materials.add(create_material_with_color(Color::hex("003366").unwrap())),
            switch_material: materials.add(create_material_with_color(Color::hex("0B6623").unwrap())),
            voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), 
        }
    }
}

#[derive(Resource)]
pub struct VoxelAssets {
    pub tile_material: Handle<StandardMaterial>,
    pub wire_material: Handle<StandardMaterial>,
    pub out_material: Handle<StandardMaterial>,
    pub not_material: Handle<StandardMaterial>,
    pub and_material: Handle<StandardMaterial>,
    pub or_material: Handle<StandardMaterial>,
    pub xor_material: Handle<StandardMaterial>,
    pub switch_material: Handle<StandardMaterial>,
    pub voxel_mesh: Handle<Mesh>,
    // Add more materials or meshes as needed
}

pub fn update_voxel_emissive(
    mut query: Query<(&StateVoxel, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (state, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            material.emissive = if state.0 {
                material.base_color // Use the base color as the emissive color
            } else {
                Color::BLACK // Non-emissive state
            };
        }
    }
}