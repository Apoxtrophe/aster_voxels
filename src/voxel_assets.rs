use bevy::prelude::*;
use bevy::render::mesh::Mesh;

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

impl VoxelAssets {
    pub fn new(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        VoxelAssets {
            tile_material: materials.add(Color::hex("808080").unwrap().into()), 
            wire_material: materials.add(Color::hex("800000").unwrap().into()),
            out_material: materials.add(Color::hex("FFFF00").unwrap().into()),
            not_material: materials.add(Color::hex("FFA500").unwrap().into()),
            and_material: materials.add(Color::hex("FF0000").unwrap().into()),
            or_material: materials.add(Color::hex("00FFFF").unwrap().into()),
            xor_material: materials.add(Color::hex("0000FF").unwrap().into()),
            switch_material: materials.add(Color::hex("00FF00").unwrap().into()),
            voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), 
        }
    }
}

