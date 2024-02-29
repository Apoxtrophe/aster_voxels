
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{a_loading::TextureHandles, v_config::{DESCRIPTOR_BOTTOM, DESCRIPTOR_COLOR, DESCRIPTOR_FADE_TIMER, DESCRIPTOR_FONT_SIZE, DESCRIPTOR_RIGHT, HOTBAR_ABOVE_BOTTOM, HOTBAR_BACKGROUND_COLOR, HOTBAR_BORDER_COLOR, HOTBAR_BORDER_SIZE, HOTBAR_ELEMENT_NUMBER, HOTBAR_SLOT_SIZE, HOTBAR_SPACING, SCREEN_HEIGHT, SCREEN_WIDTH}, v_selector::VoxelSelector};

pub fn hotbar_ui(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>, 
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let handle_texture = texture_handles.image_handles.get(3).unwrap_or_else(|| panic!("Texture handle not found"));
    //let texture_atlas = TextureAtlasLayout::from_grid(handle_texture.clone(), Vec2::new(24.0, 24.0), 9, 1, None, None);
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 9, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let slot_size = HOTBAR_SLOT_SIZE; // Assuming HOTBAR_SIZE is always 1
    let spacing = HOTBAR_SPACING; // Same assumption as above

    let total_item_size = (slot_size + spacing) * HOTBAR_ELEMENT_NUMBER as f32; 
    let side_space = (SCREEN_WIDTH - total_item_size) / 2.0;

    for i in 0..HOTBAR_ELEMENT_NUMBER {
        commands.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(slot_size),
                height: Val::Px(slot_size),
                top: Val::Px(SCREEN_HEIGHT - slot_size - HOTBAR_ABOVE_BOTTOM),
                left: Val::Px((i as f32) * (slot_size + spacing) + side_space),
                border: UiRect::all(Val::Px(HOTBAR_BORDER_SIZE)),
                ..Default::default()
            },
            border_color: HOTBAR_BORDER_COLOR.into(),
            background_color: HOTBAR_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(AtlasImageBundle {  
                style: Style {
                    min_width: Val::Px(slot_size - HOTBAR_BORDER_SIZE * 2.0),
                    min_height: Val::Px(slot_size - HOTBAR_BORDER_SIZE * 2.0),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone().into(),
                image: UiImage {
                    texture: handle_texture.clone(),
                    ..Default::default()
                },

                ..default()
            });
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
        TextBundle::from_section(
            "Welcome to Logica!",
            TextStyle {
                font: asset_server.load("Fonts/Retro Gaming.ttf"),
                font_size: DESCRIPTOR_FONT_SIZE,
                color: DESCRIPTOR_COLOR,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(DESCRIPTOR_BOTTOM),
            right: Val::Percent(DESCRIPTOR_RIGHT),
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

        for (mut text, _fading_text) in query.iter_mut() {
            let timer = countdown_timer.timer.tick(time.delta()).fraction() as f32;
            let alpha_text = (timer * (PI/2.0 )).cos() as f32;
            text.sections[0].style.color.set_a(alpha_text);
            text.sections[0].value = format!("{:?}", selected.unwrap());
        }
    }
}