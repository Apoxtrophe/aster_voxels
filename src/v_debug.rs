use std::{collections::VecDeque, time::{Duration, Instant}};

use crate::{v_components::{PositionVoxel, StateVoxel, TypeVoxel}, v_config::{BENCHMARKING, BENCHMARK_SIZE, DEBUGGING, ONE_SECOND}, v_lib::VoxelInfo, VoxelAssets};
use bevy::{asset::Assets, ecs::{entity::Entity, system::{Commands, Query, Res, ResMut, Resource}}, math::IVec3, pbr::{PbrBundle, StandardMaterial}, render::mesh::Mesh, transform::components::Transform}
;
use bevy_egui::{egui, EguiContexts};








pub fn ui_debug(
    mut contexts: EguiContexts,
    voxel_state: Res<VoxelInfo>,
    performance_metrics: &Res<PerformanceMetrics>,
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

#[derive(Resource)]
pub struct OneTime;

pub fn benchmark(
    mut commands: Commands,
    voxel_assets: Res<VoxelAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    marker: Option<Res<OneTime>>,
) {
    if marker.is_none() && BENCHMARKING {
        let voxel_type = TypeVoxel::Xor;
        let voxel_mesh_handle = voxel_assets.create_voxel_mesh(voxel_type, &mut meshes);
        let atlas_material = voxel_assets.atlas_material(&mut materials);
        for j in 0..BENCHMARK_SIZE {
            for i in 0..BENCHMARK_SIZE {
                let position = IVec3::new(i, 1, j);

                commands
                    .spawn(PbrBundle {
                        mesh: voxel_mesh_handle.clone(),  // Use the UV mapped mesh
                        material: atlas_material.clone(), // Use the atlas material
                        transform: Transform::from_translation(position.as_vec3()),
                        ..Default::default()
                    })
                    .insert(PositionVoxel(position))
                    .insert(voxel_type)
                    .insert(StateVoxel(false));
            }
        }
        commands.insert_resource(OneTime);
    }
}

#[derive(Resource)]
pub struct PerformanceMetrics {
    frame_times: VecDeque<Duration>,
    last_update: Instant,
    pub fps: f32,
    system: sysinfo::System,
    pub cpu_usage: f32,
    pub last_cpu_update: Instant,
    pub memory_usage: u64,
    pub entity_count: usize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        PerformanceMetrics {
            frame_times: VecDeque::new(),
            last_update: Instant::now(),
            fps: 0.0,
            system: sysinfo::System::new_all(),
            cpu_usage: 0.0,
            last_cpu_update: Instant::now(),
            memory_usage: 0,
            entity_count: 0,
        }
    }

    pub fn update(&mut self, entities: Query<Entity>) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_update);

        self.frame_times.push_back(frame_time);

        let one_second_ago = now - ONE_SECOND;

        while self
            .frame_times
            .front()
            .map_or(false, |&t| self.last_update - t < one_second_ago)
        {
            self.frame_times.pop_front();
        }

        self.last_update = now;

        //Calculate fps
        if let Some(avg_frame_time) = self.average_frame_time() {
            self.fps = 1.0 / avg_frame_time.as_secs_f32();
        } else {
            self.fps = 0.0;
        }

        //Calculate CPU usage
        if now.duration_since(self.last_cpu_update) >= ONE_SECOND {
            self.entity_count = entities.iter().count() - 6;
            self.system.refresh_all();
            self.cpu_usage = self.system.global_cpu_info().cpu_usage() as f32;
            self.last_cpu_update = now;
            self.memory_usage = self.system.used_memory();
        }
    }

    pub fn average_frame_time(&self) -> Option<Duration> {
        let sum: Duration = self.frame_times.iter().sum();
        let count = self.frame_times.len();
        if count > 0 {
            Some(sum / count as u32)
        } else {
            None
        }
    }
}

pub fn performance_metrics_system(
    mut metrics: ResMut<PerformanceMetrics>,
    entities: Query<Entity>,
) {
    if DEBUGGING {
        metrics.update(entities);
    }
}
