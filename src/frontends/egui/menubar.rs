use std::ops::MulAssign;
use egui::{Align, Color32, ColorImage, Context, Direction, Image, Layout, menu, Pos2, TextureFilter, TextureHandle, TextureOptions};
use egui::panel::TopBottomSide;
use log::trace;
use metalboy::timer;
use crate::app::App;

impl App {
    pub fn show_menubar(&mut self, egui_ctx: &Context) {
        egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load ROM").clicked() {

                    }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_state_view, "System state");
                    ui.checkbox(&mut self.show_tileset_view, "Tileset");
                    ui.checkbox(&mut self.show_log_view, "Logs");
                    ui.checkbox(&mut self.show_control_view, "Control");
                    ui.checkbox(&mut self.show_mem_editor, "Memory editor");
                });
            });
        });
    }
}