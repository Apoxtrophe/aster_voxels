
use bevy::{prelude::*, render::color};

use crate::{a_loading::TextureHandles, v_config::{SCREEN_WIDTH, SCREEN_HEIGHT}};

pub fn hotbar_ui(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
    
    let border_size = 10.0 * (hotbar_size * hotbar_size);

    for i in 0..num_slots {

        let _ = commands.spawn(ButtonBundle {
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
                        right: Val::Px(border_size /2.0),
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

// Hotbar border is updated in v_player2.rs

// Little cute text that shows the current voxel type
#[derive(Resource)]
pub struct TextFadeTimer(pub Timer);

pub fn voxel_descriptor(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fira_mono.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
    ));
}