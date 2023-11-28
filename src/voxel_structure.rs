use std::collections::HashMap;
use bevy::{
    asset::Handle,
    ecs::{
        entity::Entity,
        system::{Commands, Resource},
    },
    math::IVec3,
    pbr::{PbrBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
    utils::default,
};

pub enum VoxelType {
    Air,
    Tile,
    Wire,
}

pub struct Voxel {
    pub voxel_type: VoxelType,
    pub is_on: bool,
}


#[derive(Resource)]
pub struct VoxelWorld {
    pub voxels: HashMap<IVec3, Voxel>,
    entities: HashMap<IVec3, Entity>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld { 
            voxels: HashMap::new(),
            entities: HashMap::new(), // Initialize the entities HashMap
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