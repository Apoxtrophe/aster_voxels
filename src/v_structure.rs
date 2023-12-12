use bevy::{
    ecs::{system::{Commands, ResMut, Res}, entity::Entity},
    math::IVec3,
    transform::components::Transform, pbr::PbrBundle,
};

use bevy::ecs::component::Component;

use crate::{v_selector::VoxelSelector, v_graphics::VoxelAssets, V_selector::vox_material};


use bevy::ecs::system::Resource;
use bevy::ecs::system::Query;
use bevy::ecs::query::With;

#[derive(Component, Debug, Clone, Copy)]
pub struct PositionVoxel(
    pub IVec3
);

#[derive(Component, Debug, Clone, Copy)]
pub enum TypeVoxel {
    Tile, Wire, Out, Not, And, Or, Xor, Switch,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct StateVoxel(
    pub bool
);


#[derive(Resource)]
pub struct VoxelWorld;

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld
    }

    pub fn get(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        mut get_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    ) -> Option<(TypeVoxel, StateVoxel)> {
        for (entity, voxel_position, voxel_type, voxel_state) in get_query.iter_mut() {
            if voxel_position.0 == position {
                return Some((*voxel_type, *voxel_state));
            }
        }
        None
    }

    pub fn get_mut(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        new_state: bool,
        mut get_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    ) -> Option<(TypeVoxel, StateVoxel)> {
        for (entity, voxel_position, voxel_type, voxel_state) in get_query.iter_mut() {
            if voxel_position.0 == position {
                commands.entity(entity).insert(StateVoxel(new_state));
                return Some((*voxel_type, *voxel_state));
            }
        }
        None
    }

    pub fn place(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        voxel_selector: &ResMut<VoxelSelector>,
        voxel_assets: &Res<VoxelAssets>,
    ) {
        // Create the entity for the new voxel
        let voxel_type = voxel_selector.current_voxel_type();
        let material = vox_material(voxel_type, voxel_assets);
        commands.spawn(PbrBundle {
            mesh: voxel_assets.voxel_mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(position.as_vec3()),
            // Add components for the voxel's type and state
            // Use PositionVoxel, TypeVoxel, and StateVoxel components
            // to represent the voxel's properties
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
          query: Query<Entity, With<PositionVoxel>>,
        ) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
     
}

