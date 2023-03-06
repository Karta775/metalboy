use std::ops::MulAssign;
use egui::{Align, Color32, ColorImage, Context, Direction, Image, Layout, Pos2, TextureFilter, TextureHandle, TextureOptions};
use egui::panel::TopBottomSide;
use log::trace;
use metalboy::decode::decode;
use metalboy::timer;
use crate::app::App;

pub const MAX_LOG_LINES: usize = 5;

impl App {
    pub fn show_log(&mut self, egui_ctx: &Context) {
        // let line = format!("PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
        //    self.cpu.reg.pc, decode(&self.cpu).expect("Unknown opcode"),
        //    self.cpu.reg.a, self.cpu.reg.f.to_string(), self.cpu.reg.b, self.cpu.reg.c, self.cpu.reg.d,
        //    self.cpu.reg.e, self.cpu.reg.h, self.cpu.reg.l, self.cpu.reg.sp,
        // );
        // if let Some(x) = self.log_history.first() {
        //     if x != &line {
        //         self.log_history.insert(0, line);
        //     }
        // } else {
        //     self.log_history.insert(0, line);
        // }
        // if self.log_history.len() > MAX_LOG_LINES  {
        //     self.log_history.pop();
        // }
        // egui::TopBottomPanel::new(TopBottomSide::Bottom, "bottom_panel").show(egui_ctx, |ui| {
        //     for line in self.log_history.iter().rev() {
        //         ui.label(line);
        //     }
        // });

        egui::TopBottomPanel::new(TopBottomSide::Bottom, "bottom_panel").show(egui_ctx, |ui| {
            for (cb_prefix, opcode) in self.opcode_history.iter().rev() {
                // let line = format!("PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
                //    self.cpu.reg.pc, decode(*cb_prefix, *opcode).expect("Unknown opcode"),
                //    self.cpu.reg.a, self.cpu.reg.f.to_string(), self.cpu.reg.b, self.cpu.reg.c, self.cpu.reg.d,
                //    self.cpu.reg.e, self.cpu.reg.h, self.cpu.reg.l, self.cpu.reg.sp,
                // );
                // ui.label(line);
            }
        });
    }
}