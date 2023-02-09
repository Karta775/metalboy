use log::warn;
use crate::execute::execute;
use super::registers::Registers;
use super::mmu::Mmu;
use crate::bytes_from;

pub enum Interrupt {
    VBlank = 0x40,
    LCD = 0x48,
    TIMER = 0x50,
    JOYPAD = 0x60
}

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub opcode: u8,
    pub advance_pc: i16,
    pub cycles: usize,
    pub cb_prefix: bool,
    pub ime: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
            mmu: Mmu::new(),
            opcode: 0x00,
            advance_pc: 1,
            cycles: 0,
            cb_prefix: false,
            ime: true
        }
    }

    pub fn reset(&mut self) {
        self.reg.reset();
        self.mmu.reset();
        self.opcode = 0x00;
        self.advance_pc = 1;
        self.cycles = 0;
    }

    pub fn tick(&mut self) {
        self.cycles = 0;
        self.opcode = self.mmu.get(self.reg.pc);
        execute(self);
        self.reg.pc = (self.reg.pc as i16 + self.advance_pc) as u16;
        self.advance_pc = 1;
        if self.mmu.bootrom_mapped && self.reg.pc >= 0x100 {
            warn!("End of bootrom emulation, unmapping bootrom");
            self.mmu.bootrom_mapped = false;
            assert_eq!(self.reg.a, 0x01);
            // assert_eq!(self.reg.f.as_u8(), 0xB0);
            self.reg.f.set_from_u8(0xB0);
            assert_eq!(self.reg.b, 0);
            assert_eq!(self.reg.c, 0x13);
            // assert_eq!(self.reg.d, 0x13);
            self.reg.d = 0x13;
            assert_eq!(self.reg.e, 0xd8);
            assert_eq!(self.reg.h, 0x01);
            assert_eq!(self.reg.l, 0x4d);
            assert_eq!(self.reg.pc, 0x100);
        }
    }

    pub fn execute(&mut self) {
        let mut _bytes: usize = 1;
        let mut _advance_pc: bool = true;
        // self.mmu.get(self.reg.pc);
    }

    pub fn get_op(&self, offset: u16) -> u8 {
        self.mmu.get(self.reg.pc + offset)
    }

    pub fn push_word(&mut self, word: u16) {
        let (left, right) = bytes_from(word);
        self.mmu.set(self.reg.sp - 1, left);
        self.mmu.set(self.reg.sp - 2, right);
        self.reg.sp -= 2;
    }

    pub fn generate_interrupt(&mut self, id: u8, interrupt_flag: u8) {
        // FIXME: Set interrupt master switch to false
        let cleared = interrupt_flag & (0b1111_1111 ^ (1 << id));
        self.mmu.set(0xFF0F, cleared);

        self.push_word(self.reg.pc);
        match id {
            0 => self.reg.pc = Interrupt::VBlank as u16,
            1 => self.reg.pc = Interrupt::LCD as u16,
            2 => self.reg.pc = Interrupt::TIMER as u16,
            4 => self.reg.pc = Interrupt::JOYPAD as u16,
            _ => ()
        }
    }

    pub fn generate_interrupts(&mut self) {
        if true { // FIXME: Change to check if interrupt master switch is enabled
            let interrupt_flag = self.mmu.get(0xFF0F);
            let interrupt_enable = self.mmu.get(0xFFFF);

            if interrupt_flag > 0 { // Any interrupts are set
                for i in 0..5 {
                    if ((interrupt_flag & interrupt_enable) >> i) & 1 == 1 { // Interrupt 'i' is set
                        self.generate_interrupt(i, interrupt_flag);
                        break; // FIXME: Is this the correct behaviour?
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn generate_interrupts_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.set(0xFF0F, 0b0000_1101);
        cpu.mmu.set(0xFFFF, 0b0001_0111);
        cpu.generate_interrupts();
        assert_eq!(cpu.mmu.get(0xFF0F), 0b0000_1100);
    }

    #[test]
    fn generate_interrupt_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.set(0xFF0F, 0b0000_0110);
        cpu.mmu.set(0xFFFF, 0b0001_1111);
        assert_eq!(cpu.reg.sp, 0xFFFE);
        cpu.generate_interrupt(1, cpu.mmu.get(0xFF0F));
        assert_eq!(cpu.mmu.get(0xFF0F), 0b0000_0100);
        assert_eq!(cpu.reg.pc, 0x48);
        assert_eq!(cpu.reg.sp, 0xFFFE - 2);
    }

    #[test]
    fn tick_advances_pc() {
        let mut cpu = Cpu::new();
        let initial_state = cpu.reg.pc;
        cpu.tick();
        assert_eq!(cpu.reg.pc, initial_state + 1);
    }
}