use bevy::{
    ecs::{system::{Commands, ResMut, Res}, entity::Entity},
    math::IVec3,
    transform::components::Transform, pbr::{PbrBundle, StandardMaterial}, asset::Assets,
};

use bevy::ecs::component::Component;

use crate::{v_selector::VoxelSelector, v_graphics::VoxelAssets, V_selector::vox_material};
use bevy::ecs::system::Resource;
use bevy::ecs::system::Query;
use bevy::ecs::query::With;

#[derive(Component, Debug, Clone, Copy)]
pub struct PositionVoxel(pub IVec3);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum TypeVoxel {Tile, Wire, Out, Not, And, Or, Xor, Switch,}

#[derive(Component, Debug, Clone, Copy)]
pub struct StateVoxel(pub bool);

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
        materials: &mut ResMut<Assets<StandardMaterial>>, // Add this parameter
    ) {
        let voxel_type = voxel_selector.current_voxel_type();
        let material_handle = vox_material(voxel_type, voxel_assets);

        // Clone the material to create a unique instance for this voxel
        let material_instance = materials.add(materials.get(&material_handle).unwrap().clone());

        commands.spawn(PbrBundle {
            mesh: voxel_assets.voxel_mesh.clone(),
            material: material_instance, // Use the cloned material instance
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
