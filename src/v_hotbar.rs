
use std::{f32::consts::PI, iter::Once};

use bevy::{prelude::*, render::color};

use crate::{a_loading::TextureHandles, v_config::{DESCRIPTOR_BOTTOM, DESCRIPTOR_COLOR, DESCRIPTOR_FADE_TIMER, DESCRIPTOR_FONT_SIZE, DESCRIPTOR_RIGHT, HOTBAR_ABOVE_BOTTOM, HOTBAR_BACKGROUND_COLOR, HOTBAR_BORDER_COLOR, HOTBAR_BORDER_SIZE, HOTBAR_ELEMENT_NUMBER, HOTBAR_SIZE, HOTBAR_SLOT_SIZE, HOTBAR_SPACING, SCREEN_HEIGHT, SCREEN_WIDTH}, v_selector::VoxelSelector};

pub fn hotbar_ui(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handle_texture = texture_handles.image_handles.get(3).expect("Texture handle not found");
    let texture_atlas = TextureAtlas::from_grid(handle_texture.clone(), Vec2::new(24.0, 24.0), 9, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let hotbar_size = HOTBAR_SIZE;
    let num_slots = HOTBAR_ELEMENT_NUMBER;

    let slot_size = HOTBAR_SLOT_SIZE * (hotbar_size * hotbar_size);
    let spacing = HOTBAR_SPACING * hotbar_size;

    let total_item_size = (slot_size * num_slots as f32) + (spacing * num_slots as f32); 
    let side_space = (SCREEN_WIDTH - total_item_size) / 2.0;

    let bottom_alignment = SCREEN_HEIGHT - slot_size;
    let above_bottom = HOTBAR_ABOVE_BOTTOM;
    
    let border_size = HOTBAR_BORDER_SIZE * (hotbar_size * hotbar_size);

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
            border_color: HOTBAR_BORDER_COLOR.into(),
            background_color: HOTBAR_BACKGROUND_COLOR.into(),
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
#[derive(Component)]
pub struct FadingText;

pub fn voxel_descriptor(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Welcome to Logica!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("Fonts/Retro Gaming.ttf"),
                font_size: DESCRIPTOR_FONT_SIZE,
                color: DESCRIPTOR_COLOR,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(DESCRIPTOR_BOTTOM),
            right: Val::Percent(DESCRIPTOR_RIGHT),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }),
    )).insert(FadingText);
}

#[derive(Resource)]
pub struct FadeTimer {
    pub timer: Timer,
    pub active: bool,
}

impl FadeTimer {
    pub fn new() -> Self {
        FadeTimer {
            timer: Timer::from_seconds(DESCRIPTOR_FADE_TIMER, TimerMode::Once),
            active: false,
        }
    }
}

pub fn timer_update_system(
    time: Res<Time>,
    mut countdown_timer: ResMut<FadeTimer>,
    mut query: Query<(&mut Text, &mut FadingText)>,
    voxel_selector: ResMut<VoxelSelector>,
) {
    if countdown_timer.active {
        let selected = Some(voxel_selector.current_voxel_type());

        for (mut text, mut fading_text) in query.iter_mut() {
            let mut timer = countdown_timer.timer.tick(time.delta()).percent();
            let alpha_text = (timer * (PI/2.0 )).cos() as f32;
            text.sections[0].style.color.set_a(alpha_text);
            text.sections[0].value = format!("{:?}", selected.unwrap());
        }
    }
}