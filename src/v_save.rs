use bevy::asset::Assets;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::pbr::StandardMaterial;
use bevy::render::mesh::Mesh;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufReader, Write};

use crate::v_components::{PositionVoxel, StateVoxel, TypeVoxel};
use crate::v_graphics::VoxelAssets;
use crate::v_structure::Voxel;


#[derive(Serialize, Deserialize)]
pub struct SavedWorld {
    pub voxels: Vec<(PositionVoxel, TypeVoxel, StateVoxel)>,
}

fn save_world(query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>) -> io::Result<()> {
    let mut world_data = Vec::new();

    for (_, pos, typ, state) in query.iter() {
        world_data.push((*pos, *typ, *state));
    }

    let saved_world = SavedWorld { voxels: world_data };
    let serialized = serde_json::to_string(&saved_world)?;

    let mut file = File::create("save_file.json")?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

pub fn load_world() -> io::Result<SavedWorld> {
    let file = File::open("save_file.json")?;
    let reader = BufReader::new(file);
    let saved_world: SavedWorld = serde_json::from_reader(reader)?;

    Ok(saved_world)
}

pub fn check_for_save_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &PositionVoxel, &TypeVoxel, &StateVoxel)>,
) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        if let Err(e) = save_world(query) {
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
) {
    // Attempt to load the saved world
    match load_world() {
        Ok(saved_world) => {
            // Iterate over the saved world data to recreate entities

            println!("Loading world...");
            for (voxel_position, voxel_type, voxel_state) in saved_world.voxels {
                // Here you would use the loaded data to spawn entities
                // For example:

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