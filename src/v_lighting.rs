use crate::{
    v_components::Sun,
    v_config::{SUN_ANGLE, SUN_DAY_LENGTH, SUN_DAY_NIGHT, SUN_INTENSITY},
};
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

#[derive(Resource)]
pub struct SunDirection {
    pub sun_direction: f32,
}

impl SunDirection {
    pub fn new() -> Self {
        Self {
            sun_direction: SUN_ANGLE,
        }
    }
}

#[derive(Resource)]
pub struct CycleTimer(pub Timer);

pub fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut direction: ResMut<SunDirection>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    update_sun_direction(&mut direction, &mut timer, time);

    let sun_direction = direction.sun_direction;
    update_atmosphere(&mut atmosphere, sun_direction);
    update_sun_transform_and_light(&mut query, sun_direction);
}

fn update_sun_direction(
    direction: &mut ResMut<SunDirection>,
    timer: &mut ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() && SUN_DAY_NIGHT {
        direction.sun_direction += SUN_DAY_LENGTH;
    }
}

fn update_atmosphere(atmosphere: &mut AtmosphereMut<Nishita>, sun_direction: f32) {
    atmosphere.sun_position = Vec3::new(0., sun_direction.sin(), sun_direction.cos());
}

fn update_sun_transform_and_light(
    query: &mut Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    sun_direction: f32,
) {
    if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
        light_trans.rotation = Quat::from_rotation_x(-sun_direction);
        directional.illuminance = sun_direction.sin().max(0.0).powf(2.0) * SUN_INTENSITY;
    }
}
