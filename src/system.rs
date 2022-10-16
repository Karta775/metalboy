use crate::cpu::Cpu;

pub struct System {
    pub cpu: Cpu,
}

impl System {
    pub fn new() -> Self {
        let mut system = Self {
            cpu: Cpu::new(),
        };
        system.reset();
        system
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}