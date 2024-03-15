

use bevy::{app::Main, asset::{AssetServer, Assets}, core_pipeline::core_2d::Camera2dBundle, ecs::{component::Component, entity::Entity, query::With, schedule::NextState, system::{Commands, Query, Res, ResMut}}, hierarchy::{BuildChildren, Children}, render::color::Color, sprite::TextureAtlasLayout, text::{JustifyText, TextStyle}, transform::components::Transform, ui::{node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle}, widget::Button, AlignContent, BackgroundColor, BorderColor, Display, Interaction, JustifyContent, JustifyItems, Overflow, PositionType, Style, UiRect, Val, ZIndex}, utils::default, window::{PrimaryWindow, Window, WindowResolution}};

use crate::{v_components::MainMenuEntity, AppState};


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

    let parent_entity = commands.spawn(parent)
        .insert(MainMenuEntity)
        .id();

    let child_main = commands.spawn(child_main)
        .insert(MainMenuEntity)
        .id();

    let load_button_entity = commands.spawn(button_load)
        .insert(MainMenuEntity)
        .insert(MenuButton::Load)
        .with_children(|parent| {
            parent.spawn(load_text).insert(MainMenuEntity);
        })
        .id();

    let new_button_entity = commands.spawn(button_new)
        .insert(MainMenuEntity)
        .insert(MenuButton::New)
        .with_children(|parent| {
            parent.spawn(new_text).insert(MainMenuEntity);
        })
        .id();

    let settings_button_entity = commands.spawn(button_settings)
        .insert(MainMenuEntity)
        .insert(MenuButton::Settings)
        .with_children(|parent| {
            parent.spawn(settings_text).insert(MainMenuEntity);
        })
        .id();

    commands.entity(parent_entity)
        .push_children(&[child_main, load_button_entity, new_button_entity, settings_button_entity]);
}

pub fn main_menu_buttons(
    mut commands: Commands,
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
                match menu_button{
                    MenuButton::Load => {
                        next_state.set(AppState::AssetLoading);
                    }
                    MenuButton::New => {

                    }
                    MenuButton::Settings => {

                    }
                    
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
