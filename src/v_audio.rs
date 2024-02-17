use bevy::{asset::AssetServer, audio::{AudioBundle, PlaybackMode, PlaybackSettings}, ecs::{component::Component, entity::Entity, system::{Commands, Res, Resource}}};

use crate::a_loading::AudioAssets;



#[derive(Component)]
pub struct RelayLoop;

pub fn setup_relay_audio (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<AudioAssets>,
) {
    let loop_sound_handle = audio.click_sound.clone();

    let audio_loop_entity = commands.spawn(AudioBundle {
        source: loop_sound_handle.clone(),
        settings: PlaybackSettings{
            mode: PlaybackMode::Loop,
            speed: 10.0,
            ..Default::default()
        },
    }).insert(RelayLoop).id();
}