use log::trace;
use crate::execute::execute;
use super::registers::Registers;
use super::mmu::Mmu;

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub opcode: u8,
    pub advance_pc: i16,
    pub cycles: u16,
    pub cb_prefix: bool,
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
        self.opcode = self.mmu.get(self.reg.pc);
        execute(self);
        self.reg.pc = (self.reg.pc as i16 + self.advance_pc) as u16;
        self.advance_pc = 1;
        if self.mmu.bootrom_mapped && self.reg.pc >= 0x100 {
            trace!("End of bootrom emulation, unmapping bootrom");
            self.mmu.bootrom_mapped = false;
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
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn tick_advances_pc() {
        let mut cpu = Cpu::new();
        let initial_state = cpu.reg.pc;
        cpu.tick();
        assert_eq!(cpu.reg.pc, initial_state + 1);
    }
}