use bevy::{asset::AssetServer, ecs::{component::Component, system::{Commands, Res}}, hierarchy::BuildChildren, prelude::default, render::{color::Color, texture::Image, view::Visibility}, sprite::SpriteBundle, transform::components::Transform, ui::{node_bundles::{ImageBundle, MaterialNodeBundle, NodeBundle}, AlignItems, BackgroundColor, JustifyContent, Node, PositionType, Style, UiImage, Val}};
use bevy_math::Vec3;

use crate::a_loading::TextureHandles;

#[derive(Component)]
struct FloppyDiskIcon;

pub fn game_widgets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>, 
) {
    let image_handle = texture_handles.image_handles.get(4).unwrap_or_else(|| panic!("Texture handle not found"));
    // Floppy Disk Widgets
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(20.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                
                image: UiImage::new(image_handle.clone()),
                ..default()
            });
        });
}