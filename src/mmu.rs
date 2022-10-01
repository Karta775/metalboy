
pub struct Mmu {

}

impl Mmu {
    pub fn get(&self, address: u16) -> () {
        match address {
               0x0..0x3FFF => (), // 16KB ROM bank 00
            0x4000..0x7FFF => (), // 16KB ROM Bank 01~NN
            0x8000..0x9FFF => (), // 8KB Video RAM (VRAM)
            0xA000..0xBFFF => (), // 8KB External RAM
            0xC000..0xCFFF => (), // 4KB Work RAM (WRAM) bank 0
            0xD000..0xDFFF => (), // 4KB Work RAM (WRAM) bank 1~N
            0xE000..0xFDFF => (), // Mirror of C000~DDFF (ECHO RAM)
            0xFE00..0xFE9F => (), // Sprite attribute table (OAM)
            0xFEA0..0xFEFF => (), // Not usable
            0xFF00..0xFF7F => (), // I/O Registers
            0xFF80..0xFFFE => (), // High RAM (HRAM)
                    0xFFFF => (), // Interrupts Enable Register (IE)
            _ => () // Clion requires this catch-all even though the match is exhaustive :(
        }
    }
}