use std::fs::File;
use std::io::Read;
use std::path::Path;
use log::warn;

pub struct Cartridge {
    pub data: Vec<u8>,
    pub mbc: u8,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            data: vec![],
            mbc: 0,
        }
    }

    pub fn load(&mut self, rom_path: &str) {
        // TODO: Add debug traces
        let rom_path = Path::new(rom_path);
        let mut rom_file = File::open(&rom_path).expect("Unable to open the ROM file");
        let rom_size = rom_file.metadata().expect("Unable to read ROM file metadata").len();
        self.data.resize(rom_size as usize, 0);
        rom_file.read(&mut self.data).expect("Unable to read the ROM file data");
        self.set_mbc();
    }

    fn set_mbc(&mut self) {
        let mbc_id = self.data[0x147];
        self.mbc = match mbc_id {
            0x0 => 0, // ROM ONLY
            0x1 => 1, // MBC1
            0x2 => 1, // MBC1+RAM
            0x3 => 1, // MBC1+RAM+BATTERY
            _ => {warn!("Unknown MBC id '{}'", mbc_id); 0},
        };
    }
}

