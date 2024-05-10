use bevy::math::IVec3;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::v_components::{PositionVoxel, StateVoxel, TypeVoxel};

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
        let voxel_map = create_voxel_map(&voxel_query);

        for (entity, position_voxel, type_voxel, _) in voxel_query.iter() {
            match type_voxel {
                TypeVoxel::Out => {
                    let is_on = process_out_logic(position_voxel.0, &voxel_map);
                    changes.push((entity, is_on));
                    dfs_propagate(position_voxel.0, &voxel_query, &mut visited, is_on, &mut changes);
                }
                TypeVoxel::And | TypeVoxel::Or | TypeVoxel::Xor | TypeVoxel::Not | TypeVoxel::DFlipFlop => {
                    let is_on = process_logic_gate(position_voxel.0, *type_voxel, &voxel_map);
                    changes.push((entity, is_on));
                }
                _ => (),
            }
        }
        apply_changes(&mut voxel_query, changes);
    }
}

fn create_voxel_map(voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>) -> HashMap<IVec3, (Entity, TypeVoxel, StateVoxel)> {
    voxel_query
        .iter()
        .map(|(entity, position_voxel, type_voxel, state_voxel)| (position_voxel.0, (entity, *type_voxel, *state_voxel)))
        .collect()
}

fn process_logic_gate(
    position: IVec3,
    voxel_type: TypeVoxel,
    voxel_map: &HashMap<IVec3, (Entity, TypeVoxel, StateVoxel)>,
) -> bool {
    let adjacent_positions = get_adjacent_positions(position);
    let (active_inputs, total_inputs) = adjacent_positions.iter().fold((0, 0), |(active, total), adj_pos| {
        if let Some((_, TypeVoxel::Wire, state_voxel)) = voxel_map.get(adj_pos) {
            (active + state_voxel.0 as usize, total + 1)
        } else {
            (active, total)
        }
    });

    match voxel_type {
        TypeVoxel::And => active_inputs == total_inputs && total_inputs > 0,
        TypeVoxel::Or => active_inputs > 0,
        TypeVoxel::Xor => active_inputs == 1,
        TypeVoxel::Not => total_inputs == 1 && active_inputs == 0,
        TypeVoxel::DFlipFlop => process_d_flip_flop_logic(position, voxel_map),
        _ => false,
    }
}

fn dfs_propagate(
    current_position: IVec3,
    voxel_query: &Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
    visited: &mut HashSet<IVec3>,
    new_state: bool,
    changes: &mut Vec<(Entity, bool)>,
) {
    if visited.contains(&current_position) {
        return;
    }

    visited.insert(current_position);

    for adj_pos in get_adjacent_positions(current_position).iter() {
        for (entity, pos_voxel, type_voxel, _) in voxel_query.iter() {
            if *pos_voxel == PositionVoxel(*adj_pos) && matches!(type_voxel, TypeVoxel::Wire) {
                changes.push((entity, new_state));
                dfs_propagate(*adj_pos, voxel_query, visited, new_state, changes);
            }
        }
    }
}

fn process_out_logic(
    position: IVec3,
    voxel_map: &HashMap<IVec3, (Entity, TypeVoxel, StateVoxel)>,
) -> bool {
    get_adjacent_positions(position).iter().any(|adj_pos| {
        voxel_map.get(adj_pos).map_or(false, |(_, type_voxel, state_voxel)| {
            matches!(type_voxel, TypeVoxel::And | TypeVoxel::Or | TypeVoxel::Xor | TypeVoxel::Not | TypeVoxel::DFlipFlop | TypeVoxel::Switch) && state_voxel.0
        })
    })
}

fn apply_changes(
    voxel_query: &mut Query<(Entity, &PositionVoxel, &TypeVoxel, &mut StateVoxel)>,
    changes: Vec<(Entity, bool)>,
) {
    let change_map: HashMap<Entity, bool> = changes.into_iter().collect();

    for (entity, _, _, mut state_voxel) in voxel_query.iter_mut() {
        if let Some(&new_state) = change_map.get(&entity) {
            state_voxel.0 = new_state;
        }
    }
}

pub fn get_adjacent_positions(position: IVec3) -> [IVec3; 6] {
    [
        position + IVec3::new(1, 0, 0),
        position + IVec3::new(-1, 0, 0),
        position + IVec3::new(0, 1, 0),
        position + IVec3::new(0, -1, 0),
        position + IVec3::new(0, 0, 1),
        position + IVec3::new(0, 0, -1),
    ]
}

fn process_d_flip_flop_logic(
    position: IVec3,
    voxel_map: &HashMap<IVec3, (Entity, TypeVoxel, StateVoxel)>,
) -> bool {
    let top_position = position + IVec3::new(0, 1, 0);
    let side_positions = [
        position + IVec3::new(1, 0, 0),
        position + IVec3::new(-1, 0, 0),
        position + IVec3::new(0, 0, 1),
        position + IVec3::new(0, 0, -1),
    ];

    let (signal, data) = side_positions.iter().fold((0, 0), |(signal, data), side_pos| {
        if let Some((_, TypeVoxel::Wire, state_voxel)) = voxel_map.get(side_pos) {
            (signal, data + state_voxel.0 as usize)
        } else {
            (signal, data)
        }
    });

    let signal = voxel_map
    .get(&top_position)
    .and_then(|(_, type_voxel, state_voxel)| {
        if matches!(type_voxel, TypeVoxel::Wire) {
            Some(state_voxel.0 as usize)
        } else {
            None
        }
    })
    .unwrap_or(0);
    let current_state = voxel_map.get(&position).map_or(false, |(_, _, state)| state.0);

    match (signal, data) {
        (s, d) if s > 0 && d > 0 => true,
        (s, 0) if s > 0 => false,
        _ => current_state,
    }
}