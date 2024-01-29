use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::{v_components::Sun, v_config::{SUN_ANGLE, SUN_INTENSITY, ENABLE_DAY_NIGHT_CYCLE, DAY_LENGTH}};



#[derive(Resource)]
pub struct SunDirection {
    pub sun_direction: f32,
}

impl SunDirection {
    pub fn new() -> Self {
        SunDirection {
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
    timer.0.tick(time.delta());
    
    if timer.0.finished() && ENABLE_DAY_NIGHT_CYCLE{
        direction.sun_direction += DAY_LENGTH;
    }

    let sun_direction = direction.sun_direction;
    atmosphere.sun_position = Vec3::new(0., sun_direction.sin(), sun_direction.cos());

    if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
        light_trans.rotation = Quat::from_rotation_x(-sun_direction);
        directional.illuminance = sun_direction.sin().max(0.0).powf(2.0) * SUN_INTENSITY;
    }
}