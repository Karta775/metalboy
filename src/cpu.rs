use super::registers::Registers;
use super::mmu::Mmu;

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
            mmu: Mmu::new(),
        }
    }

    pub fn reset(&mut self) {
        self.reg.reset();
        self.mmu.reset();
    }

    pub fn tick(&mut self) {
        self.reg.pc += 2;
    }

    pub fn execute(&mut self) {
        let mut bytes: usize = 1;
        let mut advance_pc: bool = true;

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