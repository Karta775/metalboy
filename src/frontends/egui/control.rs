use std::ops::MulAssign;
use egui::{Align, Color32, ColorImage, Context, Direction, Image, Layout, Pos2, TextureFilter, TextureHandle, TextureOptions};
use log::trace;
use metalboy::timer;
use crate::app::App;

impl App {
    pub fn show_control(&mut self, egui_ctx: &Context) {
        egui::Window::new("Control").show(egui_ctx, |ui| {
            self.header("Execution", ui);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Toggle").clicked() {
                    self.pause_execution = !self.pause_execution;
                }
                if ui.button("Step").clicked() {
                    self.step = true;
                }
            });
        });
    }
}