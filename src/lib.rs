pub mod mmu;
pub mod cpu;
pub mod registers;
pub mod cartridge;
pub mod system;
pub mod decode;
pub mod execute;

fn word_from(left: u8, right: u8) -> u16 {
    ((left as u16) << 8) | right as u16
}

fn bytes_from(word: u16) -> (u8, u8) {
    let left = ((word & 0xFF00) >> 8) as u8;
    let right = (word & 0x00FF) as u8;
    (left, right)
}

#[cfg(test)]
mod tests {
    use crate::word_from;

    #[test]
    fn word_from_ok() {
        assert_eq!(word_from(0xBE, 0xEF), 0xBEEF);
    }
}