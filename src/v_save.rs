use bevy::asset::Assets;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{Event, EventWriter};
use bevy::ecs::system::{Commands, Local, Query, Res, ResMut};
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::pbr::StandardMaterial;
use bevy::render::mesh::Mesh;
use serde::{Serialize, Deserialize};
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
    let mut world_data = Vec::new();

    for (_, pos, typ, state) in query.iter() {
        world_data.push((*pos, *typ, *state));
    }

    let saved_world = SavedWorld { voxels: world_data };
    let serialized = serde_json::to_string(&saved_world)?;

    let file_name = format!("{}.json", world_name);
    let file_path = format!("assets/Saves/{}", file_name);
    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;


    // trigger save widget
    
    save_event_writer.send(SaveEvent);
    Ok(())
}

pub fn load_world(world_name: &str) -> io::Result<SavedWorld> {
    let file_name = format!("{}.json", world_name);
    let file_path = format!("assets/Saves/{}", file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let saved_world: SavedWorld = serde_json::from_reader(reader)?;
    Ok(saved_world)
}

pub fn check_for_save_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    world_name: Res<WorldName>,

    save_event_writer: EventWriter<SaveEvent>,
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
) {
    if let Some(world_name) = &selected_world.0 {
        match load_world(world_name) {
            Ok(saved_world) => {
                // Iterate over the saved world data to recreate entities
    
                println!("Loading world...");
                for (voxel_position, voxel_type, voxel_state) in saved_world.voxels {
                    let position = voxel_position.0;
                    let state = voxel_state.0;
                    voxel.lean_place(
                        &mut commands,
                        position,
                        voxel_type,
                        state,
                        &voxel_assets,
                        &mut meshes,
                        &mut materials,
                    )
                }
            },
            Err(e) => eprintln!("Failed to load world: {}", e),
        }
    }
}

pub fn autosave_system(
    // Other necessary resources and components for saving
    query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
    world_name: Res<WorldName>,
    mut autosave_triggered: Local<bool>,

    save_event_writer: EventWriter<SaveEvent>,
) {
    let current_time = chrono::Local::now();
    let current_minute = current_time.minute();

    if current_minute % 5 == 0 && current_time.second() == 0 {
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