use bevy::{app::Main, asset::{AssetServer, Assets}, core_pipeline::core_2d::Camera2dBundle, ecs::{component::Component, entity::Entity, query::With, schedule::NextState, system::{Commands, Query, Res, ResMut}}, hierarchy::{BuildChildren, Children}, render::color::Color, sprite::TextureAtlasLayout, text::{JustifyText, TextStyle}, transform::components::Transform, ui::{node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle}, widget::Button, AlignContent, BackgroundColor, BorderColor, Display, Interaction, JustifyContent, JustifyItems, Overflow, PositionType, Style, UiRect, Val, ZIndex}, utils::default, window::{PrimaryWindow, Window, WindowResolution}};

use crate::{v_components::MainMenuEntity, AppState};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use bevy::prelude::Resource;

#[derive(Component)]
pub enum MenuButton {
    Load,
    New,
    Settings,
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {

    windows.single_mut().resolution = WindowResolution::new(1920.0, 1080.0);
    // UI CAMERA
    commands.spawn(Camera2dBundle::default()).insert(MainMenuEntity);
    
    let main_image_handle = asset_server.load("UserInterface/logicalogo2.png");
    
    let parent = NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_content: AlignContent::Center,
            overflow: Overflow::clip(),
            justify_items: JustifyItems::Center,
            ..default()  
        },
        ..default()
        
    };
    
    let child_main = ImageBundle {
        style: Style {
            // Size of the image
            width: Val::Px(1920.0),
            height: Val::Px(1080.0),
            
            ..default()
        },
        z_index: ZIndex::Local(-5),
        image: main_image_handle.into(),
        // This makes the image non-interactive and ignores pointer events
        ..default()
    };

    let parent_entity = commands.spawn(parent)
        .insert(MainMenuEntity)
        .id();

    let child_main = commands.spawn(child_main)
        .insert(MainMenuEntity)
        .id();

    let load_button_entity = create_load_button(&mut commands, &asset_server);
    let new_button_entity = create_new_button(&mut commands, &asset_server);
    let settings_button_entity = create_settings_button(&mut commands, &asset_server);

    commands.entity(parent_entity).push_children(&[
        child_main,
        load_button_entity,
        new_button_entity,
        settings_button_entity,
    ]);
}

pub fn main_menu_buttons(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuButton, // Add the MenuButton component to the query
        ),
        With<Button>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (entity, interaction, mut color, mut border_color, menu_button) in interaction_query.iter_mut() {   
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::RED;
                match menu_button {
                    MenuButton::Load => {
                        next_state.set(AppState::LoadWorldMenu);
                    }
                    MenuButton::New => {
                        next_state.set(AppState::WorldNaming);
                    }
                    MenuButton::Settings => {}
                }
            },
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            },
            Interaction::None => {
                border_color.0 = Color::Rgba { red: (0.0), green: (0.0), blue: (0.0), alpha: (0.0) };
            },
        }
    }
}

pub fn clear_main_menu_entities(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn create_load_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let button_load = ButtonBundle {
        style: Style {
            min_width: Val::Px(200.0),
            min_height: Val::Px(80.0),
            max_height: Val::Px(80.0),
            left: Val::Percent(-50.0),
            top: Val::Percent(50.0),
            border: UiRect::all(Val::Px(5.0)),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        
        z_index: ZIndex::Local(5),
        background_color: Color::Rgba { red: (0.0), green: (0.0), blue: (0.0), alpha: (0.0) }.into(),
        transform: Transform {
            ..default()
        },
        ..default()
    };

    let load_text = TextBundle::from_section(
        "LOAD", // The text you want on the button
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"), // Load your font
            font_size: 80.0, // Set the font size
            color: Color::WHITE, // Text color
        },
    )
    .with_text_justify(JustifyText::Center) // Align text to the center
    .with_style(Style {
        ..default()
    });

    commands
        .spawn(button_load)
        .insert(MainMenuEntity)
        .insert(MenuButton::Load)
        .with_children(|parent| {
            parent.spawn(load_text).insert(MainMenuEntity);
        })
        .id()
}

fn create_new_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let button_new = ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            min_width: Val::Px(200.0),
            min_height: Val::Px(80.0),
            max_height: Val::Px(80.0),
            left: Val::Percent(50.0),
            top: Val::Percent(60.0),
            border: UiRect::all(Val::Px(5.0)),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        
        z_index: ZIndex::Local(5),
        background_color: Color::Rgba { red: (0.0), green: (0.0), blue: (0.0), alpha: (0.0) }.into(),
        transform: Transform {
            ..default()
        },
        ..default()
    };

    let new_text = TextBundle::from_section(
        "NEW", // The text you want on the button
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"), // Load your font
            font_size: 80.0, // Set the font size
            color: Color::WHITE, // Text color
        },
    )
    .with_text_justify(JustifyText::Center) // Align text to the center
    .with_style(Style {
        ..default()
    });

    commands
        .spawn(button_new)
        .insert(MainMenuEntity)
        .insert(MenuButton::New)
        .with_children(|parent| {
            parent.spawn(new_text).insert(MainMenuEntity);
        })
        .id()
}

fn create_settings_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let button_settings = ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            min_width: Val::Px(200.0),
            min_height: Val::Px(80.0),
            max_height: Val::Px(80.0),
            left: Val::Percent(50.0),
            top: Val::Percent(70.0),
            border: UiRect::all(Val::Px(5.0)),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        
        z_index: ZIndex::Local(5),
        background_color: Color::Rgba { red: (0.0), green: (0.0), blue: (0.0), alpha: (0.0) }.into(),
        transform: Transform {
            ..default()
        },
        ..default()
    };

    let settings_text = TextBundle::from_section(
        "SETTINGS", // The text you want on the button
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"), // Load your font
            font_size: 80.0, // Set the font size
            color: Color::WHITE, // Text color
        },
    )
    .with_text_justify(JustifyText::Center) // Align text to the center
    .with_style(Style {
        align_content: AlignContent::Center, //
        ..default()
    });

    commands
        .spawn(button_settings)
        .insert(MainMenuEntity)
        .insert(MenuButton::Settings)
        .with_children(|parent| {
            parent.spawn(settings_text).insert(MainMenuEntity);
        })
        .id()
}


// NEW button functionality and world naming
#[derive(Resource, Default)]
pub struct WorldName(pub String);

#[derive(Component)]
pub struct WorldNameInput {
    pub name: String,
}

pub fn setup_world_naming(
    mut commands: Commands,
) {
    commands.spawn(WorldNameInput {name: String::new()});
}

pub fn world_naming(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut world_name_input: Query<&mut WorldNameInput>,
    mut world_name: ResMut<WorldName>,
) {
    egui::Window::new("Name Your World")
        .collapsible(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            let mut world_name_input = world_name_input.single_mut();
            ui.text_edit_singleline(&mut world_name_input.name);

            if ui.button("Create World").clicked() {
                world_name.0 = world_name_input.name.clone();
                next_state.set(AppState::AssetLoading);
            }
        });
}

// Functionality for the "Load" button; loading a chosen world
#[derive(Resource, Default)]
pub struct SelectedWorld(pub Option<String>);

pub fn load_world_menu (
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut selected_world: ResMut<SelectedWorld>,
) {
    egui::Window::new("Load World")
    .collapsible(false)
    .resizable(false)
    .show(contexts.ctx_mut(), |ui| {
        if let Ok(entries) = std::fs::read_dir("assets/Saves") {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".json") {
                            let world_name = file_name.trim_end_matches(".json");
                            if ui.button(world_name).clicked() {
                                selected_world.0 = Some(world_name.to_string());
                                next_state.set(AppState::AssetLoading);
                            }
                        }
                    }
                }
            }
        }

    });
}