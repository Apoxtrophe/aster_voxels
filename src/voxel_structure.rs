use std::collections::HashMap;
use bevy::math::IVec3;

pub enum VoxelType {
    Air,
    Tile,
    Wire,
}

pub struct Voxel {
    pub voxel_type: VoxelType,
    pub is_on: bool,
}

pub struct VoxelWorld {
    pub voxels: HashMap<IVec3, Voxel>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld { voxels: HashMap::new() }
    }

    pub fn get_voxel(&self, position: IVec3) -> Option<&Voxel> {
        self.voxels.get(&position)
    }

    pub fn set_voxel(&mut self, position: IVec3, voxel: Voxel) {
        self.voxels.insert(position, voxel);
    }
}