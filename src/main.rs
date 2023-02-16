extern crate log;
use metalboy::cpu::Cpu;
use metalboy::graphics::Graphics;
extern crate minifb;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

// const SCALE: usize = 3;
const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const CLOCK_SPEED: usize = 4194304;

fn main() {
    // Initialise the logger
    env_logger::init();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "metalboy",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
            transparency: false,
            none: false,
        }
    ).unwrap_or_else(|e| { panic!("{}", e); });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Create CPU
    let mut cpu = Cpu::new();
    // cpu.mmu.cartridge.load("gb-test-roms/cpu_instrs/individual/06-ld r,r.gb");
    cpu.mmu.cartridge.load("gb-test-roms/cpu_instrs/individual/05-op rp.gb");
    // cpu.mmu.cartridge.load("test.gb");
    // cpu.mmu.cartridge.load("tetris.gb");
    cpu.mmu.load_bootrom("dmg_boot.bin");
    // cpu.mmu.load_bootrom("bootix_dmg.bin");

    let mut graphics = Graphics::new();

    // Emulation loop
    let mut cycles = 0;
    let mut instr_count = 0;
    let mut cycle_count = 0;
    let max_cycles = CLOCK_SPEED / 60;
    let _quit_at = 166666508 + 5;
    let _max_warnings = 1;

    'running: while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let col = i % WIDTH;
            let row = i / WIDTH;
            *pixel = graphics.fb[col][row];
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        // One second of CPU execution ~ 4194304 cycles
        while cycles < max_cycles {
            cpu.tick(); // Advance the CPU
            if cpu._tmp_warn_count >= _max_warnings {
                println!("Maximum number of warnings reached!");
                break 'running;
            }
            if !cpu.mmu.bootrom_mapped {
                instr_count += 1;
                if instr_count >= _quit_at {
                    break 'running;
                }
            }
            cycles += cpu.cycles * 4; // FIXME: Is this M-cycles or actual cycles?
            cycle_count += cycles;
            graphics.update(&mut cpu.mmu, cpu.cycles * 4);
            cpu.generate_interrupts();
        }
        cycles = 0;

        // Missing: Emulate sound
        // Missing: Emulate other software
        // Missing: Time synchronisation
    }

    // println!("Total instructions executed: {}", instr_count);
    // println!("Total cycles: {}", cycle_count);
}
