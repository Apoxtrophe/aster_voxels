
use std::process::id;

use bevy::{animation::{AnimationClip, AnimationPlayer, RepeatAnimation}, asset::{AssetServer, Assets}, core_pipeline::core_2d::Camera2dBundle, ecs::{component::Component, entity::Entity, query::{Changed, With}, schedule::NextState, system::{Commands, Query, Res, ResMut}, world::Mut}, hierarchy::{BuildChildren, Children}, math::{Vec2, Vec3}, prelude::{Deref, DerefMut}, render::color::Color, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite}, text::{Text, TextAlignment, TextStyle}, time::{Time, Timer, TimerMode}, transform::components::{GlobalTransform, Transform}, ui::{node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle}, widget::Button, AlignContent, AlignItems, BackgroundColor, BorderColor, Display, Interaction, JustifyContent, JustifyItems, Overflow, PositionType, Style, UiRect, Val, ZIndex}, utils::default, window::{PrimaryWindow, Window, WindowResolution}};

use crate::{main, v_components::MainMenuEntity, v_config::{CONTINUE_BUTTON_HOVER, CONTINUE_BUTTON_OFF, CONTINUE_BUTTON_ON}, AppState};

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
    .with_text_alignment(TextAlignment::Center) // Align text to the center
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
    .with_text_alignment(TextAlignment::Center) // Align text to the center
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
    .with_text_alignment(TextAlignment::Center) // Align text to the center
    .with_style(Style {
        align_content: AlignContent::Center, //
        ..default()
    });


    let parent_entity = commands.spawn(parent).id();
    let child_main = commands.spawn(child_main).id();
    let load_button_entity = commands.spawn(button_load).id();
    let new_button_entity = commands.spawn(button_new).id();
    let settings_button_entity = commands.spawn(button_settings).id();
    let load_text_entity = commands.spawn(load_text).id();
    let new_text_entity = commands.spawn(new_text).id();
    let settings_text_entity = commands.spawn(settings_text).id();

    commands.entity(parent_entity).push_children(&[child_main, load_button_entity, new_button_entity, settings_button_entity]);
    commands.entity(load_button_entity).push_children(&[load_text_entity]);
    commands.entity(new_button_entity).push_children(&[new_text_entity]);
    commands.entity(settings_button_entity).push_children(&[settings_text_entity]);

}

pub fn main_menu_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&Children>, // Now optional, to account for buttons without text as children
        ),
        With<Button>, // Query entities with Button component
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (entity, interaction, mut color, mut border_color, children) in interaction_query.iter_mut() {   
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::RED;
                next_state.set(AppState::AssetLoading);
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
