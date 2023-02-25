use std::fs::File;
use std::io::Read;
use log::error;
use crate::cartridge::Cartridge;
use crate::timer::DIV;

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
        let split_address = address as usize % 0x8000;
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
            0xE000..=0xFDFF => {error!("Reading non-existent memory: ({:04x}) ECHO RAM", address);0}, // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => self.memory[split_address], // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => {error!("Reading non-existent memory: ({:04x}) UNUSABLE", address);0}, // Not usable
            0xFF00..=0xFF7F => self.memory[split_address], // I/O Registers
            0xFF80..=0xFFFE => self.memory[split_address], // High RAM (HRAM)
                     0xFFFF => self.memory[split_address], // Interrupts Enable Register (IE)
            _ => 0 // Clion requires this catch-all even though the match is exhaustive :(
        }
    }

    pub fn get_mut(&mut self, address: u16) -> &mut u8 {
        let split_address = address as usize % 0x8000;
        if self.bootrom_mapped {
            match address {
                0x00..=0xFF => return &mut self.bootrom[split_address],
                _ => ()
            }
        }
        #[allow(unreachable_patterns)]
        match address {
            0xFF44 => panic!("Not supposed to happen"), // TODO: Remove debug code
            0x0000..=0x3FFF => &mut self.cartridge.data[split_address], // 16KB ROM bank 00
            0x4000..=0x7FFF => &mut self.cartridge.data[split_address], // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => &mut self.memory[split_address], // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => &mut self.memory[split_address], // 8KB External RAM
            0xC000..=0xCFFF => &mut self.memory[split_address], // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => &mut self.memory[split_address], // 4KB Work RAM (WRAM) bank 1~N TODO: Banking
            0xE000..=0xFDFF => {panic!("Reading non-existent memory: ({:04x}) ECHO RAM", address); }, // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => &mut self.memory[split_address], // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => {panic!("Reading non-existent memory: ({:04x}) UNUSABLE", address); }, // Not usable
            0xFF00..=0xFF7F => &mut self.memory[split_address], // I/O Registers
            0xFF80..=0xFFFE => &mut self.memory[split_address], // High RAM (HRAM)
            0xFFFF => &mut self.memory[split_address], // Interrupts Enable Register (IE)
            _ => panic!(":(") // Clion requires this catch-all even though the match is exhaustive :(
        }
    }

    pub fn set(&mut self, address: u16, byte: u8) {
        let split_address = address as usize % 0x8000;
        #[allow(unreachable_patterns)]
        match address {
            0x0000..=0x3FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 00", byte, address), // 16KB ROM bank 00
            0x4000..=0x7FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 01~NN", byte, address), // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => self.memory[split_address] = byte, // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => self.memory[split_address] = byte, // 8KB External RAM
            0xC000..=0xCFFF => self.memory[split_address] = byte, // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => self.memory[split_address] = byte, // 4KB Work RAM (WRAM) bank 1~N // TODO: Banking
            0xE000..=0xFDFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ECHO RAM", byte, address), // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => self.memory[split_address] = byte , // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) UNUSABLE", byte, address), // Not usable
            0xFF00..=0xFF7F => {
                match address {
                    DIV => self.memory[split_address] = 0,
                    0xFF46 => self.dma_transfer(byte),
                    _ => self.memory[split_address] = byte
                }
            }, // I/O Registers
            0xFF80..=0xFFFE => self.memory[split_address] = byte, // High RAM (HRAM)
                     0xFFFF => self.memory[split_address] = byte, // Interrupts Enable Register (IE)
            _ => {} // Clion requires this catch-all even though the match is exhaustive :(
        }
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