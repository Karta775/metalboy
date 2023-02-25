use crate::check_bit;
use crate::mmu::Mmu;

pub const DIV_INC: usize = 16384;
pub const DIV: u16 = 0xFF04;  // Divider register -- Increments at a rate of DIV_INC Hz
pub const TIMA: u16 = 0xFF05; // Timer counter    -- Increments based on TAC frequency
pub const TMA: u16 = 0xFF06;  // Timer modulo     -- Reset value for TIMA
pub const TAC: u16 = 0xFF07;  // Timer control    -- Enable & frequency of incrementation
pub const TIMER_INTERRUPT_ID: u8 = 2;

pub struct Timer {
    div_cycles: usize,
    tima_cycles: usize,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div_cycles: 0,
            tima_cycles: 0,
        }
    }

    fn tac_enabled(&self, control: u8) -> bool {
        check_bit(control, 2)
    }

    fn tac_frequency(&self, control: u8) -> usize {
        let select = control & 0b0011;
        match select {
            0b00 => 4096,
            0b01 => 262144,
            0b10 => 65536,
            0b11 => 16384,
            _ => panic!("This is supposed to be unreachable"),
        }
    }

    pub fn update(&mut self, mmu: &mut Mmu, cycles: usize) {
        // TODO: Reset (DIV) to 0 when (DIV) is written to by an instruction

        // Update DIV
        self.div_cycles += cycles;
        if self.div_cycles > DIV_INC {
            self.div_cycles = 0;
            let div = &mut mmu.memory[DIV as usize - 0x8000];
            *div = u8::wrapping_add(*div, 1);
        }

        // Update TIMA
        let control = mmu.get(TAC);
        if self.tac_enabled(control) {
            self.tima_cycles += cycles;
            if self.tima_cycles > self.tac_frequency(control) {
                self.tima_cycles = 0;
                let tima = mmu.get(TIMA);

                // Request an interrupt if TIMA overflows
                if tima == 0xFF {
                    let tma = mmu.get(TMA);
                    mmu.set(TIMA, tma); // Reset TIMA to TMA value
                    mmu.request_interrupt(TIMER_INTERRUPT_ID);
                } else {
                    mmu.set(TIMA, tima + 1);
                }
            }
        }
    }
}