use super::cpu::{Cpu, NORMAL_TIMINGS, CB_TIMINGS};
use super::decode::decode;
use log::warn;
use crate::flags::Flags;
use crate::{word_from, LOGGING_ENABLED, set_bit, unset_bit, bytes_from};
use crate::registers::{R8, R16};

fn op_implemented(cpu: &Cpu) {
    if LOGGING_ENABLED && !cpu.mmu.bootrom_mapped { // TODO: Remove debug code
        // println!("I PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
        //     cpu.reg.pc, decode(cpu).expect("Unknown opcode"),
        //     cpu.reg.a, cpu.reg.f.to_string(), cpu.reg.b, cpu.reg.c, cpu.reg.d, cpu.reg.e, cpu.reg.h, cpu.reg.l, cpu.reg.sp,
        // );
        println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            cpu.reg.a, cpu.reg.f.as_u8(), cpu.reg.b, cpu.reg.c, cpu.reg.d, cpu.reg.e, cpu.reg.h, cpu.reg.l, cpu.reg.sp, cpu.reg.pc,
            cpu.get_op(0), cpu.get_op(1), cpu.get_op(2), cpu.get_op(3)
        );
    }
}

fn op_unimplemented(cpu: &mut Cpu) {
    if LOGGING_ENABLED { // TODO: Remove debug code
        warn!("U PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
            cpu.reg.pc, decode(cpu).expect(&format!("Unknown opcode : {:02X}", cpu.opcode)),
            cpu.reg.a, cpu.reg.f.to_string(), cpu.reg.b, cpu.reg.c, cpu.reg.d, cpu.reg.e, cpu.reg.h, cpu.reg.l, cpu.reg.sp,
        );
    }
    cpu._tmp_warn_count += 1;
}

#[allow(unreachable_patterns)]
pub fn execute(cpu: &mut Cpu) {
    if cpu.opcode == 0xCB {
        cpu.cb_prefix = true;
        cpu.opcode = cpu.mmu.get(cpu.reg.pc + 1);
        cpu.cycles = CB_TIMINGS[cpu.opcode as usize];
        cpu.advance_pc = 2; // Every CB instruction is 2 bytes

        // Get the appropriate register based on the instruction set layout
        let reg_no = (cpu.opcode & 0x0F) % 8;
        let reg = R8::from_spec(reg_no);

        // Set the appropriate cycles for instructions that use (HL)
        cpu.cycles = 2; // Most CB instructions take 2 cycles
        if reg == R8::HLRam {
            cpu.cycles = match cpu.opcode {
                0x40..=0x80 => 3,
                _ => 4
            }
        };

        op_implemented(cpu);
        match cpu.opcode {
            0x00..=0x07 => cpu.rlc(reg), // RLC
            0x08..=0x0F => cpu.rrc(reg), // RRC
            0x10..=0x17 => cpu.rl(reg), // RL
            0x18..=0x1F => cpu.rr(reg), // RR
            0x20..=0x27 => cpu.sla(reg), // SLA
            0x28..=0x2F => cpu.sra(reg), // SRA
            0x30..=0x37 => cpu.swap(reg), // SWAP
            0x38..=0x3F => cpu.srl(reg), // SRL
            0x40..=0x7F => {
                let bit_index = (cpu.opcode - 0x40) / 8;
                cpu.bit(bit_index, reg);
            } // BIT
            0x80..=0xBF => {
                let bit_index = (cpu.opcode - 0x80) / 8;
                cpu.res(bit_index, reg);
            } // RES
            0xC0..=0xFF => {
                let bit_index = (cpu.opcode - 0xC0) / 8;
                cpu.set(bit_index, reg);
            } // SET
            _ => op_unimplemented(cpu)
        }
    }
    else {
        cpu.cb_prefix = false;
        cpu.cycles = NORMAL_TIMINGS[cpu.opcode as usize];
        match cpu.opcode {
            0x76 => {
                op_unimplemented(cpu);
                cpu.set_op_attrs(1, 1);
            }
            0x40..=0x7F => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 1;
                let reg_1_no = (cpu.opcode - 0x40) / 0x08;
                let reg_2_no = (cpu.opcode & 0x0F) % 8;
                if reg_1_no == 6 || reg_2_no == 6 {
                    cpu.cycles = 2;
                }
                let value = cpu.get_reg8_by_index(reg_2_no);
                cpu.set_reg(reg_1_no, value);

            }, // LD r,r
            0x80..=0xBF => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 1;
                let op_no = (cpu.opcode - 0x80) / 0x08;
                let reg_2_no = (cpu.opcode & 0x0F) % 8;
                if reg_2_no == 6 {
                    cpu.cycles = 2;
                };
                let byte = cpu.get_reg8_by_index(reg_2_no);
                match op_no {
                    0 => add_a_u8(cpu, byte),
                    1 => adc_a_u8(cpu, byte),
                    2 => cpu.sub(byte),
                    3 => cpu.sbc(byte),
                    4 => cpu.and(byte),
                    5 => cpu.xor(byte),
                    6 => cpu.or(byte),
                    7 => cpu.cp(byte),
                    _ => ()
                };
            }, // ARITHMETIC r,r
            0xC6 | 0xD6 | 0xE6 | 0xF6 | 0xCE | 0xDE | 0xEE | 0xFE => {
                op_implemented(cpu);
                cpu.advance_pc = 2;
                cpu.cycles = 2;
                let d8 = cpu.get_op(1);
                match cpu.opcode {
                    0xC6 => add_a_u8(cpu, d8),
                    0xD6 => sub_u8(cpu, d8),
                    0xE6 => cpu.and(d8),
                    0xF6 => cpu.or(d8),
                    0xCE => adc_a_u8(cpu, d8),
                    0xDE => cpu.sbc(d8),
                    0xEE => cpu.xor(d8),
                    0xFE => cpu.cp(d8),
                    _ => ()
                }
            }, // ARITHMETIC r,d8
            0xC2 | 0xD2 | 0xCA | 0xDA => {
                op_implemented(cpu);
                cpu.advance_pc = 3;
                cpu.cycles = 3;
                let addr = cpu.get_d16();
                match cpu.opcode {
                    0xC2 => {
                        if !cpu.reg.f.zero {
                            cpu.advance_pc = 0;
                            cpu.cycles = 4;
                            cpu.reg.pc = addr;
                        }
                    },
                    0xD2 => {
                        if !cpu.reg.f.carry {
                            cpu.advance_pc = 0;
                            cpu.cycles = 4;
                            cpu.reg.pc = addr;
                        }
                    },
                    0xCA => {
                        if cpu.reg.f.zero {
                            cpu.advance_pc = 0;
                            cpu.cycles = 4;
                            cpu.reg.pc = addr;
                        }
                    },
                    0xDA => {
                        if cpu.reg.f.carry {
                            cpu.advance_pc = 0;
                            cpu.cycles = 4;
                            cpu.reg.pc = addr;
                        }
                    },
                    _ => ()
                }
            }, // CONDITIONAL JP
            0x01 | 0x11 | 0x21 | 0x31 => {
                op_implemented(cpu);
                cpu.advance_pc = 3;
                cpu.cycles = 3;
                let word = word_from(cpu.get_op(2), cpu.get_op(1));
                cpu.set_reg16_by_index((cpu.opcode & 0xF0) >> 4, word);
            }, // LD rr, d16
            0xC7 | 0xD7 | 0xE7 | 0xF7 | 0xCF | 0xDF | 0xEF | 0xFF => {
                op_implemented(cpu);
                cpu.advance_pc = 0; // Don't advance AFTER this instruction
                cpu.cycles = 4;
                cpu.push_word(cpu.reg.pc + 1); // Advance the return pointer by one
                cpu.reg.pc = (cpu.opcode - 0xC7) as u16;
            }, // RST
            0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 1;
                let index = (cpu.opcode - 0x04) / 8;
                let reg = R8::from_spec(index);
                if reg == R8::HLRam { cpu.cycles = 3; }
                cpu.inc(reg);
            }, // INC r
            0x03 | 0x13 | 0x23 | 0x33 => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 2;
                let index = (cpu.opcode - 0x03) / 16;
                let reg = R16::from_spec(index);
                cpu.inc_rr(reg);
            }, // INC rr
            0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 1;
                let index = (cpu.opcode - 0x05) / 8;
                let reg = R8::from_spec(index);
                if reg == R8::HLRam { cpu.cycles = 3; }
                cpu.dec(reg);
            }, // DEC r
            0x0B | 0x1B | 0x2B | 0x3B => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 2;
                let index = (cpu.opcode - 0x0B) / 16;
                let reg = R16::from_spec(index);
                cpu.dec_rr(reg);
            }, // INC rr
            0xC4 | 0xD4 | 0xCC | 0xDC => {
                op_implemented(cpu);
                cpu.advance_pc = 3;
                cpu.cycles = 3;
                match cpu.opcode {
                    0xC4 => {
                        if !cpu.reg.f.zero {
                            cpu.cycles = 6;
                            call_a16(cpu);
                        }
                    },
                    0xD4 => {
                        if !cpu.reg.f.carry {
                            cpu.cycles = 6;
                            call_a16(cpu);
                        }
                    },
                    0xCC => {
                        if cpu.reg.f.zero {
                            cpu.cycles = 6;
                            call_a16(cpu);
                        }
                    },
                    0xDC => {
                        if cpu.reg.f.carry {
                            cpu.cycles = 6;
                            call_a16(cpu);
                        }
                    },
                    _ => ()
                }
            }, // CONDITIONAL CALL
            0x00 => execute_00(cpu),
            0x02 => execute_02(cpu),
            0x06 => execute_06(cpu),
            0x07 => execute_07(cpu),
            0x08 => execute_08(cpu),
            0x09 => execute_09(cpu),
            0x0a => execute_0a(cpu),
            0x0e => execute_0e(cpu),
            0x0f => execute_0f(cpu),
            0x10 => execute_10(cpu),
            0x12 => execute_12(cpu),
            0x16 => execute_16(cpu),
            0x17 => execute_17(cpu),
            0x18 => execute_18(cpu),
            0x19 => execute_19(cpu),
            0x1a => execute_1a(cpu),
            0x1e => execute_1e(cpu),
            0x1f => execute_1f(cpu),
            0x20 => execute_20(cpu),
            0x22 => execute_22(cpu),
            0x26 => execute_26(cpu),
            0x27 => execute_27(cpu),
            0x28 => execute_28(cpu),
            0x29 => execute_29(cpu),
            0x2a => execute_2a(cpu),
            0x2e => execute_2e(cpu),
            0x2f => execute_2f(cpu),
            0x30 => execute_30(cpu),
            0x32 => execute_32(cpu),
            0x36 => execute_36(cpu),
            0x37 => execute_37(cpu),
            0x38 => execute_38(cpu),
            0x39 => execute_39(cpu),
            0x3a => execute_3a(cpu),
            0x3e => execute_3e(cpu),
            0x3f => execute_3f(cpu),
            0xc0 => execute_c0(cpu),
            0xc1 => execute_c1(cpu),
            0xc3 => execute_c3(cpu),
            0xc5 => execute_c5(cpu),
            0xc8 => execute_c8(cpu),
            0xc9 => execute_c9(cpu),
            0xcb => execute_cb(cpu),
            0xcd => execute_cd(cpu),
            0xd0 => execute_d0(cpu),
            0xd1 => execute_d1(cpu),
            0xd5 => execute_d5(cpu),
            0xd8 => execute_d8(cpu),
            0xd9 => execute_d9(cpu),
            0xe0 => execute_e0(cpu),
            0xe1 => execute_e1(cpu),
            0xe2 => execute_e2(cpu),
            0xe5 => execute_e5(cpu),
            0xe8 => execute_e8(cpu),
            0xe9 => execute_e9(cpu),
            0xea => execute_ea(cpu),
            0xf0 => execute_f0(cpu),
            0xf1 => execute_f1(cpu),
            0xf2 => execute_f2(cpu),
            0xf3 => execute_f3(cpu),
            0xf5 => execute_f5(cpu),
            0xf8 => execute_f8(cpu),
            0xf9 => execute_f9(cpu),
            0xfa => execute_fa(cpu),
            0xfb => execute_fb(cpu),
            _ => op_unimplemented(cpu)
        }
    }
}

fn add_a_u8(cpu: &mut Cpu, byte: u8) {
    cpu.reg.f.compute_half_carry_add(cpu.reg.a, byte);
    (cpu.reg.a, cpu.reg.f.carry) = u8::overflowing_add(cpu.reg.a, byte);
    cpu.reg.f.sub = false;
    cpu.reg.f.zero = cpu.reg.a == 0;
}

fn adc_a_u8(cpu: &mut Cpu, byte: u8) {
    let cy = cpu.reg.f.carry as u8;
    let (byte_plus_cy, c1) = u8::overflowing_add(byte, cy);
    let (result, c2) = u8::overflowing_add(cpu.reg.a, byte_plus_cy);
    let h1 = Flags::half_carry_add_occurred(byte, cy);
    let h2 = Flags::half_carry_add_occurred(cpu.reg.a, byte_plus_cy);
    cpu.reg.a = result;
    cpu.reg.f.sub = false;
    cpu.reg.f.zero = cpu.reg.a == 0;
    cpu.reg.f.carry = c1 || c2;
    cpu.reg.f.half_carry = h1 || h2;
}

fn add_hl_u16(cpu: &mut Cpu, word: u16) {
    cpu.reg.f.compute_half_carry_add_u16(cpu.reg.hl(), word);
    let (result, carry) = cpu.reg.hl().overflowing_add(word);
    cpu.reg.set_hl(result);
    cpu.reg.f.carry = carry;
    cpu.reg.f.sub = false;
}

fn sub_u8(cpu: &mut Cpu, byte: u8) {
    cpu.reg.f.compute_half_carry_sub(cpu.reg.a, byte);
    (cpu.reg.a, cpu.reg.f.carry) = u8::overflowing_sub(cpu.reg.a, byte);
    cpu.reg.f.sub = true;
    cpu.reg.f.zero = cpu.reg.a == 0;
}

fn pop_word(cpu: &mut Cpu) -> u16 {
    let left = cpu.mmu.get(cpu.reg.sp + 1);
    let right = cpu.mmu.get(cpu.reg.sp);
    cpu.reg.sp += 2;
    word_from(left, right)
}

fn call_a16(cpu: &mut Cpu) {
    // Store PC on stack
    cpu.push_word(cpu.reg.pc + cpu.advance_pc as u16);
    cpu.advance_pc = 0;
    // Set PC to address
    let left = cpu.get_op(1);
    let right = cpu.get_op(2);
    cpu.reg.pc = word_from(right, left);
}

fn ld_d8(reg: &mut u8, byte: u8) {
    *reg = byte;
}

fn ld_mem_d8(cpu: &mut Cpu, address: u16, byte: u8) {
    cpu.mmu.set(address, byte);
}

fn rl_d8(reg: &mut u8, flags: &mut Flags) {
    let b7 = (*reg & 0b10000000) >> 7;
    *reg <<= 1;
    *reg |= flags.carry as u8;
    flags.clear();
    flags.carry = b7 != 0;
}

pub fn rr(reg: &mut u8, flags: &mut Flags) {
    let b0 = *reg & 1;
    *reg >>= 1;
    if flags.carry {
        set_bit(reg, 7);
    } else {
        unset_bit(reg, 7);
    }
    flags.clear();
    flags.zero = *reg == 0;
    flags.carry = b0 != 0;
}

pub fn srl(reg: &mut u8, flags: &mut Flags) {
    let b0 = *reg & 1;
    *reg >>= 1;
    *reg |= flags.carry as u8;
    flags.clear();
    flags.zero = *reg == 0;
    flags.carry = b0 != 0;
}

fn execute_00(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
} // NOP  [-/-/-/-]
fn execute_02(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
} // LD (BC) A [-/-/-/-]
fn execute_06(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.b, byte);
} // LD B d8 [-/-/-/-]
fn execute_07(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.rlc(R8::A);
    cpu.reg.f.zero = false;
} // RLCA  [0/0/0/C]
fn execute_08(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles = 5;
    let addr = cpu.get_d16();
    let (hi, lo) = bytes_from(cpu.reg.sp);
    cpu.mmu.set(addr, lo);
    cpu.mmu.set(addr + 1, hi);
} // LD (a16) SP [-/-/-/-]
fn execute_09(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    add_hl_u16(cpu, cpu.reg.bc());
} // ADD HL BC [-/0/H/C]
fn execute_0a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
} // LD A (BC) [-/-/-/-]
fn execute_0e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.mmu.get(cpu.reg.pc + 1);
    ld_d8(&mut cpu.reg.c, byte);
} // LD C d8 [-/-/-/-]
fn execute_0f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.rrc(R8::A);
    cpu.reg.f.zero = false;
} // RRCA  [0/0/0/C]
fn execute_10(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
} // STOP 0  [-/-/-/-]
fn execute_12(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    ld_mem_d8(cpu, cpu.reg.de(), cpu.reg.a);
} // LD (DE) A [-/-/-/-]
fn execute_16(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.d, byte);
} // LD D d8 [-/-/-/-]
fn execute_17(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    rl_d8(&mut cpu.reg.a, &mut cpu.reg.f);
} // RLA  [0/0/0/C]
fn execute_18(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 3;
    let s8 = cpu.get_op(1) as i8;
    cpu.advance_pc += s8 as i16;
} // JR r8  [-/-/-/-]
fn execute_19(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    add_hl_u16(cpu, cpu.reg.de());
} // ADD HL DE [-/0/H/C]
fn execute_1a(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    cpu.reg.a = cpu.mmu.get(cpu.reg.de());
} // LD A (DE) [-/-/-/-]
fn execute_1e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.e, byte);
} // LD E d8 [-/-/-/-]
fn execute_1f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    rr(&mut cpu.reg.a, &mut cpu.reg.f);
    cpu.reg.f.zero = false;
} // RRA  [0/0/0/C]
fn execute_20(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    if !cpu.reg.f.zero {
        let s8 = cpu.get_op(1) as i8;
        cpu.advance_pc += s8 as i16;
        cpu.cycles = 3;
    }
} // JR NZ r8 [-/-/-/-]
fn execute_22(cpu: &mut Cpu) {
    // FIXME: Might require flags being set
    // FIXME: Increment before or after?
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    ld_mem_d8(cpu, cpu.reg.hl(), cpu.reg.a);
    cpu.reg.inc_hl_nf();
} // LD (HL+) A [-/-/-/-]
fn execute_26(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.h, byte);
} // LD H d8 [-/-/-/-]
fn execute_27(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    let mut correction = 0;
    let a = cpu.reg.a;

    // To calculate BCD we add 6 if the lo nibble is greater than 9
    // I.e. [b]0000_1100 (12) + 0110 (6) = [bcd]0001_0010 (12)
    if cpu.reg.f.half_carry || (!cpu.reg.f.sub && (a & 0xf) > 9) {
        correction |= 0x6;
    }
    // We also add 0x60 if the accumulator is greater than 0x99
    if cpu.reg.f.carry || (!cpu.reg.f.sub && a > 0x99) {
        correction |= 0x60;
        cpu.reg.f.carry = true;
    }

    // If a sub occurred prior to DAA then sub the correction, else add the correction
    if cpu.reg.f.sub {
        cpu.reg.a = u8::wrapping_sub(a, correction);
    } else {
        cpu.reg.a = u8::wrapping_add(a, correction);
    }
    cpu.reg.f.zero = cpu.reg.a == 0;
    cpu.reg.f.half_carry = false;
} // DAA  [Z/-/0/C]
fn execute_28(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    if cpu.reg.f.zero {
        let s8 = cpu.get_op(1) as i8;
        cpu.advance_pc += s8 as i16;
        cpu.cycles = 3;
    }
} // JR Z r8 [-/-/-/-]
fn execute_29(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    add_hl_u16(cpu, cpu.reg.hl());
} // ADD HL HL [-/0/H/C]
fn execute_2a(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    let byte = cpu.mmu.get(cpu.reg.hl());
    ld_d8(&mut cpu.reg.a, byte);
    cpu.reg.inc_hl_nf();
} // LD A (HL+) [-/-/-/-]
fn execute_2e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.l, byte);
} // LD L d8 [-/-/-/-]
fn execute_2f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.reg.a = !cpu.reg.a;
    cpu.reg.f.sub = true;
    cpu.reg.f.half_carry = true;
} // CPL  [-/1/1/-]
fn execute_30(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    if !cpu.reg.f.carry {
        let s8 = cpu.get_op(1) as i8;
        cpu.advance_pc += s8 as i16;
        cpu.cycles = 3;
    }
} // JR NC r8 [-/-/-/-]
fn execute_32(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    cpu.mmu.set(cpu.reg.hl(), cpu.reg.a);
    cpu.reg.dec_hl_nf(); // FIXME: THis is on the right track for fixing the corrupted logo
} // LD (HL-) A [-/-/-/-]
fn execute_36(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 3;
    let byte = cpu.get_op(1);
    ld_mem_d8(cpu, cpu.reg.hl(), byte);
} // LD (HL) d8 [-/-/-/-]
fn execute_37(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.reg.f.sub = false;
    cpu.reg.f.half_carry = false;
    cpu.reg.f.carry = true;
} // SCF  [-/0/0/1]
fn execute_38(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    if cpu.reg.f.carry {
        cpu.cycles = 3;
        cpu.advance_pc += cpu.get_op(1) as i16;
    }
} // JR C r8 [-/-/-/-]
fn execute_39(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    add_hl_u16(cpu, cpu.reg.sp);
} // ADD HL SP [-/0/H/C]
fn execute_3a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
} // LD A (HL-) [-/-/-/-]
fn execute_3e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.a, byte);
} // LD A d8 [-/-/-/-]
fn execute_3f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.reg.f.sub = false;
    cpu.reg.f.half_carry = false;
    cpu.reg.f.carry = !cpu.reg.f.carry;
} // CCF  [-/0/0/C]
fn execute_c0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    if !cpu.reg.f.zero {
        cpu.advance_pc = 0;
        cpu.cycles = 5;
        cpu.reg.pc = pop_word(cpu);
    }
} // RET NZ  [-/-/-/-]
fn execute_c1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 3;
    let word = pop_word(cpu);
    cpu.reg.set_bc(word);
} // POP BC  [-/-/-/-]
fn execute_c3(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles = 4;
    let (left, right) = (cpu.get_op(1), cpu.get_op(2));
    cpu.reg.pc = word_from(right, left);
} // JP a16  [-/-/-/-]
fn execute_c5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 4;
    cpu.push_word(cpu.reg.bc());
} // PUSH BC  [-/-/-/-]
fn execute_c8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    if cpu.reg.f.zero {
        cpu.advance_pc = 0;
        cpu.cycles = 5;
        cpu.reg.pc = pop_word(cpu);
    }
} // RET Z  [-/-/-/-]
fn execute_c9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles = 4;
    cpu.reg.pc = pop_word(cpu);
} // RET  [-/-/-/-]
fn execute_cb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
} // PREFIX CB  [-/-/-/-]
fn execute_cd(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles = 6;
    call_a16(cpu);
} // CALL a16  [-/-/-/-]
fn execute_d0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    if !cpu.reg.f.carry {
        cpu.advance_pc = 0;
        cpu.cycles = 5;
        cpu.reg.pc = pop_word(cpu);
    }
} // RET NC  [-/-/-/-]
fn execute_d1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 3;
    let word = pop_word(cpu);
    cpu.reg.set_de(word);
} // POP DE  [-/-/-/-]
fn execute_d5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 4;
    cpu.push_word(cpu.reg.de());
} // PUSH DE  [-/-/-/-]
fn execute_d8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    if cpu.reg.f.carry {
        cpu.advance_pc = 0;
        cpu.cycles = 5;
        cpu.reg.pc = pop_word(cpu);
    }
} // RET C  [-/-/-/-]
fn execute_d9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles = 4;
    cpu.reg.pc = pop_word(cpu);
} // RETI  [-/-/-/-]
fn execute_e0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 3;
    let address = word_from(0xFF, cpu.get_op(1));
    ld_mem_d8(cpu, address, cpu.reg.a);
} // LDH (a8) A [-/-/-/-]
fn execute_e1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 3;
    let word = pop_word(cpu);
    cpu.reg.set_hl(word);
} // POP HL  [-/-/-/-]
fn execute_e2(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    let address = word_from(0xFF, cpu.reg.c);
    cpu.mmu.set(address, cpu.reg.a);
} // LD (C) A [-/-/-/-]
fn execute_e5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 4;
    cpu.push_word(cpu.reg.hl());
} // PUSH HL  [-/-/-/-]
fn execute_e8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 4;
    let s8 = cpu.get_op(1) as i8;
    cpu.add_sp_s8(s8);
} // ADD SP r8 [0/0/H/C]
fn execute_e9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles = 1;
    cpu.reg.pc = cpu.reg.hl();
} // JP (HL)  [-/-/-/-]
fn execute_ea(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles = 4;
    let address = word_from(cpu.get_op(2), cpu.get_op(1));
    ld_mem_d8(cpu, address, cpu.reg.a);
} // LD (a16) A [-/-/-/-]
fn execute_f0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 3;
    let address = word_from(0xFF, cpu.get_op(1));
    cpu.reg.a = cpu.mmu.get(address);
} // LDH A (a8) [-/-/-/-]
fn execute_f1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 3;
    let word = pop_word(cpu);
    cpu.reg.set_af(word);
} // POP AF  [Z/N/H/C]
fn execute_f2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
} // LD A (C) [-/-/-/-]
fn execute_f3(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.ime = false;
} // DI  [-/-/-/-]
fn execute_f5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 4;
    cpu.push_word(cpu.reg.af());
} // PUSH AF  [-/-/-/-]
fn execute_f8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 3;
    let old_sp = cpu.reg.sp;
    let s8 = cpu.get_op(1) as i8;
    cpu.add_sp_s8(s8);
    cpu.reg.set_hl(cpu.reg.sp);
    cpu.reg.sp = old_sp;
} // LD HL SP+r8 [0/0/H/C]
fn execute_f9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    cpu.reg.sp = cpu.reg.hl();
} // LD SP HL [-/-/-/-]
fn execute_fa(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles = 4;
    let address = word_from(cpu.get_op(2), cpu.get_op(1));
    ld_d8(&mut cpu.reg.a, cpu.mmu.get(address));
} // LD A (a16) [-/-/-/-]
fn execute_fb(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 1;
    cpu.ime = true;
} // EI  [-/-/-/-]

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use crate::execute::*;

    #[test]
    fn push_pop_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.reset();
        cpu.reg.set_bc(0xABCD);
        let word = cpu.reg.bc();
        cpu.push_word(word);
        cpu.reg.set_bc(0xBEEF);
        assert_eq!(cpu.reg.bc(), 0xBEEF);
        let word = pop_word(&mut cpu);
        cpu.reg.set_bc(word);
        assert_eq!(cpu.reg.bc(), 0xABCD);
    }

    #[test]
    fn swap_u8_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0b1100_1001;
        swap_u8(&mut cpu.reg.e, &mut cpu.reg.f);
        assert_eq!(cpu.reg.e, 0b1001_1100);
    }

    #[test]
    fn cp_d8_match() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0b01001001;
        cpu.reg.a = 0b01001001;
        let byte = cpu.reg.b;
        cp_d8(&mut cpu, byte);
        assert!(cpu.reg.f.zero);
        assert!(!cpu.reg.f.carry);
        assert!(cpu.reg.f.sub);
        assert!(!cpu.reg.f.half_carry);
    }

    #[test]
    fn rl_d8_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.f.carry = true;
        cpu.reg.b = 0b01101010;
        rl_d8(&mut cpu.reg.b, &mut cpu.reg.f);
        assert_eq!(cpu.reg.b, 0b11010101);
        assert!(!cpu.reg.f.carry);
    }

    #[test]
    fn dec_d8_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 1;
        dec_d8(&mut cpu.reg.b, &mut cpu.reg.f);
        assert_eq!(cpu.reg.b, 0);
        assert!(cpu.reg.f.zero);
        assert!(!cpu.reg.f.carry);
        assert!(cpu.reg.f.sub);
    }

    #[test]
    fn execute_0c_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x20;
        execute_0c(&mut cpu);
        assert_eq!(cpu.reg.c, 0x21);
    }

    #[test]
    fn execute_20_no_jmp() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x20, 0x06];
        cpu.reg.f.zero = true;
        execute_20(&mut cpu);
        assert_eq!(cpu.advance_pc, 2);
    }

    #[test]
    fn execute_32_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x32];
        cpu.reg.set_hl(0x9fff);
        cpu.reg.a = 0xBB;
        execute_32(&mut cpu);
        assert_eq!(cpu.mmu.get(0x9fff), 0xBB);
        assert_eq!(cpu.reg.hl(), 0x9ffe);
    }

    #[test]
    fn execute_af_zero() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0xAF];
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.f.zero, false);
        execute_af(&mut cpu);
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.f.zero, true);
    }

    #[test]
    fn execute_af_not_zero() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0xAF];
        cpu.reg.a = 32;
        assert_eq!(cpu.reg.f.zero, false);
        execute_af(&mut cpu);
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.f.zero, true);
    }
}