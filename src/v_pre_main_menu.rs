use bevy::ecs::{schedule::NextState, system::ResMut};

use crate::AppState;

pub fn pre_main_menu_cleanup(
    mut next_state: ResMut<NextState<AppState>>,
) {
    println!("Entering PreMainMenu");
    next_state.set(AppState::MainMenu);
}
