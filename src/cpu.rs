use super::registers::Registers;
use super::mmu::Mmu;
use super::decode::decode;

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub opcode: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
            mmu: Mmu::new(),
            opcode: 0x00,
        }
    }

    pub fn reset(&mut self) {
        self.reg.reset();
        self.mmu.reset();
    }

    pub fn tick(&mut self) {
        // Debugging
        // if self.reg.pc % 80 == 0 {
        //     println!();
        // }
        // print!("{:02x} ", self.mmu.get(self.reg.pc));

        self.opcode = self.mmu.get(self.reg.pc);
        decode(self);
        self.reg.pc += 2;
        if self.reg.pc > 0x104 {
            panic!();
        }
    }

    pub fn execute(&mut self) {
        let mut _bytes: usize = 1;
        let mut _advance_pc: bool = true;

        // self.mmu.get(self.reg.pc);
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
        assert_eq!(cpu.reg.pc, initial_state + 2);
    }
}