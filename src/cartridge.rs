use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom_path: &str) -> Self {
        Cartridge {
            data: Cartridge::load(&rom_path)
        }
    }

    fn load(rom_path: &str) -> Vec<u8> {
        // TODO: Add debug traces
        let mut rom_file = File::open(&rom_path).expect("Unable to open the ROM file");
        let mut data: Vec<u8> = Vec::new();
        rom_file.read_to_end(&mut data).expect("Unable to read the ROM file data");
        data
    }
}

#[cfg(test)]
mod tests {
    use crate::cartridge::Cartridge;

    #[test]
    fn load_rom_file() {
        let mut cart = Cartridge::new("tests/1kb_random_data.gb");
        assert_eq!(cart.data.len(), 1024);
    }
}