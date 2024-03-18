use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_asset::{RenderAssetUsages};
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use crate::a_loading::TextureHandles;
use crate::v_components::{TypeVoxel, StateVoxel};
use crate::v_config::{VOXEL_PERCIEVED_ROUGHNESS, VOXEL_METALLIC, VOXEL_REFLECTANCE};
use crate::v_graphics_helper::{calculate_indices, calculate_normals, calculate_positions, calculate_uv_coordinates};

#[derive(Resource)]
pub struct VoxelAssets {
    pub voxel_mesh: Handle<Mesh>,
    texture_atlas: Handle<Image>,
}
impl VoxelAssets {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        texture_handles: &Res<TextureHandles>,
    ) -> Self {
        let texture_handle = texture_handles.image_handles.get(0).expect("Texture handle not found");
        let voxel_assets = VoxelAssets {
            texture_atlas: texture_handle.clone(),
            voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        };
        voxel_assets
    }
    pub fn atlas_material(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color_texture: Some(self.texture_atlas.clone()),
            perceptual_roughness:VOXEL_PERCIEVED_ROUGHNESS,
            metallic: VOXEL_METALLIC,
            emissive: Color::BLACK,
            reflectance: VOXEL_REFLECTANCE,
            ..default()
        })
    }
    pub fn create_voxel_mesh(
        &self,
        voxel_type: TypeVoxel,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Handle<Mesh> {
        let uv_coordinates = match voxel_type {
            TypeVoxel::Tile => calculate_uv_coordinates(0),
            TypeVoxel::Wire => calculate_uv_coordinates(1),
            TypeVoxel::Out => calculate_uv_coordinates(2),
            TypeVoxel::Switch => calculate_uv_coordinates(3),
            TypeVoxel::And => calculate_uv_coordinates(4),
            TypeVoxel::Or => calculate_uv_coordinates(5),
            TypeVoxel::Xor => calculate_uv_coordinates(6),
            TypeVoxel::Not => calculate_uv_coordinates(7),
            TypeVoxel::DFlipFlop => calculate_uv_coordinates(8),
        };
        let positions = calculate_positions();
        let normals = calculate_normals();
        let indices = calculate_indices();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv_coordinates);
           
        mesh.insert_indices(Some(Indices::U32(indices)).unwrap());
        let mesh_handle = meshes.add(mesh);
        mesh_handle
    }
}

pub fn update_voxel_emissive(
    time: Res<Time>,
    mut query: Query<(&StateVoxel, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (state, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            if state.0 {
                let pulse_frequency = 8.0; // Adjust this value to change the pulse frequency
                let t = (time.elapsed_seconds() * pulse_frequency).sin() * 0.5 + 4.5;
                material.emissive = Color::rgb(t*0.5, t, t*0.5);
            } else {
                material.emissive = Color::BLACK;
            }
        }
    }
}