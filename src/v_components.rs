use bevy::{ecs::component::Component, math::IVec3};

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct CameraRotation {
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PositionVoxel(pub IVec3);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum TypeVoxel {
    Tile,
    Wire,
    Out,
    Not,
    And,
    Or,
    Xor,
    Switch,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct StateVoxel(pub bool);