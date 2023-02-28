use egui::{Context, RichText, Ui, Color32, Align, Layout, Direction, TextureHandle, ColorImage};
use egui::Direction::LeftToRight;
use metalboy::cpu::Cpu;
use metalboy::timer;
use super::common::*;

pub struct App {
    pub cpu: Cpu,
    pub old_tileset_vram: [u8; 0x1800],
    pub tileset_image: ColorImage,
}

impl App {
    pub fn new() -> Self {
        App {
            cpu: Cpu::new(),
            old_tileset_vram: [0; 0x1800],
            tileset_image: ColorImage::new([128, 192], Color32::BLACK),
        }
    }

    pub fn draw_windows(&mut self, egui_ctx: &Context) {
        self.show_state(egui_ctx);
        self.show_tileset(egui_ctx);
    }

    pub(crate) fn header(&mut self, text: &str, ui: &mut Ui) {
        ui.label(RichText::new(text).color(HEADER_COLOUR));
    }

    pub(crate) fn label_bold(&mut self, text: &str, ui: &mut Ui) {
        ui.label(RichText::new(text).color(BOLD_FONT_COLOUR));
    }
}
