use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            data: vec![0;0x8000],
        }
    }

    pub fn load(&mut self, rom_path: &str) {
        // TODO: Add debug traces
        let rom_path = Path::new(rom_path);
        let mut rom_file = File::open(&rom_path).expect("Unable to open the ROM file");
        rom_file.read(&mut self.data).expect("Unable to read the ROM file data");
    }
}
