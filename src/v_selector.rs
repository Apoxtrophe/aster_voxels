use bevy::ecs::system::Resource;
use crate::{v_components::TypeVoxel, v_config::HOTBAR_ELEMENT_NUMBER};

#[derive(Resource, Clone, Copy)]
pub struct VoxelSelector {
    pub current_index: usize,
}

impl VoxelSelector {
    pub fn new() -> Self {
        VoxelSelector { current_index: 0 }
    }

    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % HOTBAR_ELEMENT_NUMBER;
    }

    pub fn previous(&mut self) {
        self.current_index = (self.current_index + HOTBAR_ELEMENT_NUMBER - 1) % HOTBAR_ELEMENT_NUMBER;
    }

    pub fn current_voxel_type(&self) -> TypeVoxel {
        match self.current_index {
            0 => TypeVoxel::Tile,
            1 => TypeVoxel::Wire,
            2 => TypeVoxel::Out,
            3 => TypeVoxel::Switch,
            4 => TypeVoxel::And,
            5 => TypeVoxel::Or,
            6 => TypeVoxel::Xor,
            7 => TypeVoxel::Not,
            _ => TypeVoxel::DFlipFlop,
        }
    }
}