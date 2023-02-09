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
    Signed(i8),
    Unsigned(u8)
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
            mmu.set(0xFF44, mmu.get(0xFF44) + 1);
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
        }
    }

    pub fn draw_scanline(&mut self, mmu: &mut Mmu) {
        let control = mmu.get(0xFF40);

        if check_bit(control, 0) {
            self.render_tiles(mmu);
        }
        if check_bit(control, 1) {
            // TODO: Render sprites
        }
    }

    pub fn get_colour(&mut self, colour_no: u8, palette: u8) -> u32 {
        let left = check_bit(palette, (colour_no * 2) + 1) as u8;
        let right = check_bit(palette, colour_no * 2) as u8;
        let colour = (left << 1) | right;

        match colour {
            0 => 0xFFFFFF,
            1 => 0xCCCCCC,
            2 => 0x777777,
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
            y = mmu.get(0xFF44) - window_y;
        } else {
            bg_memory = if check_bit(control, 3) { 0x9C00 } else { 0x9800 };
            y = mmu.get(0xFF44) + scroll_y
        }


        // Draw the pixels for the current scanline
        let tile_row = (y as u16 / 8) * 32;
        for i in 0..160 {
            let mut x: u16 = (i as u16 + scroll_x as u16) as u16;
            if window_enabled && i > window_x {
                x = (i - window_x) as u16;
            }

            let tile_column: u16 = x as u16 / 8;
            let tile_address = bg_memory + tile_row + tile_column;
            let tile_no: TileNumber = if unsigned {
                Unsigned(mmu.get(tile_address))
            } else {
                Signed(mmu.get(tile_address) as i8)
            };

            let tile_location: u16 = match tile_no {
                Signed(n) => tile_data + (n * 16) as u16,
                Unsigned(n) => tile_data + (((n as u16) + 128) * 16) as u16
            };

            let line: u8 = (y % 8) * 2;
            let data_1 = mmu.get(tile_location + line as u16);
            let data_2 = mmu.get(tile_location + line as u16 + 1);

            let colour_bit = (((x as i16 % 8) - 7) * -1) as u8;
            let mut colour_no = check_bit(data_2, colour_bit) as u8;
            colour_no = (colour_no << 1) | (check_bit(data_1, colour_bit) as u8);

            let y = mmu.get(0xFF44);
            self.fb[i as usize][y as usize] = self.get_colour(colour_no, mmu.get(0xFF47));
        }
    }
}