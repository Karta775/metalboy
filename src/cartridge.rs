use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            data: Vec::new(),
        }
    }

    pub fn load(&mut self, rom_path: &str) {
        // TODO: Add debug traces
        let mut rom_file = File::open(&rom_path).expect("Unable to open the ROM file");
        rom_file.read_to_end(&mut self.data).expect("Unable to read the ROM file data");
    }
}

#[cfg(test)]
mod tests {
    use crate::cartridge::Cartridge;

    #[test]
    fn load_rom_file() {
        let mut cart = Cartridge::new();
        cart.load("tests/1kb_random_data.gb");
        assert_eq!(cart.data.len(), 1024);
    }
}