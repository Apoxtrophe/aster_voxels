use bevy::{core::Name, ecs::{entity::Entity, query::With, schedule::NextState, system::{Commands, Local, Query, Res, ResMut}}, hierarchy::DespawnRecursiveExt, input::{keyboard::KeyCode, ButtonInput}};
use bevy_egui::{egui::{self, Color32}, EguiContexts};

use crate::{v_components::MainCamera, AppState};

pub fn in_game_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_visible: Local<bool>,

    // Despawn ish
    commands: Commands,
    entities: Query<(Entity, Option<&Name>)>,

    camera_query: Query<Entity, With<MainCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        *menu_visible = !*menu_visible;
    }

    if *menu_visible {
        

        egui::SidePanel::left("in_game_menu_panel")
            .resizable(false)
            .default_width(400.0)
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading(egui::RichText::new("In-Game Menu").color(Color32::WHITE).size(48.0))
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
                        despawn_all(commands, entities, camera_query);
                        next_state.set(AppState::MainMenu);
                        *menu_visible = false;
                    }
                });
            });
    }
}

pub fn despawn_all(
    mut commands: Commands,
    entities: Query<(Entity, Option<&Name>)>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn_recursive();
    }

    for (entity, _) in entities.iter().skip(1) {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}