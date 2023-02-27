extern crate core;

pub mod mmu;
pub mod cpu;
pub mod registers;
pub mod cartridge;
pub mod system;
pub mod decode;
pub mod execute;
pub mod flags;
pub mod graphics;
pub mod timer;
pub mod joypad;

pub const LOGGING_ENABLED: bool = true;

fn word_from(left: u8, right: u8) -> u16 {
    ((left as u16) << 8) | right as u16
}

fn bytes_from(word: u16) -> (u8, u8) {
    let left = ((word & 0xFF00) >> 8) as u8;
    let right = (word & 0x00FF) as u8;
    (left, right)
}

fn check_bit(byte: u8, bit: u8) -> bool {
    (byte >> bit) & 1 == 1
}

fn set_bit(byte: &mut u8, bit: u8) {
    *byte |= 1 << bit;
}

fn unset_bit(byte: &mut u8, bit: u8) {
    *byte &= !(1 << bit);
}

fn _toggle_bit(byte: &mut u8, bit: u8) {
    *byte ^= 1 << bit;
}

#[cfg(test)]
mod tests {
    use crate::{check_bit, set_bit, word_from};

    #[test]
    fn word_from_ok() {
        assert_eq!(word_from(0xBE, 0xEF), 0xBEEF);
    }

    #[test]
    fn check_bit_ok() {
        assert!(check_bit(0b01101010, 1));
        assert!(!check_bit(0b01101010, 2));
    }

    #[test]
    fn set_bit_ok() {
        let mut byte = 0b10101010;
        set_bit(&mut byte, 6);
        assert_eq!(byte, 0b11101010);
    }
}