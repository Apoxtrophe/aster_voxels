use bevy::prelude::*;

use std::time::Duration;



use crate::{v_config::SIMULATION_RATE, v_hotbar::FadeTimer, v_lib::VoxelInfo, v_lighting::SunDirection, v_selector::VoxelSelector, v_simulation::MyTimer, v_structure::Voxel, AppState, PerformanceMetrics};

#[derive(Resource, Clone)]
pub struct TextureHandles {
    pub image_handles: Vec<Handle<Image>>,
}

impl TextureHandles {
    fn new() -> Self {
        TextureHandles {
            image_handles: Vec::new(),
            // initialize other fields...
        }
    }

    fn add_image_handle(&mut self, handle: Handle<Image>) {
        self.image_handles.push(handle);
    }

    // Methods to check if all assets are loaded, etc.
}



pub fn voxel_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("Beginning asset loading");
    // Load textures
    let mut texture_handles = TextureHandles::new();
    
    

    let logic_atlas_handle: Handle<Image> = asset_server.load("TexturePack/textures.png");
    let world_gen_grass: Handle<Image> = asset_server.load("TexturePack/Plaintile.png");
    let crosshair: Handle<Image> = asset_server.load("UserInterface/Crosshair.png");
    let hotbar_atlas: Handle<Image> = asset_server.load("UserInterface/HotbarIcons.png");

    texture_handles.add_image_handle(logic_atlas_handle);
    texture_handles.add_image_handle(world_gen_grass);
    texture_handles.add_image_handle(crosshair);
    texture_handles.add_image_handle(hotbar_atlas);

    commands.insert_resource(texture_handles);

    // Initialize the voxel world
    commands.insert_resource(Voxel::new());

    commands.insert_resource(VoxelSelector::new());

    commands.insert_resource(VoxelInfo::new());

    commands.insert_resource(MyTimer(Timer::new(
        Duration::from_millis(SIMULATION_RATE),
        TimerMode::Repeating,
    )));

    commands.insert_resource(PerformanceMetrics::new());

    commands.insert_resource(SunDirection::new());

    commands.insert_resource(FadeTimer::new());

}

pub fn asset_check(
    mut next_state: ResMut<NextState<AppState>>,
    texture_handles: Res<TextureHandles>,
    image_assets: Res<Assets<Image>>,
) {
    // Check if all assets are loaded. If so, go to the next state.
    let all_loaded = texture_handles.image_handles.iter().all(|handle| {
        image_assets.get(handle).is_some()
    });

    if all_loaded {
        println!("Moving onto GameSetup");
        next_state.set(AppState::GameSetup);
    }
}