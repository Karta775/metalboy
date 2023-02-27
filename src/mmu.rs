use std::fs::File;
use std::io::Read;
use log::error;
use crate::cartridge::Cartridge;
use crate::timer;
use crate::joypad;

const OFFSET: usize = 0x8000;

pub struct Mmu {
    pub bootrom: [u8; 256],
    pub bootrom_mapped: bool,
    pub cartridge: Cartridge,
    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x1000],
    pub io: [u8; 0x80],
    pub hram: [u8; 128],
    pub oam: [u8; 0xA0],
    pub memory: [u8; 0x8000],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            // TODO: Consider switching to one array and taking slices
            bootrom: [0; 256],
            bootrom_mapped: true,
            cartridge: Cartridge::new(),
            vram: [0; 0x2000],
            wram: [0; 0x1000],
            io: [0; 0x80],
            hram: [0; 128],
            oam: [0; 0xA0],
            memory: [0; 0x8000],
        }
    }

    pub fn load_bootrom(&mut self, rom_path: &str) {
        let mut rom_file = File::open(&rom_path).expect("Unable to open the ROM file");
        rom_file.read(&mut self.bootrom).expect("Unable to read the ROM file data");
    }

    pub fn reset(&mut self) {
        // TODO: Reset memory to defaults
    }

    #[allow(unreachable_patterns)]
    pub fn get(&self, address: u16) -> u8 {
        let split_address = address as usize % OFFSET;
        if self.bootrom_mapped {
            match address {
                0x00..=0xFF => return self.bootrom[split_address],
                _ => ()
            }
        }
        match address {
            // 0xFF44 => 0x90, // TODO: Remove debug code
            0x0000..=0x3FFF => self.cartridge.data[split_address], // 16KB ROM bank 00
            0x4000..=0x7FFF => self.cartridge.data[split_address], // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => self.memory[split_address], // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => self.memory[split_address], // 8KB External RAM
            0xC000..=0xCFFF => self.memory[split_address], // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => self.memory[split_address], // 4KB Work RAM (WRAM) bank 1~N TODO: Banking
            0xE000..=0xFDFF => self.memory[split_address - 0x2000], // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => self.memory[split_address], // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => 0, // Not usable
            0xFF00..=0xFF7F => {
                match address {
                    0xFF00 => self.memory[split_address],
                    _ => self.memory[split_address]
                }
            }, // I/O Registers
            0xFF80..=0xFFFE => self.memory[split_address], // High RAM (HRAM)
                     0xFFFF => self.memory[split_address], // Interrupts Enable Register (IE)
            _ => 0 // Clion requires this catch-all even though the match is exhaustive :(
        }
    }

    pub fn set(&mut self, address: u16, byte: u8) {
        let split_address = address as usize % OFFSET;
        #[allow(unreachable_patterns)]
        match address {
            0x0000..=0x3FFF => { error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 00", byte, address) }, // 16KB ROM bank 00
            0x4000..=0x7FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 01~NN", byte, address), // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => self.memory[split_address] = byte, // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => self.memory[split_address] = byte, // 8KB External RAM
            0xC000..=0xCFFF => self.memory[split_address] = byte, // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => self.memory[split_address] = byte, // 4KB Work RAM (WRAM) bank 1~N // TODO: Banking
            0xE000..=0xFDFF => self.memory[split_address - 0x2000] = byte, // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => self.memory[split_address] = byte , // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => { }, // Not usable
            0xFF00..=0xFF7F => {
                match address {
                    joypad::JOYP => {
                        let current = self.get(joypad::JOYP);
                        let new = (byte & 0xf0) | (current & 0x0f);
                        self.memory[joypad::JOYP as usize % OFFSET] = new;
                    },
                    timer::DIV => self.memory[split_address] = 0,
                    0xFF46 => self.dma_transfer(byte),
                    _ => self.memory[split_address] = byte
                }
            }, // I/O Registers
            0xFF80..=0xFFFE => self.memory[split_address] = byte, // High RAM (HRAM)
                     0xFFFF => self.memory[split_address] = byte, // Interrupts Enable Register (IE)
            _ => {} // Clion requires this catch-all even though the match is exhaustive :(
        }
    }

    // This method bypasses set()
    pub fn set_joypad_buttons(&mut self, byte: u8) {
        let current = self.get(joypad::JOYP);
        let new = (current & 0xf0) | (byte & 0x0f);
        self.memory[joypad::JOYP as usize % OFFSET] = new;
    }

    pub fn set_initial_state(&mut self) {
        self.set(0xFF00, 0xCF);
        self.set(0xFF01, 0x00);
        self.set(0xFF02, 0x7E);
        self.set(0xFF04, 0xAB);
        self.set(0xFF05, 0x00);
        self.set(0xFF06, 0x00);
        self.set(0xFF07, 0xF8);
        self.set(0xFF0F, 0xE1);
        self.set(0xFF10, 0x80);
        self.set(0xFF11, 0xBF);
        self.set(0xFF12, 0xF3);
        self.set(0xFF13, 0xFF);
        self.set(0xFF14, 0xBF);
        self.set(0xFF16, 0x3F);
        self.set(0xFF17, 0x00);
        self.set(0xFF18, 0xFF);
        self.set(0xFF19, 0xBF);
        self.set(0xFF1A, 0x7F);
        self.set(0xFF1B, 0xFF);
        self.set(0xFF1C, 0x9F);
        self.set(0xFF1D, 0xFF);
        self.set(0xFF1E, 0xBF);
        self.set(0xFF20, 0xFF);
        self.set(0xFF21, 0x00);
        self.set(0xFF22, 0x00);
        self.set(0xFF23, 0xBF);
        self.set(0xFF24, 0x77);
        self.set(0xFF25, 0xF3);
        self.set(0xFF26, 0xF1);
        self.set(0xFF40, 0x91);
        self.set(0xFF41, 0x85);
        self.set(0xFF42, 0x00);
        self.set(0xFF43, 0x00);
        self.set(0xFF44, 0x00);
        self.set(0xFF45, 0x00);
        self.set(0xFF46, 0xFF);
        self.set(0xFF47, 0xFC);
        // self.set(0xFF48, 0x00);
        // self.set(0xFF49, 0x00);
        self.set(0xFF4A, 0xFF);
        self.set(0xFF4B, 0xFF);
        self.set(0xFF4D, 0xFF);
        self.set(0xFF4F, 0xFF);
        self.set(0xFF51, 0xFF);
        self.set(0xFF52, 0xFF);
        self.set(0xFF53, 0xFF);
        self.set(0xFF54, 0xFF);
        self.set(0xFF55, 0xFF);
        self.set(0xFF56, 0xFF);
        self.set(0xFF68, 0xFF);
        self.set(0xFF69, 0xFF);
        self.set(0xFF6A, 0xFF);
        self.set(0xFF6B, 0xFF);
        self.set(0xFF70, 0xFF);
        self.set(0xFFFF, 0x00);
    }

    pub fn dma_transfer(&mut self, data: u8) {
        let address: u16 = (data as u16) << 8;
        for i in 0x00..0xA0 {
            self.set(0xFE00 + i, self.get(address + i));
        }
    }

    pub fn request_interrupt(&mut self, id: u8) {
        let mut interrupt_flag = self.get(0xFF0F);
        interrupt_flag |= 1 << id;
        self.set(0xFF0F, interrupt_flag);
    }
}