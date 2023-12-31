use std::time::Duration;

use bevy::{ecs::{system::{Commands, ResMut, Res, Resource}, schedule::NextState}, asset::{AssetServer, Assets, Handle}, render::texture::Image, time::{Timer, TimerMode}};

use crate::{AppState, v_structure::Voxel, v_selector::VoxelSelector, v_lib::VoxelInfo, v_simulation::MyTimer, v_config::LOGIC_RATE, v_performance::PerformanceMetrics};



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
    
    

    let logic_atlas_handle: Handle<Image> = asset_server.load("TexturePack/LogicAtlas.png");
    let world_gen_grass: Handle<Image> = asset_server.load("TexturePack/GrassAtlas.png");
    let crosshair: Handle<Image> = asset_server.load("Crosshair.png");

    texture_handles.add_image_handle(logic_atlas_handle);
    texture_handles.add_image_handle(world_gen_grass);
    texture_handles.add_image_handle(crosshair);

    commands.insert_resource(texture_handles);

    // Initialize the voxel world
    commands.insert_resource(Voxel::new());

    commands.insert_resource(VoxelSelector::new());

    commands.insert_resource(VoxelInfo::new());

    commands.insert_resource(MyTimer(Timer::new(
        Duration::from_millis(LOGIC_RATE),
        TimerMode::Repeating,
    )));

    commands.insert_resource(PerformanceMetrics::new());
    
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