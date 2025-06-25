use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{EguiContextPass, EguiContexts},
    egui,
};

use crate::screens::Screen;

pub(crate) struct MainScreenPlugin;

#[derive(Default, Resource)]
struct MainScreenUiState {
    start_game_clicked: bool,
}

impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainScreenUiState>();
        app.add_systems(
            EguiContextPass,
            display_main_screen.run_if(in_state(Screen::Main)),
        );
        app.add_systems(
            Update,
            switch_to_gameplay_screen.run_if(in_state(Screen::Main)),
        );
    }
}

/// Shows the main screen.
fn display_main_screen(mut egui_ctxs: EguiContexts, mut ui_state: ResMut<MainScreenUiState>) {
    if let Some(ctx) = egui_ctxs.try_ctx_mut() {
        egui::Area::new(egui::Id::new("Main_Screen")).show(ctx, |ui| {
            if ui.button("Start Game").clicked() {
                ui_state.start_game_clicked = true;
            } else {
                ui_state.start_game_clicked = false;
            }

            // TODO: Add settings, new game, load game, etc
        });
    }
}

/// Switches to the gameplay screen.
fn switch_to_gameplay_screen(
    mut next_state: ResMut<NextState<Screen>>,
    ui_state: Res<MainScreenUiState>,
) {
    if ui_state.start_game_clicked {
        next_state.set(Screen::Gameplay);
    }
}
