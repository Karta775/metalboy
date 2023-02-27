use egui::Context;
use metalboy::cpu::Cpu;

pub struct App {
    pub cpu: Cpu,
}

impl App {
    pub fn new() -> Self {
        App {
            cpu: Cpu::new(),
        }
    }

    pub fn show_controls(&mut self, egui_ctx: &Context) {
        egui::Window::new("Control").show(egui_ctx, |ui| {

        });
    }
}
