use std::ops::MulAssign;
use egui::{Align, Color32, ColorImage, Context, Direction, Image, Layout, Pos2, TextureFilter, TextureHandle, TextureOptions};
use log::trace;
use metalboy::timer;
use crate::app::App;

impl App {
    pub fn show_tileset(&mut self, egui_ctx: &Context) {
        egui::Window::new("Tileset").default_pos(Pos2::new(730., 15.))
            .show(egui_ctx, |ui| {
            let image = self.render().unwrap_or(self.tileset_image.clone());
            let texture = ui.ctx().load_texture(
                "tileset",
                image,
                TextureOptions::NEAREST,
            );
            let mut size = texture.size_vec2();
            size.mul_assign(1.6);
            ui.image(&texture, size);
        });
    }

    fn render(&mut self) -> Option<ColorImage> {
        let vram = &self.cpu.mmu.memory[0x8000 % 0x8000..0x9800 % 0x8000];
        // Return if VRAM hasn't changed since the last run
        if vram == self.old_tileset_vram {
            return None;
        } else {
            self.old_tileset_vram = <[u8; 6144]>::try_from(vram).unwrap();
        }
        trace!("[app/tileset] Rendering a new tileset image");

        let mut image = ColorImage::new([128, 192], Default::default());
        let palette = self.cpu.mmu.get(0xFF47);

        for tile_no in 0..384 {
            // Tiles are 16-bytes in length, tile 0 is at 0x8000, tile 1 is at 0x8010, etc.
            let tile_address = 0x8000 + (tile_no * 0x10);
            let tile_data = &vram[tile_address % 0x8000 .. (tile_address + 0x10) % 0x8000];

            for (tile_y, line) in tile_data.chunks(2).enumerate() {
                let (d1, d2) = (line[0], line[1]);

                for i in 0..8 {
                    let bit = ((i as i8 - 7) * -1) as u8;
                    let mut colour_no = check_bit(d2, bit) as u8;
                    colour_no = (colour_no << 1) | (check_bit(d1, bit) as u8);
                    let colour = get_colour(colour_no, palette);
                    let x = (((tile_no * 8) + i as usize) % 128);
                    let y = (((tile_no * 8) / 128) * 8) + tile_y as usize;
                    let pixel_idx = x + (y * 128);
                    image.pixels[pixel_idx] = colour;
                }
            }
        }

        self.tileset_image = image.clone();
        return Some(image);
    }
}

fn get_colour(colour_no: u8, palette: u8) -> Color32 {
    let left = check_bit(palette, (colour_no * 2) + 1) as u8;
    let right = check_bit(palette, colour_no * 2) as u8;
    let colour = (left << 1) | right;

    match colour {
        0 => Color32::from_rgb(0x8B, 0xAC, 0x0F),
        1 => Color32::from_rgb(0x30, 0x62, 0x30),
        2 => Color32::from_rgb(0x0F, 0x38, 0x0F),
        3 => Color32::from_rgb(0x00, 0x00, 0x00),
        _ => panic!("No such colour: {}", colour)
    }
}

fn check_bit(byte: u8, bit: u8) -> bool {
    (byte >> bit) & 1 == 1
}
