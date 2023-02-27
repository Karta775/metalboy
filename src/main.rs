extern crate log;
use metalboy::cpu::{Cpu, CLOCK_SPEED};
use metalboy::graphics::Graphics;
use std::env;
use std::process;
extern crate minifb;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use metalboy::cpu;
use metalboy::cpu::Status::InfiniteLoop;
use metalboy::joypad::{Button, Joypad};

// const SCALE: usize = 3;
const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    // Initialise the logger
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must provide a ROM file");
        process::exit(-1);
    }

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
    cpu.mmu.cartridge.load(&args[1]);
    // cpu.mmu.load_bootrom("dmg_boot.bin");
    cpu.mmu.load_bootrom("bootix_dmg.bin");

    let mut graphics = Graphics::new();

    // Emulation loop
    let mut cycles = 0;
    let mut instr_count = 0;
    let mut _cycle_count = 0;
    let max_cycles = CLOCK_SPEED / 60;
    let _quit_at = 7000000 * 100;
    let _max_warnings = 1;

    'running: while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let col = i % WIDTH;
            let row = i / WIDTH;
            *pixel = graphics.fb[col][row];
        }

        let mut pressed: Vec<Button> = Vec::new();
        window.get_keys().iter().for_each(|key| match key {
            Key::Up => pressed.push(Button::Up),
            Key::Down => pressed.push(Button::Down),
            Key::Left => pressed.push(Button::Left),
            Key::Right => pressed.push(Button::Right),
            Key::M => pressed.push(Button::A),
            Key::N => pressed.push(Button::B),
            Key::J => pressed.push(Button::Start),
            Key::H => pressed.push(Button::Select),
            _ => (),
        });

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        // One second of CPU execution ~ 4194304 cycles
        while cycles < max_cycles && cpu.status != InfiniteLoop {
            cpu.tick(); // Advance the CPU
            if cpu._tmp_warn_count >= _max_warnings {
                println!("Maximum number of warnings reached!");
                break 'running;
            }
            if !cpu.mmu.bootrom_mapped && cpu.status != cpu::Status::Halt {
                instr_count += 1;
                if instr_count >= _quit_at {
                    break 'running;
                }
            }
            cycles += cpu.cycles * 4; // FIXME: Is this M-cycles or actual cycles?
            _cycle_count += cycles;
            cpu.timer.update(&mut cpu.mmu, cpu.cycles * 4);
            graphics.update(&mut cpu.mmu, cpu.cycles * 4);
            Joypad::update(&mut cpu.mmu, &pressed);
            cpu.service_interrupts();
        }
        cycles = 0;
        if cpu.status == InfiniteLoop {
            break 'running;
        }
        // Missing: Emulate sound
        // Missing: Emulate other software
    }
    // println!("Total instructions executed: {}", instr_count);
    // println!("Total cycles: {}", cycle_count);
}
