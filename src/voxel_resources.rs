use bevy::ecs::system::Resource;
use bevy::math::IVec3;
use bevy::ecs::component::Component;
use bevy::utils::HashMap;
use bevy::ecs::entity::Entity;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::render::mesh::Mesh;


//Used in main

#[derive(Component)]
pub struct Ground;

//Used in Voxel Structure
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VoxelType {
    Tile,
    Wire,
    Out,
    Not,
    And,
    Or,
    Xor,
    Switch,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub is_on: bool,
}


#[derive(Resource)]
pub struct VoxelWorld {
    pub voxels: HashMap<IVec3, Voxel>,
    pub entities: HashMap<IVec3, Entity>,
}

#[derive(Resource, Clone, Copy)]
pub struct VoxelSelector {
    pub current_index: usize,
}

// Used in player
#[derive(Component)]
pub struct CameraRotation {
    pub pitch: f32,
    pub yaw: f32,
}

// Used in voxel lib
#[derive(Resource)]
pub struct VoxelState {
    pub position: IVec3,
    pub adjacent: IVec3,
    pub voxel_type: Option<VoxelType>,
    pub in_range: bool,
    pub is_on: Option<bool>,
    pub selected: Option<VoxelType>,
}

// Used in Voxel Assets
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
