use std::time::Duration;

//Screen
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

//Player
pub const PLAYER_SPEED: f32 = 8.0;
pub const PLAYER_SPRINT: f32 = 14.0;
pub const MOUSE_SENSITIVITY: f32 = 0.05;

//Player Raycasting
pub const INTERACTION_DISTANCE: f32 = 10.0;

// Simulation Settings
pub const LOGIC_RATE: u64 = 100;

// Benchmarking
pub const BENCHMARKING: bool = false;

// Performance Metrics
pub const ONE_SECOND: Duration = Duration::from_secs(1);