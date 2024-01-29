use bevy::{ecs::component::Component, math::IVec3};

#[derive(Component)]
pub struct Ground;

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

#[derive(Component)]
pub struct Sun;