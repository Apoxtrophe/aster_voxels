use bevy::math::Vec3;
//Screen
pub const SCREEN_WIDTH: f32 = 600.0;
pub const SCREEN_HEIGHT: f32 = 400.0;

//Player
pub const PLAYER_SPEED: f32 = 8.0;
pub const PLAYER_SPRINT: f32 = 14.0;
pub const MOUSE_SENSITIVITY:f32 = 0.1;

//World
pub const STARTING_SIZE: i32 = 10;

//Player Raycasting
pub const DIST: Vec3 = Vec3::new(0.0, 0.0, -7.0);