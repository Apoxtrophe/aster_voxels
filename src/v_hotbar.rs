use std::num;

use bevy::{prelude::*, math::vec2};
use bevy_egui::{EguiContext, egui::{self, ImageSource}, EguiContexts, EguiUserTextures};
use rand::seq::index;

use crate::{a_loading::TextureHandles, v_config::{SCREEN_WIDTH, SCREEN_HEIGHT}};

struct Hotbar;

pub fn hotbar_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas_images: Query<&mut UiTextureAtlasImage>,
) {
    let handle_texture = texture_handles.image_handles.get(3).expect("Texture handle not found");
    let texture_atlas = TextureAtlas::from_grid(handle_texture.clone(), Vec2::new(24.0, 24.0), 9, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let hotbar_size = 1.0;
    let num_slots = 9;

    let slot_size = 96.0 * hotbar_size;
    let spacing = 5.0 * hotbar_size;


    let total_item_size = (slot_size * num_slots as f32 * hotbar_size) + (spacing * num_slots as f32); 
    let side_space = (SCREEN_WIDTH - total_item_size) / 2.0;

    let bottom_alignment = SCREEN_HEIGHT - (slot_size * hotbar_size);
    let above_bottom = 10.0;
    
    for i in 0..num_slots {

        let item = commands.spawn(ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Px(slot_size * hotbar_size),
                height: Val::Px(slot_size * hotbar_size),

                top: Val::Px(bottom_alignment - above_bottom),
                left: Val::Px(((slot_size * i as f32 * hotbar_size) + spacing * i as f32) + side_space),
                ..Default::default()
            },
            //background_color: Color::BLACK.into(),
            
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                AtlasImageBundle {
                    style: Style {
                        min_width: Val::Px(slot_size * hotbar_size),
                        min_height: Val::Px(slot_size * hotbar_size),
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle.clone(),
                    texture_atlas_image: UiTextureAtlasImage { index: (i), flip_x: (false), flip_y: (false) },

                    //background_color: Color::WHITE.into(),

                    ..Default::default()
                },
                Interaction::default(),
                
            ));
        });
    }
}