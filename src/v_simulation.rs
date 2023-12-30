use crate::v_structure::{PositionVoxel, StateVoxel, TypeVoxel};
use bevy::math::IVec3;
use bevy::prelude::*;
use std::collections::HashSet;

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
                    dfs_propagate(
                        position_voxel.0,
                        &voxel_query,
                        &mut visited,
                        is_on,
                        &mut changes,
                    );
                }
                TypeVoxel::And | TypeVoxel::Or | TypeVoxel::Xor | TypeVoxel::Not => {
                    // Update state of logic gates without propagation
                    let is_on = process_logic_gate(position_voxel.0, *type_voxel, &voxel_query);
                    changes.push((entity, is_on));
                }
                // Add other voxel types here if needed
                _ => (),
            }
        }
        apply_changes(&mut voxel_query, changes)
    }
}

fn process_logic_gate(
    position: IVec3,
    voxel_type: TypeVoxel,
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
) -> bool {
    let adjacent_positions = get_adjacent_positions(position);
    let mut active_inputs = 0;
    let mut total_inputs = 0;

    for adj_pos in adjacent_positions.iter() {
        if let Some((_, state_voxel)) =
            voxel_query
                .iter()
                .find_map(|(_, pos_voxel, type_voxel, state_voxel)| {
                    if *adj_pos == pos_voxel.0 && matches!(type_voxel, TypeVoxel::Wire) {
                        Some((type_voxel, state_voxel))
                    } else {
                        None
                    }
                })
        {
            total_inputs += 1;
            if state_voxel.0 {
                active_inputs += 1;
            }
        }
    }

    match voxel_type {
        TypeVoxel::And => active_inputs == total_inputs && total_inputs > 0,
        TypeVoxel::Or => active_inputs > 0,
        TypeVoxel::Xor => active_inputs == 1,
        TypeVoxel::Not => total_inputs == 1 && active_inputs == 0,
        _ => false, // Default case for other types of voxels
    }
}

fn dfs_propagate(
    current_position: IVec3,
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
    visited: &mut HashSet<IVec3>,
    new_state: bool,
    changes: &mut Vec<(Entity, bool)>,
) {
    // Check if the current voxel is already visited
    if visited.contains(&current_position) {
        return;
    }

    // Mark the current voxel as visited
    visited.insert(current_position);

    // Iterate over adjacent positions
    for adj_pos in get_adjacent_positions(current_position).iter() {
        // Find adjacent wire voxels and propagate the state
        for (entity, pos_voxel, type_voxel, _) in voxel_query.iter() {
            if *pos_voxel == PositionVoxel(*adj_pos) && matches!(type_voxel, TypeVoxel::Wire) {
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
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
) -> bool {
    get_adjacent_positions(position).iter().any(|adj_pos| {
        voxel_query
            .iter()
            .any(|(_, pos_voxel, type_voxel, state_voxel)| {
                *adj_pos == pos_voxel.0
                    && !matches!(
                        type_voxel,
                        TypeVoxel::Tile | TypeVoxel::Wire | TypeVoxel::Out
                    )
                    && state_voxel.0
            })
    })
}

fn apply_changes(
    voxel_query: &mut Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
    changes: Vec<(Entity, bool)>,
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
