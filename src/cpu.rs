use crate::execute::execute;
use super::registers::{Registers, R8, R16};
use super::flags::Flags;
use super::mmu::Mmu;
use crate::{bytes_from, set_bit, unset_bit};

pub enum Interrupt {
    VBlank = 0x40,
    LCD = 0x48,
    TIMER = 0x50,
    JOYPAD = 0x60
}

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub opcode: u8,
    pub advance_pc: i16,
    pub cycles: usize,
    pub cb_prefix: bool,
    pub ime: bool,
    pub _tmp_warn_count: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
            mmu: Mmu::new(),
            opcode: 0x00,
            advance_pc: 1,
            cycles: 0,
            cb_prefix: false,
            ime: true,
            _tmp_warn_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.reg.reset();
        self.mmu.reset();
        self.opcode = 0x00;
        self.advance_pc = 1;
        self.cycles = 0;
    }

    pub fn tick(&mut self) {
        self.cycles = 0;
        self.opcode = self.mmu.get(self.reg.pc);
        execute(self);
        self.reg.pc = (self.reg.pc as i16 + self.advance_pc) as u16;
        self.advance_pc = 1;
        if self.mmu.bootrom_mapped && self.reg.pc >= 0x100 {
            // warn!("End of bootrom emulation, unmapping bootrom"); // TODO: Debugging
            self.mmu.bootrom_mapped = false;
            assert_eq!(self.reg.a, 0x01);
            assert_eq!(self.reg.f.as_u8(), 0xB0);
            assert_eq!(self.reg.b, 0);
            assert_eq!(self.reg.c, 0x13);
            assert_eq!(self.reg.d, 0);
            assert_eq!(self.reg.e, 0xd8);
            assert_eq!(self.reg.h, 0x01);
            assert_eq!(self.reg.l, 0x4d);
            assert_eq!(self.reg.sp, 0xFFFE);
            assert_eq!(self.reg.pc, 0x100);
        }
    }

    pub fn execute(&mut self) {
        let mut _bytes: usize = 1;
        let mut _advance_pc: bool = true;
        // self.mmu.get(self.reg.pc);
    }

    pub fn get_op(&self, offset: u16) -> u8 {
        self.mmu.get(self.reg.pc + offset)
    }

    pub fn get_reg(&mut self, index: u8) -> u8 {
        match index {
            0 => self.reg.b,
            1 => self.reg.c,
            2 => self.reg.d,
            3 => self.reg.e,
            4 => self.reg.h,
            5 => self.reg.l,
            6 => self.mmu.get(self.reg.hl()),
            7 => self.reg.a,
            _ => panic!("This is supposed to be unreachable"),
        }
    }

    pub fn get_reg8_mut(&mut self, reg: R8) -> &mut u8 {
        match reg {
            R8::A => &mut self.reg.a,
            R8::B => &mut self.reg.b,
            R8::C => &mut self.reg.c,
            R8::D => &mut self.reg.d,
            R8::E => &mut self.reg.e,
            R8::H => &mut self.reg.h,
            R8::L => &mut self.reg.l,
            R8::HLRam => self.mmu.memory.get_mut(self.reg.hl() as usize).unwrap(),
        }
    }

    pub fn set_reg(&mut self, index: u8, value: u8) {
        match index {
            0 => self.reg.b = value,
            1 => self.reg.c = value,
            2 => self.reg.d = value,
            3 => self.reg.e = value,
            4 => self.reg.h = value,
            5 => self.reg.l = value,
            6 => self.mmu.set(self.reg.hl(), value),
            7 => self.reg.a = value,
            _ => panic!("This is supposed to be unreachable"),
        }
    }

    pub fn push_word(&mut self, word: u16) {
        let (left, right) = bytes_from(word);
        self.mmu.set(self.reg.sp - 1, left);
        self.mmu.set(self.reg.sp - 2, right);
        self.reg.sp -= 2;
    }

    pub fn sub_u8(&mut self, byte: u8) {
        self.reg.f.compute_half_carry_sub(self.reg.a, byte);
        (self.reg.a, self.reg.f.carry) = u8::overflowing_sub(self.reg.a, byte);
        self.reg.f.sub = true;
        self.reg.f.zero = self.reg.a == 0;
    }

    pub fn sbc_u8(&mut self, byte: u8) {
        let old_cy = self.reg.f.carry as u8;
        let (r1, cy1) = u8::overflowing_sub(self.reg.a, byte);
        let (r2, cy2) = u8::overflowing_sub(r1, old_cy);
        let h1 = Flags::half_carry_sub_occurred(self.reg.a, byte);
        let h2 = Flags::half_carry_sub_occurred(r1, old_cy);
        self.reg.a = r2;
        self.reg.f.sub = true;
        self.reg.f.zero = self.reg.a == 0;
        self.reg.f.half_carry = h1 || h2;
        self.reg.f.carry = cy1 || cy2;
    }

    pub fn and_u8(&mut self, byte: u8) {
        self.reg.a &= byte;
        self.reg.f.clear();
        self.reg.f.zero = self.reg.a == 0;
        self.reg.f.half_carry = true;
    }

    pub fn xor_u8(&mut self, byte: u8) {
        self.reg.a ^= byte;
        self.reg.f.clear();
        self.reg.f.zero = self.reg.a == 0;
    }

    pub fn rlc(&mut self, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let b7 = (*reg & 0b10000000) >> 7;
        *reg <<= 1;
        *reg |= b7;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b7 != 0;
    }

    pub fn rrc(&mut self, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let b0 = *reg & 1;
        *reg >>= 1;
        *reg |= b0 << 7;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b0 != 0;
    }

    pub fn rl(&mut self, reg: R8) {
        let old_cy = self.reg.f.carry as u8;
        let reg = self.get_reg8_mut(reg);
        let b7 = (*reg & 0b10000000) >> 7;
        *reg <<= 1;
        *reg |= old_cy;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b7 != 0;
    }

    pub fn rr(&mut self, reg: R8) {
        let old_cy = self.reg.f.carry as u8;
        let reg = self.get_reg8_mut(reg);
        let b0 = *reg & 1;
        *reg >>= 1;
        *reg |= old_cy << 7;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b0 != 0;
    }

    pub fn sla(&mut self, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let b7 = (*reg & 0b10000000) >> 7;
        *reg <<= 1;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b7 != 0;
    }

    pub fn sra(&mut self, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let b0 = *reg & 1;
        *reg >>= 1;
        *reg |= b0 << 7;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b0 != 0;
    }

    pub fn swap(&mut self, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let left = *reg >> 4;
        let right = (*reg & 0xf) << 4;
        *reg = right | left;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = false;
    }

    pub fn srl(&mut self, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let b0 = *reg & 1;
        *reg >>= 1;
        self.reg.f.zero = *reg == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = b0 != 0;
    }

    pub fn bit(&mut self, index: u8, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        let bit = *reg & (1 << index);
        self.reg.f.zero = bit == 0;
        self.reg.f.sub = false;
        self.reg.f.half_carry = true;
    }

    pub fn res(&mut self, index: u8, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        unset_bit(reg, index);
    }

    pub fn set(&mut self, index: u8, reg: R8) {
        let reg = self.get_reg8_mut(reg);
        set_bit(reg, index);
    }

    pub fn generate_interrupt(&mut self, id: u8, interrupt_flag: u8) {
        self.ime = false;
        let cleared = interrupt_flag & (0b1111_1111 ^ (1 << id));
        self.mmu.set(0xFF0F, cleared);

        self.push_word(self.reg.pc);
        match id {
            0 => self.reg.pc = Interrupt::VBlank as u16,
            1 => self.reg.pc = Interrupt::LCD as u16,
            2 => self.reg.pc = Interrupt::TIMER as u16,
            4 => self.reg.pc = Interrupt::JOYPAD as u16,
            _ => ()
        }
    }

    pub fn generate_interrupts(&mut self) {
        if self.ime {
            let interrupt_flag = self.mmu.get(0xFF0F);
            let interrupt_enable = self.mmu.get(0xFFFF);

            if interrupt_flag > 0 { // Any interrupts are set
                for i in 0..5 {
                    if ((interrupt_flag & interrupt_enable) >> i) & 1 == 1 { // Interrupt 'i' is set
                        self.generate_interrupt(i, interrupt_flag);
                        break; // FIXME: Is this the correct behaviour?
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use crate::registers::R8;

    #[test]
    fn generate_interrupts_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.set(0xFF0F, 0b0000_1101);
        cpu.mmu.set(0xFFFF, 0b0001_0111);
        cpu.generate_interrupts();
        assert_eq!(cpu.mmu.get(0xFF0F), 0b0000_1100);
    }

    #[test]
    fn generate_interrupt_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.set(0xFF0F, 0b0000_0110);
        cpu.mmu.set(0xFFFF, 0b0001_1111);
        assert_eq!(cpu.reg.sp, 0xFFFE);
        cpu.generate_interrupt(1, cpu.mmu.get(0xFF0F));
        assert_eq!(cpu.mmu.get(0xFF0F), 0b0000_0100);
        assert_eq!(cpu.reg.pc, 0x48);
        assert_eq!(cpu.reg.sp, 0xFFFE - 2);
    }

    #[test]
    fn tick_advances_pc() {
        let mut cpu = Cpu::new();
        let initial_state = cpu.reg.pc;
        cpu.tick();
        assert_eq!(cpu.reg.pc, initial_state + 1);
    }

    #[test]
    fn rlc_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0b10101010;
        cpu.rlc(R8::D);
        assert_eq!(cpu.reg.d, 0b01010101);
    }
}