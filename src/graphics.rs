use crate::mmu::Mmu;

const SCANLINE_RESET: i32 = 456;

pub struct Graphics {
    pub fb: [[[u8; 3]; 144]; 160],
    pub scanline_count: i32,
    pub lcd_enabled: bool
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            fb: [[[0xFF; 3]; 144]; 160],
            scanline_count: SCANLINE_RESET,
            lcd_enabled: true, // TODO: Should be false by default
        }
    }

    pub fn update(&mut self, mmu: &mut Mmu, cycles: usize) {
        if self.lcd_enabled {
            self.scanline_count -= cycles as i32; // FIXME: This is definitely bad code
        } else {
            return ();
        }

        if self.scanline_count <= 0 {
            self.scanline_count = SCANLINE_RESET;
            mmu.set(0xFF44, mmu.get(0xFF44) + 1);
            let current_line = mmu.get(0xFF44);

            // VBlank Period
            if current_line == 144 {
                // TODO: Request interrupt
            }

            // Reset back to scanline 0
            else if current_line > 153 {
                mmu.set(0xFF44, 0);
            }

            // Draw the scanline
            else if current_line < 144 {
                // TODO: Draw function
            }
        }
    }
}