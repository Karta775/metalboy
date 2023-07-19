use crate::{word_from, bytes_from};
use crate::flags::Flags;

#[derive(PartialEq, Clone, Copy)]
pub enum R8 {
    A,
    B, C,
    D, E,
    H, L,
    HLRam
}

impl R8 {
    pub fn from_spec(index: u8) -> Self {
        match index {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::HLRam,
            7 => Self::A,
            _ => panic!("This is supposed to be unreachable"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

impl R16 {
    pub fn from_spec(index: u8) -> Self {
        match index {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HL,
            3 => Self::SP,
            _ => panic!("This is supposed to be unreachable"),
        }
    }
}

pub struct Registers {
    pub a: u8, pub f: Flags,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        let f = Flags {
            zero: false,
            sub: false,
            carry: false,
            half_carry: false
        };
        Registers {
            a: 0, f,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            pc: 0,
            sp: 0xFFFE,
        }
    }

    pub fn reset(&mut self) {
        self.set_af(0);
        self.set_bc(0);
        self.set_de(0);
        self.set_hl(0);
        self.pc = 0;
        self.sp = u16::MAX - 1;
    }

    pub fn af(&self) -> u16 {
        word_from(self.a, self.f.as_u8())
    }

    pub fn bc(&self) -> u16 {
        word_from(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        word_from(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        word_from(self.h, self.l)
    }

    pub fn set_af(&mut self, word: u16) {
        let (a,f) = bytes_from(word);
        self.a = a;
        self.f.set_from_u8(f);
    }

    pub fn set_bc(&mut self, word: u16) {
        let (b,c) = bytes_from(word);
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, word: u16) {
        let (d,e) = bytes_from(word);
        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, word: u16) {
        let (h,l) = bytes_from(word);
        self.h = h;
        self.l = l;
    }

    pub fn set_sp(&mut self, word: u16) {
        self.sp = word;
    }

    pub fn hl_post_inc(&mut self) -> u16 {
        let old = self.hl();
        self.set_hl(self.hl().wrapping_add(1));
        old
    }

    pub fn hl_post_dec(&mut self) -> u16 {
        let old = self.hl();
        self.set_hl(self.hl().wrapping_sub(1));
        old
    }

    pub fn inc_hl_nf(&mut self) {
        self.set_hl(self.hl().wrapping_add(1));
    }

    pub fn dec_hl_nf(&mut self) {
        self.set_hl(self.hl().wrapping_sub(1));
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::Registers;

    #[test]
    fn compute_half_carry_add_ok() {
        let mut registers = Registers::new();
        assert!(!registers.f.half_carry);
        registers.f.compute_half_carry_add(0b00001010, 0b00001100);
        assert!(registers.f.half_carry);
    }

    #[test]
    fn compute_half_carry_add_no_carry() {
        let mut registers = Registers::new();
        assert!(!registers.f.half_carry);
        registers.f.compute_half_carry_add(0b00000101, 0b00000100);
        assert!(!registers.f.half_carry);
    }

    #[test]
    fn flags_as_u8_ok() {
        let mut registers = Registers::new();
        registers.f.zero = true;
        registers.f.half_carry = true;
        assert_eq!(registers.f.as_u8(), 0b10100000);
    }

    #[test]
    fn flags_set_from_u8_ok() {
        let mut registers = Registers::new();
        registers.f.set_from_u8(0b10010000);
        assert!( registers.f.zero);
        assert!(!registers.f.sub);
        assert!(!registers.f.half_carry);
        assert!( registers.f.carry);
    }

    #[test]
    fn get_register_combo() {
        let mut registers = Registers::new();
        registers.b = 0xAB;
        registers.c = 0xCD;
        assert_eq!(registers.bc(), 0xABCD);
    }

    #[test]
    fn set_register_combo() {
        let mut registers = Registers::new();
        registers.set_bc(0xABCD);
        assert_eq!(registers.b, 0xAB);
        assert_eq!(registers.c, 0xCD);
    }
}