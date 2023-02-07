
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

    pub fn compute_half_carry_add(&mut self, left: u8, right: u8) {
        self.half_carry = ((left & 0xf) + (right & 0xf)) & 0x10 == 0x10;
    }

    pub fn compute_half_carry_sub(&mut self, left: u8, right: u8) {
        let sub = u8::wrapping_sub(left & 0xf, right & 0xf);
        self.half_carry = sub & 0x10 == 0x10;
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