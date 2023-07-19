use egui::{Align, Context, Direction, Layout};
use metalboy::timer;
use crate::app::App;

impl App {
    pub fn show_state(&mut self, egui_ctx: &Context) {
        egui::Window::new("General State").show(egui_ctx, |ui| {
            ui.set_max_width(220.);

            // CPU
            ui.horizontal_wrapped(|ui| {
                self.header("CPU Info", ui);
                ui.label(format!("({:?})", self.cpu.status));
            });
            ui.horizontal_wrapped(|ui| {
                self.label_bold("PC:", ui);
                ui.label(format!("{:04X} ", self.cpu.reg.pc));
                self.label_bold("OP:", ui);
                ui.label(format!("{:02X} ", self.cpu.opcode));
                self.label_bold("SP:", ui);
                ui.label(format!("{:02X} ", self.cpu.reg.sp));
            });
            ui.horizontal_wrapped(|ui| {
                self.label_bold("NEXT OP:", ui);
                ui.label(format!("{:02X} ", self.cpu.mmu.get(self.cpu.reg.pc + 1)));
            });
            ui.separator();

            // MMU
            ui.horizontal_wrapped(|ui| {
                self.header("MMU Info", ui);
                ui.label(format!("(Cart uses MBC{})", self.cpu.mmu.cartridge.mbc));
            });
            ui.horizontal_wrapped(|ui| {
                self.label_bold("ROM BANK:", ui);
                ui.label(format!("{:02X} ", self.cpu.mmu.rom_bank));
            });
            ui.separator();

            // Timers
            self.header("Timers", ui);
            ui.horizontal_wrapped(|ui| {
                self.label_bold("DIV:", ui);
                ui.label(format!("{:02X} ", self.cpu.mmu.get(timer::DIV)));
                self.label_bold("TIMA:", ui);
                ui.label(format!("{:02X} ", self.cpu.mmu.get(timer::TIMA)));
                self.label_bold("TMA:", ui);
                ui.label(format!("{:02X} ", self.cpu.mmu.get(timer::TMA)));
            });
            ui.separator();

            // Columnar view of register values and set flags
            self.header("Registers", ui);
            ui.columns(2, |columns| {
                columns[0].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        self.label_bold("AF:", ui);
                        ui.label(format!("{:02X} {:02X}", self.cpu.reg.a, self.cpu.reg.f.as_u8()));
                    }); // AF
                    ui.horizontal_wrapped(|ui| {
                        self.label_bold("BC:", ui);
                        ui.label(format!("{:02X} {:02X}", self.cpu.reg.b, self.cpu.reg.c));
                    }); // BC
                    ui.horizontal_wrapped(|ui| {
                        self.label_bold("DE:", ui);
                        ui.label(format!("{:02X} {:02X}", self.cpu.reg.d, self.cpu.reg.e));
                    }); // DE
                    ui.horizontal_wrapped(|ui| {
                        self.label_bold("HL:", ui);
                        ui.label(format!("{:02X} {:02X}", self.cpu.reg.h, self.cpu.reg.l));
                    }); // HL
                });
                columns[1].with_layout(egui::Layout::top_down(Align::TOP), |ui| {
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        self.cpu.reg.f.zero,
                        "Zero"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        self.cpu.reg.f.sub,
                        "Sub"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        self.cpu.reg.f.half_carry,
                        "Half-carry"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        self.cpu.reg.f.carry,
                        "Carry"
                    ));
                });
            });


            // Columnar view of register values and set flags
            self.header("Interrupts", ui);
            let int_enable = self.cpu.mmu.get(0xFFFF);
            let int_flag = self.cpu.mmu.get(0xFF0F);
            ui.columns(2, |columns| {
                columns[0].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        self.cpu.ime,
                        "IME"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        check_bit(int_enable, 1),
                        "LCD STAT"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        check_bit(int_enable, 3),
                        "Serial"
                    ));
                });
                columns[1].with_layout(egui::Layout::top_down(Align::TOP), |ui| {
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        check_bit(int_enable, 0),
                        "VBlank"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        check_bit(int_enable, 2),
                        "Timer"
                    ));
                    ui.add_enabled(false, egui::SelectableLabel::new(
                        check_bit(int_enable, 4),
                        "Joypad"
                    ));
                });
            });
        });
    }
}

fn check_bit(byte: u8, bit: u8) -> bool {
    (byte >> bit) & 1 == 1
}