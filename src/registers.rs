use crate::{word_from, bytes_from};

pub struct Flags {
    pub zero: bool,
    pub sub: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result += if self.zero { "z" } else { " " };
        result += if self.sub { "n" } else { " " };
        result += if self.half_carry { "h" } else { " " };
        result += if self.carry { "c" } else { " " };
        result
    }

    pub fn as_u8(&self) -> u8 {
        let mut f = 0b0000000;
        f += (self.zero as u8)       << 7;
        f += (self.sub as u8)        << 6;
        f += (self.half_carry as u8) << 5;
        f += (self.carry as u8)      << 4;
        return f;
    }

    pub fn set_from_u8(&mut self, f: u8) {
        self.zero =       ((f >> 7) & 1) == 1;
        self.sub =        ((f >> 6) & 1) == 1;
        self.half_carry = ((f >> 5) & 1) == 1;
        self.carry =      ((f >> 4) & 1) == 1;
    }

    pub fn set_from_bool(&mut self, zero: bool, sub: bool, half_carry: bool, carry: bool) {
        self.zero = zero;
        self.sub = sub;
        self.half_carry = half_carry;
        self.carry = carry;
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
            sp: u16::MAX,
        }
    }

    pub fn reset(&mut self) {
        self.set_bc(0);
        self.set_de(0);
        self.set_hl(0);
        self.pc = 0;
        self.sp = u16::MAX;
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
}

#[cfg(test)]
mod tests {
    use crate::registers::Registers;

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