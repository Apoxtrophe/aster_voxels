use crate::{v_lib::VoxelInfo, v_performance::PerformanceMetrics};
use bevy::
    ecs::system::Res
;
use bevy_egui::{egui, EguiContexts};

pub fn ui_debug(
    mut contexts: EguiContexts,
    voxel_state: Res<VoxelInfo>,
    performance_metrics: Res<PerformanceMetrics>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Position: {:?}", voxel_state.position));
        ui.label(format!("Voxel type: {:?}", voxel_state.voxel_type));
        ui.label(format!("Activated: {:?}", voxel_state.is_on));
        ui.label(format!("Selected voxel type: {:?}", voxel_state.selected));
        if let Some(avg_frame_time) = performance_metrics.average_frame_time() {
            let avg_frame_time_ms = avg_frame_time.as_secs_f32() * 1000.0;
            ui.label(format!("Average frame time: {:.2} ms", avg_frame_time_ms));
        } else {
            ui.label("Average frame time: Calculating...");
        }
        ui.label(format!("FPS: {:?} /s", performance_metrics.fps));
        ui.label(format!("CPU usage: {:?} %", performance_metrics.cpu_usage));
        ui.label(format!(
            "Memory usage: {:?} GB",
            performance_metrics.memory_usage / 1073741824
        ));
        ui.label(format!(
            "Voxel count: {:?}",
            performance_metrics.entity_count
        ));
    });
}
