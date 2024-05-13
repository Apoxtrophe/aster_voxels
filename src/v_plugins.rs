use std::time::Duration;
use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::BuildChildren,
    prelude::default,
    render::view::Visibility,
    sprite::{TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
    ui::{
        node_bundles::{AtlasImageBundle, ImageBundle, NodeBundle},
        AlignItems, FlexDirection, JustifyContent, Style, UiImage, Val,
    },
};
use bevy_egui::egui::epaint::image;
use bevy_math::Vec2;

use crate::{
    a_loading::{SaveNotificationTimer, TextureHandles},
    v_save::SaveEvent,
    v_simulation::MyTimer,
    AppState,
};

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SaveNotificationTimer(Timer::from_seconds(  2.0, TimerMode::Once,)))
        .add_systems(OnEnter(AppState::InGame), (
            setup_save_notification,
            setup_speed_widget, 
        ))
        .add_systems(
            Update, (
                update_save_notification,
                update_speed_widget,
            ).run_if(in_state(AppState::InGame)),
           
        ); 
    }
}

#[derive(Component)]
pub struct SaveNotification;

pub fn setup_save_notification(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    let image_handle = texture_handles.image_handles.get(4).expect("Texture handle not found");

    commands.spawn((ImageBundle {
            style: Style {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..default()
            },
            image: UiImage::new(image_handle.clone()),
            visibility: Visibility::Hidden,
            ..default()
        },
        SaveNotification,
    ));
}

pub fn update_save_notification(
    mut query: Query<&mut Visibility, With<SaveNotification>>,
    save_event_reader: EventReader<SaveEvent>,
    time: Res<Time>,
    mut timer: ResMut<SaveNotificationTimer>,
) {
    if !save_event_reader.is_empty() {
        for mut visibility in &mut query {
            *visibility = Visibility::Visible;
        }
        timer.0.reset();
    } else if timer.0.tick(time.delta()).just_finished() {
        for mut visibility in &mut query {
            *visibility = Visibility::Hidden;
        }
    }
}

// Speed Controller
#[derive(Resource)]
pub struct SpeedBar {
    pub speed_index: usize,
}

impl SpeedBar {
    pub fn new() -> Self {
        SpeedBar { speed_index: 1 }
    }
}

#[derive(Component)]
pub struct SpeedWidget;

pub fn update_speed_widget(
    speed_bar: ResMut<SpeedBar>,
    mut simulation_timer: ResMut<MyTimer>,
    mut query: Query<&mut TextureAtlas, With<SpeedWidget>>,
) {
    let index = speed_bar.speed_index - 1;

    let simulation_speed: u32;
    match index {
        0 => simulation_speed = 500000,
        1 => simulation_speed = 500,
        2 => simulation_speed = 100,
        3 => simulation_speed = 10,
        4 => simulation_speed = 1,
        _ => simulation_speed = 0,
    };

    simulation_timer.0.set_duration(Duration::from_millis(simulation_speed.into()));

    for mut texture_atlas in &mut query {
        texture_atlas.index = index;
    }
}

pub fn setup_speed_widget(
    texture_handles: Res<TextureHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let texture_handle = texture_handles.image_handles.get(5).unwrap_or_else(|| panic!("Texture handle not found"));
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32.0, 16.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((AtlasImageBundle {
        style: Style {
            width: Val::Px(160.),
            height: Val::Px(80.),
            ..default()
        },
        texture_atlas: TextureAtlas{layout: texture_atlas_handle,index: (0)},
        image: UiImage::new(texture_handle.clone()),
        ..default()
        },
        SpeedWidget,
    ));
}
