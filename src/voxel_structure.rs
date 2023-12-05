use bevy::{
    asset::Handle,
    ecs::system::Commands,
    math::IVec3,
    pbr::{PbrBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
    utils::{default, hashbrown},
};

use super::voxel_resources::*;
use hashbrown::HashMap as HashbrownMap;

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld { 
            voxels: HashbrownMap::new(),
            entities: HashbrownMap::new(), // Initialize the entities HashMap
        }
    }

    pub fn get_voxel(&self, position: IVec3) -> Option<&Voxel> {
        self.voxels.get(&position)
    }

    pub fn set_voxel(&mut self, commands: &mut Commands, position: IVec3, voxel: Voxel, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) {
        // Create the entity for the new voxel
        let entity = commands.spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position.as_vec3()),
            ..default()
        }).id();

        self.voxels.insert(position, voxel);
        self.entities.insert(position, entity);
    }

    pub fn remove_voxel(&mut self, commands: &mut Commands, position: &IVec3) {
        if let Some(entity) = self.entities.remove(position) {
            commands.entity(entity).despawn();
        }
        self.voxels.remove(position);
    }
}

impl VoxelSelector {
    pub fn new() -> Self {
        VoxelSelector { current_index: 0 }
    }

    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % 8;
    }

    pub fn previous(&mut self) {
        if self.current_index == 0 {
            self.current_index = 7;
        } else {
            self.current_index -= 1;
        }
    }

    pub fn current_voxel_type(&self) -> VoxelType {
        match self.current_index {
            0 => VoxelType::Tile,
            1 => VoxelType::Wire,
            2 => VoxelType::Out,
            3 => VoxelType::Not,
            4 => VoxelType::And,
            5 => VoxelType::Or,
            6 => VoxelType::Xor,
            _ => VoxelType::Switch,
        }
    }
}