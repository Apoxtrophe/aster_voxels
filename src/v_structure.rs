use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Res, ResMut},
    },
    math::IVec3,
    pbr::{PbrBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
};
use crate::{v_graphics::VoxelAssets, v_selector::VoxelSelector, v_components::{PositionVoxel, TypeVoxel, StateVoxel}};
use bevy::ecs::system::Query;
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct Voxel;

impl Voxel {
    pub fn new() -> Self {
        Voxel
    }

    pub fn get(
        &self,
        position: IVec3,
        get_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    ) -> Option<(TypeVoxel, StateVoxel)> {
        for (_, voxel_position, voxel_type, voxel_state) in get_query.iter() {
            if voxel_position.0 == position {
                return Some((*voxel_type, *voxel_state));
            }
        }
        None
    }

    pub fn set_state(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        new_state: bool,
        mut state_query: Query<(Entity, &PositionVoxel, &mut StateVoxel)>,
    ) {
        for (entity, voxel_position, mut voxel_state) in state_query.iter_mut() {
            if voxel_position.0 == position {
                // Update the state of the voxel
                *voxel_state = StateVoxel(new_state);
                // Reflect this change in the ECS by using commands
                commands.entity(entity).insert(StateVoxel(new_state));
                // No return needed, as we are just setting the state
            }
        }
    }

    pub fn place(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        voxel_selector: &ResMut<VoxelSelector>,
        voxel_assets: &Res<VoxelAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        let voxel_type = voxel_selector.current_voxel_type();
        let voxel_mesh_handle = voxel_assets.create_voxel_mesh(voxel_type, &mut meshes);

        // Use the atlas material
        let atlas_material = voxel_assets.atlas_material(&mut materials);

        commands
            .spawn(PbrBundle {
                mesh: voxel_mesh_handle,  // Use the UV mapped mesh
                material: atlas_material, // Use the atlas material
                transform: Transform::from_translation(position.as_vec3()),
                ..Default::default()
            })
            .insert(PositionVoxel(position))
            .insert(voxel_type)
            .insert(StateVoxel(false));
    }

    pub fn remove(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        query: Query<(Entity, &PositionVoxel)>, // Include the PositionVoxel component in the query
    ) {
        for (entity, voxel_position) in query.iter() {
            // Check if the voxel's position matches the target position
            if voxel_position.0 == position {
                // Despawn the entity if the positions match
                commands.entity(entity).despawn();
            }
        }
    }
}
