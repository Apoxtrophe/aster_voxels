use std::time::Duration;

use bevy::render::color::Color;

//Screen
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

//Player
pub const YAW_SPEED: f32 = 12.0;
pub const PITCH_SPEED: f32 = 8.0;
pub const AIR_ACCELERATION: f32 = 80.0;
pub const PLAYER_HEIGHT: f32 = 1.5;

pub const CAMERA_HEIGHT: f32 = 0.0;
pub const CAMERA_RADIUS: f32 = 0.75;
pub const FIELD_OF_VIEW: f32 = 5.0;

//Player Raycasting
pub const INTERACTION_DISTANCE: f32 = 10.0;

// Simulation Settings
pub const LOGIC_RATE: u64 = 100;

// Benchmarking
pub const BENCHMARKING: bool = false;
pub const BENCHMARK_SIZE: i32 = 100;

// Performance Metrics
pub const ONE_SECOND: Duration = Duration::from_secs(1);

// World Generation
pub const WORLD_SIZE: i32 = 128;
pub const WORLD_HEIGHT: f32 = 0.5;
pub const TEXTURE_BIAS: usize = 24;
pub const NORMALS_MULTIPLIER: f32 = 0.5;
pub const TERRAIN_HEIGHT_VARIANCE: f32 = 0.7;
pub const TERRIAN_ROUGHNESS: f64 = 0.1;

pub const GROUND_RELFECTANCE: f32 = 0.0;
pub const GROUND_METALLIC: f32 = 0.0;
pub const GROUND_ROUGHNESS: f32 = 1.0;
 

// Voxel Graphics
pub const V_ROUGHNESS: f32 = 1.0;
pub const V_METALLIC: f32 = 1.0;
pub const V_REFLECTANCE: f32 = 0.0;

//pub const V_EMMITANCE_COLOR: Color = Color::DARK_GRAY;

pub const V_TEXTURE_ATLAS_SIZE: usize = 8;

// Lighting
pub const SUN_ANGLE: f32 = 65.0;
pub const SUN_INTENSITY: f32 = 100000.0;
pub const SUN_SHADOWS: bool = true;
pub const SUN_LOCATION: [f32; 3] = [10.0, 10.0, 0.0];

pub const AMBIENT_INTENSITY: f32 = 0.6;
pub const AMBIENT_COLOR: Color = Color::BEIGE;

// Shadow
pub const SHADOW_CASCADES: usize = 4;
pub const SHADOW_DISTANCE: f32 = 100.0;
pub const FIRST_CASCADE_BOUND: f32 = 15.0;
pub const OVERLAP_PROPORTION: f32 = 0.3;