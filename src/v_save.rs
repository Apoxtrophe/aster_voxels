use bevy::asset::Assets;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{Event, EventWriter};
use bevy::ecs::system::{Commands, Local, Query, Res, ResMut};
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::pbr::StandardMaterial;
use bevy::render::mesh::Mesh;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Write};
use crate::v_components::{PositionVoxel, StateVoxel, TypeVoxel};
use crate::v_graphics::VoxelAssets;
use crate::v_main_menu::{SelectedWorld, WorldName};
use crate::v_structure::Voxel;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SavedWorld {
    pub voxels: Vec<(PositionVoxel, TypeVoxel, StateVoxel)>,
}

#[derive(Event)]
pub struct SaveEvent;

fn save_world(
    query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    world_name: &str,
    mut save_event_writer: EventWriter<SaveEvent>,
) -> io::Result<()> {
    let world_data: Vec<_> = query.iter().map(|(_, pos, typ, state)| (*pos, *typ, *state)).collect();
    let saved_world = SavedWorld { voxels: world_data };
    let serialized = serde_json::to_string(&saved_world)?;

    let file_path = format!("assets/Saves/{}.json", world_name);
    File::create(file_path)?.write_all(serialized.as_bytes())?;

    save_event_writer.send(SaveEvent);
    Ok(())
}

pub fn load_world(world_name: &str) -> io::Result<SavedWorld> {
    let file_path = format!("assets/Saves/{}.json", world_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn check_for_save_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    world_name: Res<WorldName>,
    mut save_event_writer: EventWriter<SaveEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        if let Err(e) = save_world(query, &world_name.0, save_event_writer) {
            eprintln!("Failed to save world: {}", e);
        } else {
            println!("World saved successfully.");
        }
    }
}

pub fn world_loader(
    mut voxel: ResMut<Voxel>,
    mut commands: Commands,
    voxel_assets: Res<VoxelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected_world: Res<SelectedWorld>,
    mut world_name: ResMut<WorldName>,
) {
    if let Some(world_name_str) = &selected_world.0 {
        if let Ok(saved_world) = load_world(world_name_str) {
            // Update the WorldName resource with the loaded world name
            world_name.0 = world_name_str.clone();

            for (voxel_position, voxel_type, voxel_state) in saved_world.voxels {
                voxel.lean_place(
                    &mut commands,
                    voxel_position.0,
                    voxel_type,
                    voxel_state.0,
                    &voxel_assets,
                    &mut meshes,
                    &mut materials,
                );
            }
        } else {
            eprintln!("Failed to load world: {}", world_name_str);
        }
    }
}

pub fn autosave_system(
    query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    world_name: Res<WorldName>,
    mut autosave_triggered: Local<bool>,
    mut save_event_writer: EventWriter<SaveEvent>,
) {
    let current_time = chrono::Local::now();
    let (current_minute, current_second) = (current_time.minute(), current_time.second());

    if current_minute % 5 == 0 && current_second == 0 {
        if !*autosave_triggered {
            if let Err(e) = save_world(query, &world_name.0, save_event_writer) {
                println!("Failed to save world: {}", e);
            } else {
                println!("World saved successfully.");
            }

            *autosave_triggered = true;
            println!("Autosave triggered at {:02}:{:02}", current_time.hour(), current_minute);
        }
    } else {
        *autosave_triggered = false;
    }
}