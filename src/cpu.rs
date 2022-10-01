use super::registers::Registers;
use super::mmu::Mmu;

pub struct Cpu {
    pub reg: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
        }
    }

    pub fn tick(&mut self) {
        println!("CPU: tick");
        self.reg.pc += 2;
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