use gemboy::cpu::Cpu;

fn main() {
    println!("Hello, gemboy!");

    // Create CPU
    let mut cpu = Cpu::new();

    // Emulation loop
    loop {
        cpu.tick();
    }
}
