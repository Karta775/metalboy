#[macro_use]
extern crate log;
use metalboy::cpu::Cpu;

fn main() {
    // Initialise the logger
    env_logger::init();

    // Create CPU
    let mut cpu = Cpu::new();
    cpu.mmu.cartridge.load("tetris.gb");

    // Emulation loop
    loop {
        cpu.tick(); // Advance the CPU
        // Missing: Generate interrupts
        // Missing: Emulate graphics
        // Missing: Emulate sound
        // Missing: Emulate other software
        // Missing: Time synchronisation
    }
}
