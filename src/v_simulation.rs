use bevy::prelude::*;
use crate::v_structure::{PositionVoxel, TypeVoxel, StateVoxel};
use std::collections::HashSet;
use bevy::math::IVec3;

#[derive(Resource)]
pub struct MyTimer(pub Timer);

pub fn logic_operation_system(
    time: Res<Time>, 
    mut timer: ResMut<MyTimer>,
    mut voxel_query: Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut changes = Vec::new();
        let mut visited = HashSet::new(); 

        for (entity, position_voxel, type_voxel, state_voxel) in voxel_query.iter() {
            match type_voxel {
                TypeVoxel::Out => {
                    let is_on = process_out_logic(position_voxel.0, &voxel_query);
                    changes.push((entity, is_on));
                    println!("Starting propagation from: {:?}", position_voxel.0); // Debug print
                    dfs_propagate(position_voxel.0, &voxel_query, &mut visited, is_on, &mut changes);
                },
                // Add other voxel types here if needed
                _ => (),
            }
        }
        apply_changes(&mut voxel_query, changes)
    }
}

fn dfs_propagate(
    current_position: IVec3,
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
    visited: &mut HashSet<IVec3>,
    new_state: bool,
    changes: &mut Vec<(Entity, bool)>
) {
    // Check if the current voxel is already visited
    if visited.contains(&current_position) {
        println!("Visited: {:?}", current_position); // Debug print
        return;
    }

    // Mark the current voxel as visited
    visited.insert(current_position);

    // Iterate over adjacent positions
    for adj_pos in get_adjacent_positions(current_position).iter() {
        // Find adjacent wire voxels and propagate the state
        for (entity, pos_voxel, type_voxel, _) in voxel_query.iter() {
            if *pos_voxel == PositionVoxel(*adj_pos) && matches!(type_voxel, TypeVoxel::Wire) {
                println!("Propagating to: {:?} with state: {}", adj_pos, new_state); // Debug print

                // Add the adjacent wire voxel to the changes vector
                changes.push((entity, new_state));

                // Recursively call dfs_propagate to continue the propagation
                dfs_propagate(*adj_pos, voxel_query, visited, new_state, changes);
            }
        }
    }
}



fn process_out_logic(
    position: IVec3,
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>
) -> bool {
    get_adjacent_positions(position).iter()
        .any(|adj_pos| voxel_query.iter()
            .any(|(_, pos_voxel, type_voxel, state_voxel)| 
                *adj_pos == pos_voxel.0 && !matches!(type_voxel, TypeVoxel::Tile |TypeVoxel::Wire| TypeVoxel::Out) && state_voxel.0))
}

fn apply_changes(
    voxel_query: &mut Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
    changes: Vec<(Entity, bool)>
) {
    for (entity, new_state) in changes {
        if let Ok(mut state_voxel) = voxel_query.get_component_mut::<StateVoxel>(entity) {
            state_voxel.0 = new_state;
        }
    }
}

pub fn get_adjacent_positions(position: IVec3) -> Vec<IVec3> {
    vec![
        position + IVec3::new(1, 0, 0),
        position + IVec3::new(-1, 0, 0),
        position + IVec3::new(0, 1, 0),
        position + IVec3::new(0, -1, 0),
        position + IVec3::new(0, 0, 1),
        position + IVec3::new(0, 0, -1),
    ]
}
