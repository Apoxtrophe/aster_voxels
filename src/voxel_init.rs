use crate::voxel_assets;

use super::voxel_structure::*;
use super::voxel_assets::*;
use super::config::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn voxel_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_world: ResMut<VoxelWorld>,
    mut voxel_assets: ResMut<VoxelAssets>,
) {
}