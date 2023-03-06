use std::ops::MulAssign;
use egui::{Align, Color32, ColorImage, Context, Direction, Image, Layout, Pos2, TextureFilter, TextureHandle, TextureOptions};
use log::trace;
use metalboy::timer;
use crate::app::App;

impl App {
    pub fn show_gameboy_view(&mut self, egui_ctx: &Context) {
        egui::CentralPanel::default().show(egui_ctx, |ui| {
            let image = self.fb_to_image();
            let texture = ui.ctx().load_texture(
                "tileset",
                image,
                TextureOptions::NEAREST,
            );
            let mut size = texture.size_vec2();
            size.mul_assign(2.5);
            ui.image(&texture, size);
        });
    }

    fn fb_to_image(&self) -> ColorImage {
        let mut image = ColorImage::new([160, 144], Default::default());
        for i in 0..(160 * 144) {
            let col = i % 160;
            let row = i / 160;
            let rgb = self.graphics.fb[col][row];
            let r = (rgb & 0xFF0000) >> 16;
            let g = (rgb & 0x00FF00) >> 8;
            let b =  rgb & 0x0000FF;
            image.pixels[i] = Color32::from_rgb(r as u8, g as u8, b as u8);
        }
        image
    }
}

