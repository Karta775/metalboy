use std::fs::File;
use std::io::Read;
use log::error;
use crate::cartridge::Cartridge;

pub struct Mmu {
    pub bootrom: [u8; 256],
    pub bootrom_mapped: bool,
    pub cartridge: Cartridge,
    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x1000],
    pub io: [u8; 0x80],
    pub hram: [u8; 128],
    pub oam: [u8; 0xA0],
    pub ie: u8,
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
            ie: 0,
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
        if self.bootrom_mapped {
            match address {
                0x0000..=0x0100 => return self.bootrom[address as usize],
                _ => ()
            }
        }
        match address {
            0x0000..=0x3FFF => self.cartridge.data[address as usize], // 16KB ROM bank 00
            0x4000..=0x7FFF => self.cartridge.data[address as usize], // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000], // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => {error!("Reading non-existent memory: ({:04x}) EXT RAM", address);0}, // 8KB External RAM
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000], // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => {error!("Reading non-existent memory: ({:04x}) WRAM 0~N", address);0}, // 4KB Work RAM (WRAM) bank 1~N
            0xE000..=0xFDFF => {error!("Reading non-existent memory: ({:04x}) ECHO RAM", address);0}, // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00], // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => {error!("Reading non-existent memory: ({:04x}) UNUSABLE", address);0}, // Not usable
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00], // I/O Registers
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80], // High RAM (HRAM)
                     0xFFFF => self.ie, // Interrupts Enable Register (IE)
            _ => 0 // Clion requires this catch-all even though the match is exhaustive :(
        }
    }

    pub fn set(&mut self, address: u16, byte: u8) {
        #[allow(unreachable_patterns)]
        match address {
            0x0000..=0x3FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 00", byte, address), // 16KB ROM bank 00
            0x4000..=0x7FFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ROM 01~NN", byte, address), // 16KB ROM Bank 01~NN
            0x8000..=0x9FFF => {
                self.vram[address as usize - 0x8000] = byte
            }, // 8KB Video RAM (VRAM)
            0xA000..=0xBFFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) EXT RAM", byte, address), // 8KB External RAM
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000] = byte, // 4KB Work RAM (WRAM) bank 0
            0xD000..=0xDFFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) WRAM 1~N", byte, address), // 4KB Work RAM (WRAM) bank 1~N
            0xE000..=0xFDFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) ECHO RAM", byte, address), // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => { self.oam[address as usize - 0xFE00] = byte }, // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => error!("Writing to non-existent memory: {:02x} -> ({:04x}) UNUSABLE", byte, address), // Not usable
            0xFF00..=0xFF7F => {
                match address {
                    0xFF46 => self.dma_transfer(byte),
                    _ => self.io[address as usize - 0xFF00] = byte
                }
            }, // I/O Registers
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = byte, // High RAM (HRAM)
                     0xFFFF => self.ie = byte, // Interrupts Enable Register (IE)
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