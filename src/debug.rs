use bevy::ecs::system::Res;
use bevy_egui::{EguiContexts, egui};
use crate::voxel_lib::VoxelState;

pub fn ui_debug(
    mut contexts: EguiContexts,
    voxel_state: Res<VoxelState>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Position: {:?}", voxel_state.position));
        ui.label(format!("Adjacent: {:?}", voxel_state.adjacent));
        ui.label(format!("Voxel type: {:?}", voxel_state.voxel_type));
        ui.label(format!("Is in range: {:?}", voxel_state.in_range));
        ui.label(format!("Activated: {:?}", voxel_state.is_on));
        ui.label(format!("Selected voxel type: {:?}", voxel_state.selected));
    });
}