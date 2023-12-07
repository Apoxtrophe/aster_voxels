use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use super::voxel_resources::*;

fn create_material_with_color(
    color: Color, 
    is_emissive: bool,
) -> StandardMaterial {
    
    let mut material = StandardMaterial {
        base_color: color,
        reflectance: 0.5,
        metallic: 0.5,
        perceptual_roughness: 0.5,
        // Add other shared properties here
        ..default()
    };
    if is_emissive {
        material.emissive = color;
    }
    material
}

impl VoxelAssets {
    pub fn new(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        VoxelAssets {
            tile_material: materials.add(create_material_with_color(Color::hex("808080").unwrap(), false)), 
            wire_material: materials.add(create_material_with_color(Color::hex("800000").unwrap(), false)),
            out_material: materials.add(create_material_with_color(Color::hex("FF4500").unwrap(),false)),
            not_material: materials.add(create_material_with_color(Color::hex("FFA500").unwrap(), false)),
            and_material: materials.add(create_material_with_color(Color::hex("FF69B4").unwrap(), false)),
            or_material: materials.add(create_material_with_color(Color::hex("00FFFF").unwrap(), false)),
            xor_material: materials.add(create_material_with_color(Color::hex("0000FF").unwrap(), false)),
            switch_material: materials.add(create_material_with_color(Color::hex("32CD32").unwrap(), false)),
            voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), 
        }
    }
}

