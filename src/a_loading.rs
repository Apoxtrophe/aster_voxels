use crate::{
    v_components::MainMenuEntity, v_config::SIMULATION_RATE, v_hotbar::FadeTimer, v_lib::VoxelInfo,
    v_lighting::SunDirection, v_main_menu::clear_main_menu_entities, v_selector::VoxelSelector,
    v_simulation::MyTimer, v_structure::Voxel, v_plugins::SpeedBar, AppState,
};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource, Clone)]
pub struct TextureHandles {
    pub image_handles: Vec<Handle<Image>>,
}

impl TextureHandles {
    fn new(handles: Vec<Handle<Image>>) -> Self {
        Self {
            image_handles: handles,
        }
    }
}

#[derive(Resource)]
pub struct SaveNotificationTimer(pub Timer);

pub fn voxel_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Beginning asset loading");

    let texture_handles = load_textures(&asset_server);

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
    commands.insert_resource(SpeedBar::new());
}

fn load_textures(asset_server: &Res<AssetServer>) -> TextureHandles {
    let texture_paths = [
        "TexturePack/TexturePack_V4.png",
        "TexturePack/Tile.png",
        "UserInterface/Crosshair.png",
        "UserInterface/Hotbar_V2.png",
        "UserInterface/SaveIcon.png",
        "UserInterface/SpeedBar.png",
    ];

    let texture_handles = texture_paths
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();

    TextureHandles::new(texture_handles)
}

pub fn asset_check(
    mut next_state: ResMut<NextState<AppState>>,
    texture_handles: Res<TextureHandles>,
    image_assets: Res<Assets<Image>>,
    commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
) {
    if all_assets_loaded(&texture_handles, &image_assets) {
        println!("Moving onto GameSetup");
        next_state.set(AppState::GameSetup);
        clear_main_menu_entities(commands, query);
    }
}

fn all_assets_loaded(texture_handles: &TextureHandles, image_assets: &Assets<Image>) -> bool {
    texture_handles
        .image_handles
        .iter()
        .all(|handle| image_assets.get(handle).is_some())
}

#[derive(Resource, Clone)]
pub struct AudioAssets {
    pub click_sound: Handle<AudioSource>,
}
