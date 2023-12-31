use bevy::ecs::system::Res;
use bevy::prelude::*;
use std::{time::{Duration, Instant}, collections::VecDeque};
use sysinfo::{System};

use crate::v_config::ONE_SECOND;

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

        while self.frame_times.front().map_or(false, |&t| self.last_update - t < one_second_ago) {
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
    time: Res<Time>, 
    mut metrics: ResMut<PerformanceMetrics>,
    entities: Query<Entity>,
) {
    metrics.update(entities);
}