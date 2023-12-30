use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use crate::v_structure::{StateVoxel, TypeVoxel};
use bevy::asset::AssetServer;

#[derive(Resource)]
pub struct VoxelAssets {
    pub texture_atlas: Handle<Image>, // Handle to the texture atlas
    pub voxel_mesh: Handle<Mesh>,     // General voxel mesh
                                      // Add more materials or meshes as needed
}

impl VoxelAssets {
    pub fn new(
        asset_server: Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let texture_handle: Handle<Image> = asset_server.load("TexturePack/Textures.png");

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
            perceptual_roughness: 1.0,
            metallic: 1.0,
            emissive: Color::BLACK,
            reflectance: 0.0,
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
        };

        // Example positions for a unit cube. You'll need to adjust these based on your specific mesh.
        let positions = vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5],  // vertex with index 1
            [0.5, 0.5, 0.5],   // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ];

        // Calculate normals for each vertex
        // Normals are usually unit vectors perpendicular to the surface
        let normals = vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ];

        let mesh = Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv_coordinates)
            .with_indices(Some(Indices::U32(vec![
                0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
                4, 5, 7, 5, 6, 7, // bottom (-y)
                8, 11, 9, 9, 11, 10, // right (+x)
                12, 13, 15, 13, 14, 15, // left (-x)
                16, 19, 17, 17, 19, 18, // back (+z)
                20, 21, 23, 21, 22, 23, // forward (-z)
            ])));

        meshes.add(mesh) // Add the mesh to the asset system and return the handle
    }
}

pub fn update_voxel_emissive(
    mut query: Query<(&StateVoxel, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (state, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            material.emissive = if state.0 {
                Color::DARK_GRAY // Use the base color as the emissive color
            } else {
                Color::BLACK // Non-emissive state
            };
        }
    }
}

fn calculate_uv_coordinates(texture_index: u32) -> Vec<[f32; 2]> {
    let atlas_width = 8.0; // Total number of textures in atlas horizontally
    let texture_size = 1.0 / atlas_width;

    let left = texture_index as f32 * texture_size;
    let right = left + texture_size;
    let top = 0.0;
    let bottom = 1.0;

    // Assuming a simple cube where each face uses the same part of the texture
    vec![
        // UVs for each face of the cube
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
        // Repeat this for each of the 6 faces of the cube
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
    ]
}
