use bevy::prelude::*;
use std::time::Duration;
use crate::{v_config::SIMULATION_RATE, v_hotbar::FadeTimer, v_lib::VoxelInfo, v_lighting::SunDirection, v_selector::VoxelSelector, v_simulation::MyTimer, v_structure::Voxel, AppState};

#[derive(Resource, Clone)]
pub struct TextureHandles {
    pub image_handles: Vec<Handle<Image>>,
}

impl TextureHandles {
    fn new(handles: Vec<Handle<Image>>) -> Self {
        TextureHandles {
            image_handles: handles,
        }
    }
}

pub fn voxel_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("Beginning asset loading");
    // Load textures

    let texture_paths = [
        "TexturePack/TexturePack_V4.png",
        "TexturePack/Tile.png",
        "UserInterface/Crosshair.png",
        "UserInterface/Hotbar_V2.png",
    ];

    let texture_handles = texture_paths.iter()
        .map(|path| asset_server.load(*path))
        .collect();

    let texture_handles = TextureHandles::new(texture_handles);

    commands.insert_resource(texture_handles);
    commands.insert_resource(Voxel::new());
    commands.insert_resource(VoxelSelector::new());
    commands.insert_resource(VoxelInfo::new());
    commands.insert_resource(MyTimer(Timer::new(
        Duration::from_millis(SIMULATION_RATE),
        TimerMode::Repeating,
    )));
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