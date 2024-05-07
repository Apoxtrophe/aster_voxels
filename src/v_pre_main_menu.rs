use bevy::ecs::{entity::Entity, query::With, schedule::NextState, system::{Commands, Query, ResMut}};

use crate::{v_components::MainCamera, AppState};

pub fn pre_main_menu_cleanup(
    mut commands: Commands,

    mut next_state: ResMut<NextState<AppState>>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    println!("Entering PreMainMenu");
    next_state.set(AppState::MainMenu);
}
