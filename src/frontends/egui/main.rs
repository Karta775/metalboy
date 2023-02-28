#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod common;
mod state;
mod tileset;

use macroquad::prelude::*;
use app::App;
extern crate log;
use metalboy::cpu::CLOCK_SPEED;
use metalboy::graphics::Graphics;
use std::env;
use std::process;
use egui::Color32;
use egui::style::{Selection, Visuals};
use common::*;

extern crate minifb;
use metalboy::cpu::Status::InfiniteLoop;
use metalboy::joypad::{Button, Joypad};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const BORDER_SIZE: f32 = 2.0;

const KEY_MAP: [(KeyCode, Button); 8] = [
    (KeyCode::Up,    Button::Up),
    (KeyCode::Down,  Button::Down),
    (KeyCode::Left,  Button::Left),
    (KeyCode::Right, Button::Right),
    (KeyCode::X,     Button::A),
    (KeyCode::Z,     Button::B),
    (KeyCode::S,     Button::Start),
    (KeyCode::A,     Button::Select),
];

fn window_conf() -> Conf {
    Conf {
        window_title: "metalboy debug".to_owned(),
        high_dpi: true,
        window_resizable: true,
        window_width: 960,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must provide a ROM file");
        process::exit(-1);
    }
    // Initialise the logger
    env_logger::init();

    let mut app = App::new();
    app.cpu.mmu.cartridge.load(&args[1]);
    // app.cpu.mmu.load_bootrom("bootix_dmg.bin");
    app.cpu.mmu.load_bootrom("dmg_boot.bin");
    let mut graphics = Graphics::new();

    // Emulation loop
    let mut cycles = 0;
    let mut _cycle_count = 0;
    let max_cycles = CLOCK_SPEED / 60;
    let _quit_at = 7000000 * 100;
    let _max_warnings = 1;

    // Set up texture for macroquad
    let mut texture = fb_to_texture2d(&graphics.fb);
    texture.set_filter(FilterMode::Nearest);


    // Setup
    egui_macroquad::ui(|ctx| {
        setup_custom_fonts(&ctx);
        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.selection.bg_fill = SELECTED_BG_FILL;
        ctx.set_style(style);
    });

    loop {
        egui_macroquad::ui(|egui_ctx| {
            app.draw_windows(&egui_ctx);
        });

        // Get key presses
        let mut pressed: Vec<Button> = Vec::new();
        for (key, button) in KEY_MAP {
            if is_key_down(key) {
                pressed.push(button);
            }
        }

        // Emulation loop for 1/60 of the CPU clock
        while cycles < CLOCK_SPEED / 60 && app.cpu.status != InfiniteLoop {
            app.cpu.tick(); // Advance the CPU
            cycles += app.cpu.cycles * 4;
            _cycle_count += cycles;
            app.cpu.timer.update(&mut app.cpu.mmu, app.cpu.cycles * 4);
            graphics.update(&mut app.cpu.mmu, app.cpu.cycles * 4);
            Joypad::update(&mut app.cpu.mmu, &pressed);
            app.cpu.service_interrupts();
        }
        cycles = 0;

        // Render everything
        texture = fb_to_texture2d(&graphics.fb);
        clear_background(BLACK);
        set_camera(&Camera2D {
            zoom: vec2(4.0 / screen_width(), 4.0 / screen_height()),
            target: vec2((WIDTH / 2) as f32, (HEIGHT / 2) as f32),
            ..Default::default()
        });
        draw_rectangle(-BORDER_SIZE, -BORDER_SIZE,
                       WIDTH as f32 + BORDER_SIZE * 2.,
                       HEIGHT as f32 + BORDER_SIZE * 2.,
                       DARKGRAY
        );
        draw_texture_ex(texture, 0.0, 0.0, WHITE,
                        DrawTextureParams{
                            flip_y: true,
                            ..Default::default()
                        }
        );
        egui_macroquad::draw();
        next_frame().await
    }
}

fn fb_to_texture2d(framebuffer: &[[u32; 144]; 160]) -> Texture2D {
    let mut bytes: Vec<u8> = Vec::from([0; WIDTH * HEIGHT * 4]);
    for i in 0..(WIDTH * HEIGHT) {
        let col = i % WIDTH;
        let row = i / WIDTH;
        let rgb = framebuffer[col][row];
        let r = (rgb & 0xFF0000) >> 16;
        let g = (rgb & 0x00FF00) >> 8;
        let b =  rgb & 0x0000FF;
        let offset = i * 4;
        bytes[offset + 0] = r as u8;
        bytes[offset + 1] = g as u8;
        bytes[offset + 2] = b as u8;
        bytes[offset + 3] = 255;
    }
    let texture = Texture2D::from_rgba8(WIDTH as u16, HEIGHT as u16, &bytes);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "JetBrains Mono".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/JetBrainsMono/fonts/ttf/JetBrainsMono-Regular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "JetBrains Mono".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("JetBrains Mono".to_owned());

    ctx.set_fonts(fonts);
}