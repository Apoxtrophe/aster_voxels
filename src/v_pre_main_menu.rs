use std::os::windows;

use bevy::{asset::AssetServer, core::Name, core_pipeline::core_2d::Camera2dBundle, ecs::{entity::Entity, query::With, schedule::NextState, system::{Commands, Query, Res, ResMut}, world::Mut}, window::{CursorGrabMode, CursorIcon, PresentMode, PrimaryWindow, Window, WindowMode, WindowResolution, WindowTheme}};
use crate::{v_components::{MainCamera, MainMenuEntity}, v_config::{SCREEN_HEIGHT, SCREEN_WIDTH}, v_in_game_menu::despawn_all, AppState};

pub fn pre_main_menu_cleanup(
    mut next_state: ResMut<NextState<AppState>>,

    mut commands: Commands,
    entities: Query<(Entity, Option<&Name>)>,
    camera_query: Query<Entity, With<MainCamera>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    println!("Entering PreMainMenuCleanup");

    commands
        .spawn(Camera2dBundle::default())
        .insert(MainMenuEntity);

    setup_window(windows.single_mut());
    
    despawn_all(commands, entities, camera_query);
    
    next_state.set(AppState::MainMenu);
}

fn configure_window(window: &mut Window) {
    window.title = "Logica".to_string();
    window.resolution = WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    window.present_mode = PresentMode::AutoVsync;
    window.cursor.icon = CursorIcon::Crosshair;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.window_theme = Some(WindowTheme::Dark);
    window.mode = WindowMode::Windowed;
    window.cursor.visible = true;
    window.decorations = true;
}

fn setup_window(mut window: Mut<Window>) {
    configure_window(&mut window);
}