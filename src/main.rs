use bevy::{diagnostic::*, prelude::*};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, egui};
use strum::IntoEnumIterator;

use crate::display::DisplayPlugin;
use crate::fighter::components::{ChangeStateEvent, Direction, Fighter, FighterState};
use crate::fighter::FighterPlugin;
use crate::stage::StagePlugin;

mod config;
mod display;
mod fighter;
mod sprite;
mod stage;

#[derive(Resource, Default, Copy, Clone, PartialEq, Debug)]
pub struct UiState {
    fighter: Fighter,
    fighter_state: FighterState,
    direction: Direction,
    flip_x: bool,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DisplayPlugin)
        .init_resource::<UiState>()
        .add_state::<GameState>()
        .add_event::<ChangeStateEvent>()
        .add_plugins((StagePlugin, FighterPlugin))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins((EguiPlugin, WorldInspectorPlugin::new()))
        .add_systems(Update, (inspector_game_state, inspector_ui))
        .run();
}

fn inspector_ui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut writer: EventWriter<ChangeStateEvent>,
) {
    egui::Window::new("State Machine").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut ui_state.fighter,
                Fighter::Ken,
                format!("{:?}", Fighter::Ken),
            );
            ui.radio_value(
                &mut ui_state.fighter,
                Fighter::Ryu,
                format!("{:?}", Fighter::Ryu),
            );
        });
        let selected_text = &ui_state.fighter_state;
        egui::ComboBox::from_label("State")
            .selected_text(format!("{selected_text:?}"))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                for value in FighterState::iter() {
                    ui.selectable_value(&mut ui_state.fighter_state, value, format!("{:?}", value));
                }
            });
        if ui.button("Change").clicked() {
            info!("Selected fighter: {:?}", ui_state.fighter);
            writer.send(ChangeStateEvent {
                fighter: ui_state.fighter,
                new_state: ui_state.fighter_state,
            });
        }
    });
}

fn inspector_game_state(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    egui::Window::new("Game State").show(contexts.ctx_mut(), |ui| {
        if ui.button("Start").clicked() {
            next_state.set(GameState::InGame);
        }
    });
}
