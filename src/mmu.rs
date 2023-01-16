use log::error;
use crate::cartridge::Cartridge;

pub struct Mmu {
    pub cartridge: Cartridge,
    pub vram: Vec<u8>,
    pub hram: Vec<u8>,
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            cartridge: Cartridge::new(),
            vram: vec![0; 0x2000],
            hram: vec![0; 128],
        }
    }

    pub fn reset(&mut self) {
        // TODO: Reset memory to defaults
    }

    pub fn get(&self, address: u16) -> u8 {
        #[allow(unreachable_patterns)]
        match address {
            0x0000..=0x3FFF => self.cartridge.data[address as usize], // 16KB ROM bank 00
            0x4000..=0x7FFF => self.cartridge.data[address as usize], // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000], // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => {error!("Reading non-existent memory: ({:04x}) EXT RAM", address);0}, // 8KB External RAM
            0xC000..=0xCFFF => {error!("Reading non-existent memory: ({:04x}) WRAM 0", address);0}, // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => {error!("Reading non-existent memory: ({:04x}) WRAM 0~N", address);0}, // 4KB Work RAM (WRAM) bank 1~N
            0xE000..=0xFDFF => {error!("Reading non-existent memory: ({:04x}) ECHO RAM", address);0}, // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => {error!("Reading non-existent memory: ({:04x}) OAM", address);0}, // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => {error!("Reading non-existent memory: ({:04x}) UNUSABLE", address);0}, // Not usable
            0xFF00..=0xFF7F => {error!("Reading non-existent memory: ({:04x}) I/O REG", address);0}, // I/O Registers
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80], // High RAM (HRAM)
                     0xFFFF => {error!("Reading non-existent memory: ({:04x}) INTERRUPT", address);0}, // Interrupts Enable Register (IE)
            _ => 0 // Clion requires this catch-all even though the match is exhaustive :(
        }
    }

    pub fn set(&mut self, address: u16, byte: u8) {
        #[allow(unreachable_patterns)]
        match address {
            0x0000..=0x3FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 00", byte, address), // 16KB ROM bank 00
            0x4000..=0x7FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 01~NN", byte, address), // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = byte, // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) EXT RAM", byte, address), // 8KB External RAM
            0xC000..=0xCFFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) WRAM 0", byte, address), // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) WRAM 0~N", byte, address), // 4KB Work RAM (WRAM) bank 1~N
            0xE000..=0xFDFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ECHO RAM", byte, address), // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => error!("Writing to non-existent memory: {:02x} -> ({:04x}) OAM", byte, address), // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) UNUSABLE", byte, address), // Not usable
            0xFF00..=0xFF7F => error!("Writing to non-existent memory: {:02x} -> ({:04x}) I/O REG", byte, address), // I/O Registers
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = byte, // High RAM (HRAM)
                     0xFFFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) INTERRUPT", byte, address), // Interrupts Enable Register (IE)
            _ => {} // Clion requires this catch-all even though the match is exhaustive :(
        }
    }
}