use bevy::prelude::*;
use bevy::render::mesh::Mesh;

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