use bevy::render::color::{self, Color};
//Screen
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

//Player
pub const PLAYER_YAW_SPEED: f32 = 12.0;
pub const PLAYER_PITCH_SPEED: f32 = 8.0;
pub const PLAYER_CAMERA_HEIGHT: f32 = 0.0;
pub const PLAYER_CAMERA_RADIUS: f32 = 0.75;
pub const PLAYER_FOV: f32 = 5.0;
pub const PLAYER_INTERACTION_MAX: f32 = 10.0;

// Simulation Settings
pub const SIMULATION_RATE: u64 = 100;

// World Generation
pub const WORLD_SIZE: i32 = 256;
pub const WORLD_HEIGHT_OFFSET: f32 = 0.25;
pub const WORLD_REFLECTANCE: f32 = 0.0;
pub const WORLD_METALLIC: f32 = 0.0;
pub const WORLD_PERCIEVED_ROUGHNESS: f32 = 1.0;

// Voxel Graphics
pub const VOXEL_PERCIEVED_ROUGHNESS: f32 = 1.0;
pub const VOXEL_METALLIC: f32 = 1.0;
pub const VOXEL_REFLECTANCE: f32 = 0.0;
pub const VOXEL_ATLAS_SIZE: usize = 18;

// Lighting
pub const SUN_INTENSITY: f32 = 10000.0;
pub const SUN_ANGLE: f32 = 65.0;
pub const SUN_DAY_NIGHT: bool = false;
pub const SUN_DAY_LENGTH: f32 = 0.01;
pub const SUN_TIMER_RATE: u64 = 100;
pub const AMBIENT_INTENSITY: f32 = 500.0;
pub const AMBIENT_COLOR: Color = Color::BEIGE;

// Hotbar
pub const HOTBAR_ELEMENT_NUMBER: usize = 9;
pub const HOTBAR_SLOT_SIZE: f32 = 96.0;
pub const HOTBAR_SPACING: f32 = 5.0;
pub const HOTBAR_ABOVE_BOTTOM: f32 = 10.0;
pub const HOTBAR_BORDER_SIZE: f32 = 10.0;
pub const HOTBAR_BACKGROUND_COLOR: Color = Color::GRAY;
pub const HOTBAR_BORDER_COLOR: Color = color::Color::ORANGE_RED;

// Hotbar Descriptor
pub const DESCRIPTOR_FONT_SIZE: f32 = 64.0;
pub const DESCRIPTOR_COLOR: Color = Color::SEA_GREEN;
pub const DESCRIPTOR_BOTTOM: f32 = 8.0;
pub const DESCRIPTOR_RIGHT: f32 = 50.0;
pub const DESCRIPTOR_FADE_TIMER: f32 = 1.0;

// Main menu
