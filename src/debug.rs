use bevy::{ecs::system::{Resource, Res, ResMut}, math::IVec3};
use bevy_egui::{EguiContexts, egui};

use crate::voxel_structure::{VoxelType, VoxelSelector};

#[derive(Resource, Debug)]
pub struct VoxelLookedAt {
    pub position: Option<IVec3>,
    pub voxel_type: Option<VoxelType>,
}

impl VoxelLookedAt {
    pub fn update(&mut self, position: IVec3, voxel_type: VoxelType) {
        self.position = Some(position);
        self.voxel_type = Some(voxel_type);
    }

    pub fn clear(&mut self) {
        self.position = None;
        self.voxel_type = None;
    }
}

pub fn ui_debug(
    mut contexts: EguiContexts,
    voxel_look: Res<VoxelLookedAt>,
    voxel_selector: ResMut<VoxelSelector>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        let voxel_type = voxel_selector.current_voxel_type();
        
        ui.label(format!("Selected Voxel: {:?}", voxel_type));
        match (voxel_look.position, voxel_look.voxel_type) {
            (Some(position), Some(voxel_type)) => {
                ui.label(format!("Position: {:?}", position));
                ui.label(format!("Voxel Type: {:?}", voxel_type));
            }
            _ => {
                ui.label("No voxel currently looked at");
            }
        }
    });
}