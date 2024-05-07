use bevy::{ecs::component::Component, math::IVec3, reflect::Reflect};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Ground;

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
pub struct PositionVoxel(pub IVec3);

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
pub enum TypeVoxel {
    Tile,
    Wire,
    Out,
    Not,
    And,
    Or,
    Xor,
    Switch,
    DFlipFlop,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub struct StateVoxel(pub bool);

#[derive(Component)]
pub struct Sun;

#[derive(Component)] 
pub struct MainMenuEntity;

#[derive(Component)] 
pub struct MainCamera;
