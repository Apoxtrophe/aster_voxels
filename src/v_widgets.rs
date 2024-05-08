use bevy::{app::{App, Plugin, Startup, Update}, asset::AssetServer, ecs::{component::Component, entity::Entity, event::{EventReader, Events}, query::With, schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter}, system::{Commands, Query, Res, ResMut}}, hierarchy::BuildChildren, prelude::default, render::{color::Color, texture::Image, view::Visibility}, sprite::SpriteBundle, time::{Time, Timer, TimerMode}, transform::components::Transform, ui::{node_bundles::{ImageBundle, MaterialNodeBundle, NodeBundle}, AlignItems, BackgroundColor, JustifyContent, Node, PositionType, Style, UiImage, Val}};
use bevy_math::Vec3;

use crate::{a_loading::{SaveNotificationTimer, TextureHandles}, v_save::SaveEvent, AppState};

pub struct SaveNotificationPlugin;

impl Plugin for SaveNotificationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SaveNotificationTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_systems(OnEnter(AppState::InGame), setup_save_notification)
            .add_systems(Update, (update_save_notification).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct SaveNotification;

pub fn setup_save_notification(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
) {
    let image_handle = texture_handles.image_handles.get(4).unwrap_or_else(|| panic!("Texture handle not found"));

    commands.spawn(ImageBundle {
        style: Style {

            width: Val::Px(64.0),
            height: Val::Px(64.0),
            ..default()
        },
        image: UiImage::new(image_handle.clone()),
        visibility: Visibility::Hidden,
        ..Default::default()
    })
    .insert(SaveNotification);
}

 
pub fn update_save_notification(
    mut query: Query<&mut Visibility, With<SaveNotification>>,
    mut save_event_reader: EventReader<SaveEvent>,
    time: Res<Time>,
    mut timer: ResMut<SaveNotificationTimer>,
) {
    println!("{:?}", timer.0);

    if !save_event_reader.is_empty() {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Visible;
        }
        timer.0.reset();
    } else if timer.0.tick(time.delta()).just_finished() {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Hidden;
            println!("I tried");
        }
    }
}