use crate::mmu::Mmu;
use crate::check_bit;
use crate::graphics::TileNumber::{Signed, Unsigned};

const SCANLINE_RESET: i32 = 456;
pub const LCD_CONTROL: u16 = 0xFF40;
pub const SCROLL_Y: u16 = 0xFF42;
pub const SCROLL_X: u16 = 0xFF43;
pub const WINDOW_Y: u16 = 0xFF4A;
pub const WINDOW_X: u16 = 0xFF4B;

pub struct Graphics {
    pub fb: [[u32; 144]; 160],
    pub scanline_count: i32,
}

pub enum TileNumber {
    Signed(i16),
    Unsigned(u16)
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            fb: [[0xFFFFFF; 144]; 160],
            scanline_count: SCANLINE_RESET,
        }
    }

    fn lcd_enabled(&self, mmu: &Mmu) -> bool {
        (mmu.get(LCD_CONTROL) >> 7) & 1 == 1
    }

    pub fn update(&mut self, mmu: &mut Mmu, cycles: usize) {
        if self.lcd_enabled(mmu) {
            self.scanline_count -= cycles as i32;
        } else {
            return ();
        }

        if self.scanline_count <= 0 {
            self.scanline_count = SCANLINE_RESET;
            let current_line = mmu.get(0xFF44);

            // VBlank Period
            if current_line == 144 {
                mmu.request_interrupt(0);
            }
            // Reset back to scanline 0
            else if current_line > 153 {
                mmu.set(0xFF44, 0);
            }
            // Draw the scanline
            else if current_line < 144 {
                self.draw_scanline(mmu);
            }

            mmu.set(0xFF44, mmu.get(0xFF44) + 1);
        }
    }

    pub fn draw_scanline(&mut self, mmu: &mut Mmu) {
        let control = mmu.get(0xFF40);

        if check_bit(control, 0) {
            self.render_tiles(mmu);
        }
        if check_bit(control, 1) {
            self.render_sprites(mmu);
        }
    }

    pub fn get_colour(&mut self, colour_no: u8, palette: u8) -> u32 {
        let left = check_bit(palette, (colour_no * 2) + 1) as u8;
        let right = check_bit(palette, colour_no * 2) as u8;
        let colour = (left << 1) | right;

        match colour {
            0 => 0x8bac0f,
            1 => 0x306230,
            2 => 0x0f380f,
            3 => 0x000000,
            _ => panic!("No such colour: {}", colour)
        }
    }

    fn render_tiles(&mut self, mmu: &mut Mmu) {
        let control = mmu.get(LCD_CONTROL);
        let tile_data: u16;
        let bg_memory: u16;

        // Get boundaries
        let scroll_y = mmu.get(SCROLL_Y);
        let scroll_x = mmu.get(SCROLL_X);
        let window_y = mmu.get(WINDOW_Y);
        let window_x = mmu.get(WINDOW_X).wrapping_sub(7);

        // Check if the window is enabled
        let window_enabled = check_bit(control, 5) && window_y <= mmu.get(0xFF44);

        // Set tile data location & sign
        let unsigned = check_bit(control, 4);
        tile_data = if unsigned { 0x8000 } else { 0x8800 };

        // Set window/background memory location & tile pos
        let y: u8;
        if window_enabled {
            bg_memory = if check_bit(control, 6) { 0x9C00 } else { 0x9800 };
            y = mmu.get(0xFF44).wrapping_sub(window_y);
        } else {
            bg_memory = if check_bit(control, 3) { 0x9C00 } else { 0x9800 };
            y = mmu.get(0xFF44).wrapping_add(scroll_y);
        }

        // Draw the pixels for the current scanline
        let tile_row: u16 = (y / 8) as u16 * 32;
        for i in 0u8..160 {
            let mut x: u8 = i.wrapping_add(scroll_x);
            if window_enabled && i >= window_x {
                x = i - window_x;
            }

            let tile_column = (x / 8) as u16;
            let tile_address = bg_memory + tile_row + tile_column;
            let tile_no: TileNumber = if unsigned {
                Unsigned(mmu.get(tile_address) as u16)
            } else {
                Signed(mmu.get(tile_address) as i16)
            };

            let tile_location: u16 = match tile_no {
                Unsigned(n) => tile_data + (n.wrapping_mul(16)) as u16,
                Signed(n) => (tile_data + (((n as u16) + 128) * 16)) as u16
            };

            let line: u8 = (y % 8) * 2;
            let data_1 = mmu.get(tile_location + line as u16);
            let data_2 = mmu.get(tile_location + line as u16 + 1);

            let colour_bit = (((x % 8) as i8 - 7) * -1) as u8;
            let mut colour_no = check_bit(data_2, colour_bit) as u8;
            colour_no = (colour_no << 1) | (check_bit(data_1, colour_bit) as u8);

            let y = mmu.get(0xFF44);
            if y < 144 {
                let colour = self.get_colour(colour_no, mmu.get(0xFF47));
                self.fb[i as usize][y as usize] = colour;
            }
        }
    }

    fn render_sprites(&mut self, mmu: &mut Mmu) {
        let control = mmu.get(LCD_CONTROL);
        let tall_sprite = check_bit(control, 2);
        let y_size = if tall_sprite { 16 } else { 8 };

        for sprite in 0..40 {
            let index = sprite * 4;
            let y = mmu.get(0xFE00 + index).wrapping_sub(16);
            let x = mmu.get(0xFE00 + index + 1).wrapping_sub(8);
            let tile_location = mmu.get(0xFE00 + index + 2);
            let attributes = mmu.get(0xFE00 + index + 3);

            let y_flip = check_bit(attributes, 6);
            let x_flip = check_bit(attributes, 5);

            let scanline = mmu.get(0xFF44);

            if scanline >= y && scanline < y + y_size {
                let mut line = (scanline as i16) - y as i16;

                // Flipped on the y axis
                if y_flip {
                    line = (line - y_size as i16) * -1;
                }
                line *= 2;

                let tile_data_address = line as u16 + 0x8000 + (tile_location as u16 * 16);
                let data_1 = mmu.get(tile_data_address);
                let data_2 = mmu.get(tile_data_address + 1);

                for sprite_pixel in (0..8).rev() {
                    let mut colour_bit = sprite_pixel;
                    if x_flip {
                        colour_bit = ((colour_bit as i16 - 7) * -1) as u8;
                    }

                    let mut colour_no = check_bit(data_2, colour_bit) as u8;
                    colour_no = (colour_no << 1) | (check_bit(data_1, colour_bit) as u8);
                    let palette_select = check_bit(attributes, 4) as u16;
                    let palette = mmu.get(0xFF48 + palette_select);
                    let colour = self.get_colour(colour_no, palette);

                    // White should be skipped due to transparency
                    if colour == 0x8bac0f {
                        continue;
                    }

                    let x_pixel: i16 = (0 - sprite_pixel as i16) + 7;
                    let pixel = x_pixel + x as i16;

                    // Draw the pixel to the framebuffer
                    self.fb[pixel as usize][scanline as usize] = colour;
                }
            }
        }
    }
}