use bevy_egui::*;
use bevy::prelude::*;
use bevy::window::Window;


pub fn ui_system(mut context: EguiContexts) {
    egui::Window::new("Hello").show(context.ctx_mut(), |ui| {
        // Create a horizontal layout for the hot bar
        ui.horizontal(|ui| {
            for i in 0..8 {
                if ui.button(format!("Slot {}", i)).clicked() {
                    // Handle slot click (e.g., mark as selected)
                }
            }
        });
    });
}

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub bottom: f32,
}

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}