use bevy::{
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        query::With,
        schedule::NextState,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{JustifyText, TextStyle},
    transform::components::Transform,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignContent, BackgroundColor, BorderColor, Display, Interaction, JustifyContent,
        JustifyItems, Overflow, PositionType, Style, UiRect, Val, ZIndex,
    },
    utils::default,
    window::{PrimaryWindow, Window, WindowResolution},
};
use crate::{v_components::MainMenuEntity, AppState};
use bevy::prelude::Resource;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};

#[derive(Component)]
pub enum MenuButton {
    Load,
    New,
    Settings,
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
    window.resolution = WindowResolution::new(1920.0, 1080.0);
    window.cursor.visible = true;

    commands
        .spawn(Camera2dBundle::default())
        .insert(MainMenuEntity);
    let main_image_handle = asset_server.load("UserInterface/logicalogo2.png");
    println!("Entering Main Menu");

    let parent_style = Style {
        display: Display::Flex,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_content: AlignContent::Center,
        overflow: Overflow::clip(),
        justify_items: JustifyItems::Center,
        ..default()
    };

    let child_main_style = Style {
        width: Val::Px(1920.0),
        height: Val::Px(1080.0),
        ..default()
    };

    let parent_entity = commands
        .spawn(NodeBundle {
            style: parent_style,
            ..default()
        })
        .insert(MainMenuEntity)
        .id();

    let child_main = commands
        .spawn(ImageBundle {
            style: child_main_style,
            z_index: ZIndex::Local(-5),
            image: main_image_handle.into(),
            ..default()
        })
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
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuButton,
        ),
        (With<Button>, With<MenuButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (_, interaction, _, mut border_color, menu_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::RED;
                match menu_button {
                    MenuButton::Load => next_state.set(AppState::LoadWorldMenu),
                    MenuButton::New => next_state.set(AppState::WorldNaming),
                    MenuButton::Settings => {}
                }
            }
            Interaction::Hovered => border_color.0 = Color::WHITE,
            Interaction::None => border_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.0),
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

#[derive(Resource, Default)]
pub struct WorldName(pub String);

#[derive(Component)]
pub struct WorldNameInput {
    pub name: String,
}

fn create_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text: &str,
    position: (Val, Val),
    menu_button: MenuButton,
) -> Entity {
    let button_style = Style {
        min_width: Val::Px(200.0),
        min_height: Val::Px(80.0),
        max_height: Val::Px(80.0),
        left: position.0,
        top: position.1,
        border: UiRect::all(Val::Px(5.0)),
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button = ButtonBundle {
        style: button_style,
        z_index: ZIndex::Local(5),
        background_color: Color::NONE.into(),
        ..default()
    };

    let text_bundle = TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"),
            font_size: 80.0,
            color: Color::WHITE,
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style::default());

    commands
        .spawn(button)
        .insert(MainMenuEntity)
        .insert(menu_button)
        .with_children(|parent| {
            parent.spawn(text_bundle).insert(MainMenuEntity);
        })
        .id()
}

pub fn setup_world_naming(mut commands: Commands) {
    commands.spawn(WorldNameInput {
        name: String::new(),
    });
}

fn create_load_button(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let button_style = Style {
        min_width: Val::Px(200.0),
        min_height: Val::Px(80.0),
        max_height: Val::Px(80.0),
        left: Val::Percent(-50.0),
        top: Val::Percent(50.0),
        border: UiRect::all(Val::Px(5.0)),
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button_load = ButtonBundle {
        style: button_style,
        z_index: ZIndex::Local(5),
        background_color: Color::NONE.into(),
        ..default()
    };

    let load_text = TextBundle::from_section(
        "LOAD",
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"),
            font_size: 80.0,
            color: Color::WHITE,
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style { ..default() });

    commands
        .spawn(button_load)
        .insert(MainMenuEntity)
        .insert(MenuButton::Load)
        .with_children(|parent| {
            parent.spawn(load_text).insert(MainMenuEntity);
        })
        .id()
}

fn create_new_button(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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
        background_color: Color::Rgba {
            red: (0.0),
            green: (0.0),
            blue: (0.0),
            alpha: (0.0),
        }
        .into(),
        transform: Transform { ..default() },
        ..default()
    };

    let new_text = TextBundle::from_section(
        "NEW",
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"),
            font_size: 80.0,
            color: Color::WHITE,
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style { ..default() });

    commands
        .spawn(button_new)
        .insert(MainMenuEntity)
        .insert(MenuButton::New)
        .with_children(|parent| {
            parent.spawn(new_text).insert(MainMenuEntity);
        })
        .id()
}

fn create_settings_button(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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
        background_color: Color::Rgba {
            red: (0.0),
            green: (0.0),
            blue: (0.0),
            alpha: (0.0),
        }
        .into(),
        transform: Transform { ..default() },
        ..default()
    };

    let settings_text = TextBundle::from_section(
        "SETTINGS",
        TextStyle {
            font: asset_server.load("Fonts/Retro Gaming.ttf"),
            font_size: 80.0,
            color: Color::WHITE,
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style {
        align_content: AlignContent::Center,
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

pub fn world_naming(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut world_name_input: Query<&mut WorldNameInput>,
    mut world_name: ResMut<WorldName>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Some(mut input) = world_name_input.iter_mut().next() {
        egui::SidePanel::right("Create World")
            .resizable(false)
            .default_width(400.0)
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Create World");
                    ui.separator();
                    ui.add_space(10.0);
                    ui.text_edit_singleline(&mut input.name);
                    ui.separator();
                    ui.add_space(20.0);
                    if ui.button("Create").clicked() {
                        world_name.0 = input.name.clone();
                        next_state.set(AppState::AssetLoading);
                    }
                });
            });
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}

#[derive(Resource, Default)]
pub struct SelectedWorld(pub Option<String>);

pub fn load_world_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut selected_world: ResMut<SelectedWorld>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    egui::SidePanel::right("load_world_panel")
        .resizable(false)
        .default_width(400.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Load World");
            ui.separator();
            ui.add_space(16.0);

            if let Ok(entries) = std::fs::read_dir("assets/Saves") {
                for entry in entries.flatten() {
                    if let Some(world_name) = entry
                        .file_name()
                        .to_str()
                        .and_then(|s| s.strip_suffix(".json"))
                    {
                        let selected = selected_world.0 == Some(world_name.to_string());
                        ui.selectable_label(selected, world_name)
                            .clicked()
                            .then(|| {
                                selected_world.0 = Some(world_name.to_string());
                            });
                        ui.add_space(8.0);
                    }
                }
            }

            ui.separator();
            ui.add_space(16.0);

            if ui.button("Load").clicked() && selected_world.0.is_some() {
                next_state.set(AppState::AssetLoading);
            }

            ui.add_space(8.0);

            if ui.button("Delete").clicked() {
                if let Some(world) = &selected_world.0 {
                    let file_path = format!("assets/Saves/{}.json", world);
                    if std::fs::remove_file(&file_path).is_ok() {
                        selected_world.0 = None;
                    }
                }
            }
        });

    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}
