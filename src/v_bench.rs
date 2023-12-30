use std::borrow::BorrowMut;

use bevy::{prelude::*, ecs::system::adapter::new};

use crate::{v_graphics::VoxelAssets, v_selector::VoxelSelector, v_structure::{Voxel, TypeVoxel, PositionVoxel, StateVoxel}, v_lib::VoxelInfo, v_config::BENCHMARKING};




#[derive(Resource)]
pub struct MyOneTimeSystemMarker;

pub fn benchmark(
    voxel_assets: Res<VoxelAssets>,
    mut voxel_selector: ResMut<VoxelSelector>,
    mut commands: Commands,
    mut voxel: ResMut<Voxel>,
    voxel_info: Res<VoxelInfo>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    
    //mut commands: Commands,
     marker: Option<Res<MyOneTimeSystemMarker>>,
) {
    let mut voxel = Voxel::new();
    if marker.is_none() && BENCHMARKING{
        voxel_selector.current_index = 7;
        voxel.place(
            &mut commands,
            IVec3::new(10, 10, 10),
            &voxel_selector, 
            &voxel_assets, 
            materials, 
            meshes);
    
        commands.insert_resource(MyOneTimeSystemMarker);
    }
}