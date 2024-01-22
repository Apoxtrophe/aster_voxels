use std::num;

use bevy::{prelude::*, math::vec2, render::color};
use bevy_egui::{EguiContext, egui::{self, ImageSource, color_picker::Alpha}, EguiContexts, EguiUserTextures};
use rand::seq::index;

use crate::{a_loading::TextureHandles, v_config::{SCREEN_WIDTH, SCREEN_HEIGHT}, v_selector::VoxelSelector};

struct HotbarSlot {
    index: usize,
}

pub fn hotbar_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas_images: Query<&mut UiTextureAtlasImage>,
    voxel_selector: Res<VoxelSelector>,
) {
    let handle_texture = texture_handles.image_handles.get(3).expect("Texture handle not found");
    let texture_atlas = TextureAtlas::from_grid(handle_texture.clone(), Vec2::new(24.0, 24.0), 9, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let hotbar_size = 1.0;
    let num_slots = 9;

    let slot_size = 96.0 * (hotbar_size * hotbar_size);
    let spacing = 5.0 * hotbar_size;


    let total_item_size = (slot_size * num_slots as f32) + (spacing * num_slots as f32); 
    let side_space = (SCREEN_WIDTH - total_item_size) / 2.0;

    let bottom_alignment = SCREEN_HEIGHT - slot_size;
    let above_bottom = 10.0;
    
    let border_size = 10.0;

    for i in 0..num_slots {

        let item = commands.spawn(ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Px(slot_size),
                height: Val::Px(slot_size),
                top: Val::Px(bottom_alignment - above_bottom),
                
                left: Val::Px(((slot_size * i as f32) + spacing * i as f32) + side_space),

                border: UiRect::all(Val::Px(border_size)),
                //margin: UiRect::all(Val::Px(20.)),
                ..Default::default()
            },
            border_color: color::Color::WHITE.into(),
            background_color: BackgroundColor(Color::rgba(0.8, 0.8, 0.8, 0.3)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                AtlasImageBundle {  
                    style: Style {
                        min_width: Val::Px(slot_size - border_size),
                        min_height: Val::Px(slot_size - border_size),
                        right: Val::Px(5.),
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle.clone(),
                    texture_atlas_image: UiTextureAtlasImage { index: (i), flip_x: (false), flip_y: (false) },

                    ..Default::default()
                },
                Interaction::default(),
                
            ));
        });
    }
}

pub fn update_hotbar_selection(
    mut query: Query<&mut BorderColor>,
    voxel_selector: Res<VoxelSelector>,
) {
    for (i, mut border_color) in query.iter_mut().enumerate() {
        if i == voxel_selector.current_index {
            border_color.0 = color::Color::GOLD.into();
        } else {
            border_color.0 = color::Color::DARK_GRAY.into();
        }
    }
}