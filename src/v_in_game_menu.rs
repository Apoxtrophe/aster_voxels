use bevy::{
    core::Name,
    ecs::{
        entity::Entity,
        query::With,
        schedule::NextState,
        system::{Commands, Local, Query, Res, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    input::{keyboard::KeyCode, ButtonInput},
};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};
use crate::{v_components::MainCamera, AppState};

pub fn in_game_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_visible: Local<bool>,
    commands: Commands,
    entities: Query<(Entity, Option<&Name>)>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    toggle_menu_visibility(keyboard_input, &mut menu_visible);

    if *menu_visible {
        show_in_game_menu(
            contexts,
            commands,
            entities,
            camera_query,
            &mut next_state,
            menu_visible,
        );
    }
}

fn toggle_menu_visibility(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    menu_visible: &mut Local<bool>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        **menu_visible = !**menu_visible;
    }
}

fn show_in_game_menu(
    mut contexts: EguiContexts,
    commands: Commands,
    entities: Query<(Entity, Option<&Name>)>,
    camera_query: Query<Entity, With<MainCamera>>,
    next_state: &mut ResMut<NextState<AppState>>,
    mut menu_visible: Local<bool>,
) {
    egui::SidePanel::left("in_game_menu_panel")
        .resizable(false)
        .default_width(400.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(
                    egui::RichText::new("In-Game Menu")
                        .color(Color32::WHITE)
                        .size(48.0),
                )
            });
            ui.separator();
            ui.add_space(800.0);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .button(
                        egui::RichText::new("Exit to Main Menu")
                            .heading()
                            .color(Color32::WHITE)
                            .size(36.0),
                    )
                    .clicked()
                {
                    next_state.set(AppState::PreMainMenu);
                    *menu_visible = false;
                }
            });
        });
}