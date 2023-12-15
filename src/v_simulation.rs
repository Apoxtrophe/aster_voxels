use std::time::Duration;

use bevy::{ecs::system::{Resource, ResMut, Res}, utils::Instant, time::Time};

#[derive(Resource)]
pub struct SimulationTickrate{
    pub interval: Duration,
    pub last_update: Instant,
}

impl SimulationTickrate {
    pub fn new(interval_in_seconds:f64) -> Self {
        SimulationTickrate {
            interval: Duration::from_secs_f64(interval_in_seconds),
            last_update: Instant::now(),
            }
    }
}

pub fn tickrate_system(
    time: Res<Time>, 
    mut tickrate: ResMut<SimulationTickrate>,
) {
    if tickrate.last_update.elapsed() >= tickrate.interval {
        tickrate.last_update = Instant::now();
        println!("Hello World");
    }
}