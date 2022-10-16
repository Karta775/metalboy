use crate::{word_from, bytes_from};

pub struct Registers {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn reset(&mut self) {
        self.set_bc(0);
        self.set_de(0);
        self.set_hl(0);
        self.pc = 0;
        self.sp = 0;
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