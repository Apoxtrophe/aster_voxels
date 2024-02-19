
use bevy::{animation::{AnimationClip, AnimationPlayer, RepeatAnimation}, asset::{AssetServer, Assets}, core_pipeline::core_2d::Camera2dBundle, ecs::{component::Component, entity::Entity, query::{Changed, With}, schedule::NextState, system::{Commands, Query, Res, ResMut}, world::Mut}, hierarchy::{BuildChildren, Children}, math::{Vec2, Vec3}, prelude::{Deref, DerefMut}, render::color::Color, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite}, text::{Text, TextStyle}, time::{Time, Timer, TimerMode}, transform::components::{GlobalTransform, Transform}, ui::{node_bundles::{ButtonBundle, NodeBundle, TextBundle}, widget::Button, AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, Style, UiRect, Val}, utils::default, window::{PrimaryWindow, Window, WindowResolution}};

use crate::{main, v_components::MainMenuEntity, v_config::{CONTINUE_BUTTON_HOVER, CONTINUE_BUTTON_OFF, CONTINUE_BUTTON_ON}, AppState};

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {

    windows.single_mut().resolution = WindowResolution::new(1920.0, 1080.0);
    // UI CAMERA
    commands.spawn(Camera2dBundle::default()).insert(MainMenuEntity);
    
    let main_menu_handle = asset_server.load("UserInterface/main_splash.png");
    let texture_atlas = TextureAtlas::from_grid(main_menu_handle.clone(), Vec2::new(379.0, 216.0), 20, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {first: 1, last: 19};
    commands.spawn((SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(animation_indices.first),
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(5.0)),
        //global_transform: GlobalTransform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)
    )));
    



    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: CONTINUE_BUTTON_OFF.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )).insert(MainMenuEntity);
                }).insert(MainMenuEntity);
        }).insert(MainMenuEntity);
}

pub fn main_menu_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = CONTINUE_BUTTON_ON.into();
                border_color.0 = Color::RED;

                next_state.set(AppState::AssetLoading);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = CONTINUE_BUTTON_HOVER.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = CONTINUE_BUTTON_OFF.into();
                border_color.0 = Color::BLACK;
            }
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

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}