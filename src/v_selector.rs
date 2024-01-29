use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, ResMut, Resource},
    }, input::mouse::MouseWheel, ui::BorderColor
};

use crate::{v_components::TypeVoxel};

#[derive(Resource, Clone, Copy)]
pub struct VoxelSelector {
    pub current_index: usize,
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

    pub fn current_voxel_type(&self) -> TypeVoxel {
        match self.current_index {
            0 => TypeVoxel::Tile,
            1 => TypeVoxel::Wire,
            2 => TypeVoxel::Out,
            3 => TypeVoxel::Switch,
            4 => TypeVoxel::And,
            5 => TypeVoxel::Or,
            6 => TypeVoxel::Xor,
            _ => TypeVoxel::Not,
        }
    }
}
