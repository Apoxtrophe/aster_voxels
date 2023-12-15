use bevy::prelude::*;
use std::time::Duration;

use crate::v_structure::{PositionVoxel, TypeVoxel, StateVoxel};

#[derive(Resource)]
pub struct MyTimer(pub Timer);

// Logic system that operates on a tick rate
pub fn logic_operation_system(
    time: Res<Time>, 
    mut timer: ResMut<MyTimer>,
    mut voxel_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Collect data
        let mut changes = Vec::new();
        for (entity, position_voxel, type_voxel, state_voxel) in voxel_query.iter() {
            if let TypeVoxel::Out = type_voxel {
                let is_on = process_out_logic(position_voxel.0, &voxel_query);
                changes.push((entity, is_on));
            }
        }

        // Apply changes
        for (entity, new_state) in changes {
            if let Ok(mut state_voxel) = voxel_query.get_component_mut::<StateVoxel>(entity) {
                state_voxel.0 = new_state;
            }
        }

        println!("Tick!");
    }
}



fn get_adjacent_positions(position: IVec3) -> Vec<IVec3> {
    vec![
        position + IVec3::new(1, 0, 0),
        position + IVec3::new(-1, 0, 0),
        position + IVec3::new(0, 1, 0),
        position + IVec3::new(0, -1, 0),
        position + IVec3::new(0, 0, 1),
        position + IVec3::new(0, 0, -1),
    ]
}

fn process_out_logic(
    position: IVec3,
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>
) -> bool {
    let adjacent_positions = get_adjacent_positions(position);

    for (_, pos_voxel, type_voxel, state_voxel) in voxel_query.iter() {
        if adjacent_positions.contains(&pos_voxel.0) {
            match type_voxel {
                TypeVoxel::Tile | TypeVoxel::Wire | TypeVoxel::Out => continue,
                _ => if state_voxel.0 { return true; }
            }
        }
    }

    false
}
