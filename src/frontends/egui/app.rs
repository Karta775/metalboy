use egui::{Context, RichText, Ui, Color32, Align, Layout, Direction, TextureHandle, ColorImage};
use egui::Direction::LeftToRight;
use egui_memory_editor::MemoryEditor;
use metalboy::cpu::Cpu;
use metalboy::graphics::Graphics;
use metalboy::timer;
use super::common::*;

pub struct App {
    pub cpu: Cpu,
    pub old_tileset_vram: [u8; 0x1800],
    pub tileset_image: ColorImage,
    pub graphics: Graphics,
    pub log_history: Vec<String>,
    pub opcode_history: Vec<(bool, u8)>,
    pub pause_execution: bool,
    pub step: bool,
    pub show_tileset_view: bool,
    pub show_control_view: bool,
    pub show_state_view: bool,
    pub show_log_view: bool,
    pub show_mem_editor: bool,
    pub mem_editor: MemoryEditor,
}

impl App {
    pub fn new() -> Self {
        App {
            cpu: Cpu::new(),
            old_tileset_vram: [0; 0x1800],
            tileset_image: ColorImage::new([128, 192], Color32::BLACK),
            graphics: Graphics::new(),
            log_history: vec![],
            opcode_history: vec![],
            pause_execution: false,
            step: false,
            show_tileset_view: false,
            show_control_view: true,
            show_state_view: true,
            show_log_view: false,
            show_mem_editor: false,
            mem_editor: MemoryEditor::new()
                .with_address_range("0. All", 0..0xFFFF)
                .with_address_range("1. ROM", 0x0000..0x8000)
                .with_address_range("2. VRAM", 0x8000..0xA000)
                .with_address_range("3. EXTRAM", 0xA000..0xC000)
                .with_address_range("4. WRAM", 0xC000..0xE000)
                .with_address_range("5. OAM", 0xFE00..0xFEA0)
                .with_address_range("6. IO", 0xFF00..0xFF80)
                .with_address_range("7. HRAM", 0xFF80..0xFFFF)
                .with_window_title("Memory Editor"),
        }
    }

    pub fn draw_windows(&mut self, egui_ctx: &Context) {
        self.show_menubar(egui_ctx);
        if self.show_tileset_view { self.show_tileset(egui_ctx); }
        if self.show_control_view { self.show_control(egui_ctx); }
        if self.show_log_view { self.show_log(egui_ctx); }
        if self.show_state_view { self.show_state(egui_ctx); }

        self.mem_editor.window_ui(
            egui_ctx,
            &mut self.show_mem_editor,
            &mut self.cpu.mmu,
            |mmu, address| mmu.get(address as u16).into(),
            |mmu, address, val| mmu.set(address as u16, val),
        );
    }

    pub(crate) fn header(&mut self, text: &str, ui: &mut Ui) {
        ui.label(RichText::new(text).color(HEADER_COLOUR));
    }

    pub(crate) fn label_bold(&mut self, text: &str, ui: &mut Ui) {
        ui.label(RichText::new(text).color(BOLD_FONT_COLOUR));
    }
}
