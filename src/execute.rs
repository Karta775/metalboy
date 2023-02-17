use super::cpu::Cpu;
use super::decode::decode;
use log::warn;
use crate::flags::Flags;
use crate::{word_from, LOGGING_ENABLED, set_bit, unset_bit};
use crate::registers::{R8::*, R16::*, Registers, R8};

fn op_implemented(cpu: &Cpu) {
    if LOGGING_ENABLED && !cpu.mmu.bootrom_mapped { // TODO: Remove debug code
        // debug!("I PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
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
        cpu.advance_pc = 2; // Every CB instruction is 2 bytes

        // Get the appropriate register based on the instruction set layout
        let reg_no = (cpu.opcode & 0x0F) % 8;
        let reg = R8::from_spec(reg_no);

        // Set the appropriate cycles for instructions that use (HL)
        cpu.cycles = 2; // Most CB instructions take 2 cycles
        if reg == HLRam {
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
                let bit_index = (cpu.opcode - 0x80) / 8;
                cpu.set(bit_index, reg);
            } // SET
            _ => op_unimplemented(cpu)
        }
    }
    else {
        cpu.cb_prefix = false;
        match cpu.opcode {
            0x40..=0x7F => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 1;
                let reg_1_no = (cpu.opcode - 0x40) / 0x08;
                let reg_2_no = (cpu.opcode & 0x0F) % 8;
                let value = cpu.get_reg(reg_2_no);
                cpu.set_reg(reg_1_no, value);
                if reg_1_no == 6 || reg_2_no == 6 {
                    cpu.cycles = 2;
                }
            }, // LD r,r
            0x80..=0xBF => {
                op_implemented(cpu);
                cpu.advance_pc = 1;
                cpu.cycles = 1;
                let op_no = (cpu.opcode - 0x80) / 0x08;
                let reg_2_no = (cpu.opcode & 0x0F) % 8;
                let byte = cpu.get_reg(reg_2_no);
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
                if reg_2_no == 6 {
                    cpu.cycles = 2;
                };
            }, // ARITHMETIC r,r
            0xC6 | 0xD6 | 0xE6 | 0xF6 => {
                op_implemented(cpu);
                cpu.advance_pc = 2;
                cpu.cycles = 2;
                let d8 = cpu.get_op(1);
                match cpu.opcode & 0xF0 {
                    0xC0 => add_a_u8(cpu, d8),
                    0xD0 => sub_u8(cpu, d8),
                    0xE0 => cpu.and(d8),
                    0xF0 => cpu.or(d8),
                    _ => ()
                }
            }, // ARITHMETIC r,d8
            0x00 => execute_00(cpu),
            0x01 => execute_01(cpu),
            0x02 => execute_02(cpu),
            0x03 => execute_03(cpu),
            0x04 => execute_04(cpu),
            0x05 => execute_05(cpu),
            0x06 => execute_06(cpu),
            0x07 => execute_07(cpu),
            0x08 => execute_08(cpu),
            0x09 => execute_09(cpu),
            0x0a => execute_0a(cpu),
            0x0b => execute_0b(cpu),
            0x0c => execute_0c(cpu),
            0x0d => execute_0d(cpu),
            0x0e => execute_0e(cpu),
            0x0f => execute_0f(cpu),
            0x10 => execute_10(cpu),
            0x11 => execute_11(cpu),
            0x12 => execute_12(cpu),
            0x13 => execute_13(cpu),
            0x14 => execute_14(cpu),
            0x15 => execute_15(cpu),
            0x16 => execute_16(cpu),
            0x17 => execute_17(cpu),
            0x18 => execute_18(cpu),
            0x19 => execute_19(cpu),
            0x1a => execute_1a(cpu),
            0x1b => execute_1b(cpu),
            0x1c => execute_1c(cpu),
            0x1d => execute_1d(cpu),
            0x1e => execute_1e(cpu),
            0x1f => execute_1f(cpu),
            0x20 => execute_20(cpu),
            0x21 => execute_21(cpu),
            0x22 => execute_22(cpu),
            0x23 => execute_23(cpu),
            0x24 => execute_24(cpu),
            0x25 => execute_25(cpu),
            0x26 => execute_26(cpu),
            0x27 => execute_27(cpu),
            0x28 => execute_28(cpu),
            0x29 => execute_29(cpu),
            0x2a => execute_2a(cpu),
            0x2b => execute_2b(cpu),
            0x2c => execute_2c(cpu),
            0x2d => execute_2d(cpu),
            0x2e => execute_2e(cpu),
            0x2f => execute_2f(cpu),
            0x30 => execute_30(cpu),
            0x31 => execute_31(cpu),
            0x32 => execute_32(cpu),
            0x33 => execute_33(cpu),
            0x34 => execute_34(cpu),
            0x35 => execute_35(cpu),
            0x36 => execute_36(cpu),
            0x37 => execute_37(cpu),
            0x38 => execute_38(cpu),
            0x39 => execute_39(cpu),
            0x3a => execute_3a(cpu),
            0x3b => execute_3b(cpu),
            0x3c => execute_3c(cpu),
            0x3d => execute_3d(cpu),
            0x3e => execute_3e(cpu),
            0x3f => execute_3f(cpu),
            0x80 => execute_80(cpu),
            0x81 => execute_81(cpu),
            0x82 => execute_82(cpu),
            0x83 => execute_83(cpu),
            0x84 => execute_84(cpu),
            0x85 => execute_85(cpu),
            0x86 => execute_86(cpu),
            0x87 => execute_87(cpu),
            0x88 => execute_88(cpu),
            0x89 => execute_89(cpu),
            0x8a => execute_8a(cpu),
            0x8b => execute_8b(cpu),
            0x8c => execute_8c(cpu),
            0x8d => execute_8d(cpu),
            0x8e => execute_8e(cpu),
            0x8f => execute_8f(cpu),
            0x90 => execute_90(cpu),
            0x91 => execute_91(cpu),
            0x92 => execute_92(cpu),
            0x93 => execute_93(cpu),
            0x94 => execute_94(cpu),
            0x95 => execute_95(cpu),
            0x96 => execute_96(cpu),
            0x97 => execute_97(cpu),
            0x98 => execute_98(cpu),
            0x99 => execute_99(cpu),
            0x9a => execute_9a(cpu),
            0x9b => execute_9b(cpu),
            0x9c => execute_9c(cpu),
            0x9d => execute_9d(cpu),
            0x9e => execute_9e(cpu),
            0x9f => execute_9f(cpu),
            0xa0 => execute_a0(cpu),
            0xa1 => execute_a1(cpu),
            0xa2 => execute_a2(cpu),
            0xa3 => execute_a3(cpu),
            0xa4 => execute_a4(cpu),
            0xa5 => execute_a5(cpu),
            0xa6 => execute_a6(cpu),
            0xa7 => execute_a7(cpu),
            0xa8 => execute_a8(cpu),
            0xa9 => execute_a9(cpu),
            0xaa => execute_aa(cpu),
            0xab => execute_ab(cpu),
            0xac => execute_ac(cpu),
            0xad => execute_ad(cpu),
            0xae => execute_ae(cpu),
            0xaf => execute_af(cpu),
            0xb0 => execute_b0(cpu),
            0xb1 => execute_b1(cpu),
            0xb2 => execute_b2(cpu),
            0xb3 => execute_b3(cpu),
            0xb4 => execute_b4(cpu),
            0xb5 => execute_b5(cpu),
            0xb6 => execute_b6(cpu),
            0xb7 => execute_b7(cpu),
            0xb8 => execute_b8(cpu),
            0xb9 => execute_b9(cpu),
            0xba => execute_ba(cpu),
            0xbb => execute_bb(cpu),
            0xbc => execute_bc(cpu),
            0xbd => execute_bd(cpu),
            0xbe => execute_be(cpu),
            0xbf => execute_bf(cpu),
            0xc0 => execute_c0(cpu),
            0xc1 => execute_c1(cpu),
            0xc2 => execute_c2(cpu),
            0xc3 => execute_c3(cpu),
            0xc4 => execute_c4(cpu),
            0xc5 => execute_c5(cpu),
            0xc6 => execute_c6(cpu),
            0xc7 => execute_c7(cpu),
            0xc8 => execute_c8(cpu),
            0xc9 => execute_c9(cpu),
            0xca => execute_ca(cpu),
            0xcb => execute_cb(cpu),
            0xcc => execute_cc(cpu),
            0xcd => execute_cd(cpu),
            0xce => execute_ce(cpu),
            0xcf => execute_cf(cpu),
            0xd0 => execute_d0(cpu),
            0xd1 => execute_d1(cpu),
            0xd2 => execute_d2(cpu),
            0xd4 => execute_d4(cpu),
            0xd5 => execute_d5(cpu),
            0xd6 => execute_d6(cpu),
            0xd7 => execute_d7(cpu),
            0xd8 => execute_d8(cpu),
            0xd9 => execute_d9(cpu),
            0xda => execute_da(cpu),
            0xdc => execute_dc(cpu),
            0xde => execute_de(cpu),
            0xdf => execute_df(cpu),
            0xe0 => execute_e0(cpu),
            0xe1 => execute_e1(cpu),
            0xe2 => execute_e2(cpu),
            0xe5 => execute_e5(cpu),
            0xe6 => execute_e6(cpu),
            0xe7 => execute_e7(cpu),
            0xe8 => execute_e8(cpu),
            0xe9 => execute_e9(cpu),
            0xea => execute_ea(cpu),
            0xee => execute_ee(cpu),
            0xef => execute_ef(cpu),
            0xf0 => execute_f0(cpu),
            0xf1 => execute_f1(cpu),
            0xf2 => execute_f2(cpu),
            0xf3 => execute_f3(cpu),
            0xf5 => execute_f5(cpu),
            0xf6 => execute_f6(cpu),
            0xf7 => execute_f7(cpu),
            0xf8 => execute_f8(cpu),
            0xf9 => execute_f9(cpu),
            0xfa => execute_fa(cpu),
            0xfb => execute_fb(cpu),
            0xfe => execute_fe(cpu),
            0xff => execute_ff(cpu),
            _ => op_unimplemented(cpu)
        }
    }
}

// TODO: Consider renaming these (i.e. d8 might mean direct)
fn inc_d8(reg: &mut u8, flags: &mut Flags) {
    flags.compute_half_carry_add(*reg, 1);
    *reg = u8::wrapping_add(*reg, 1);
    flags.sub = false;
    flags.zero = *reg == 0;
}

fn dec_d8(reg: &mut u8, flags: &mut Flags) {
    flags.compute_half_carry_sub(*reg, 1);
    *reg = u8::wrapping_sub(*reg, 1);
    flags.sub = true;
    flags.zero = *reg == 0;
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

fn cp_d8(cpu: &mut Cpu, byte: u8) {
    cpu.reg.f.compute_half_carry_sub(cpu.reg.a, byte);
    let (result, carry) = cpu.reg.a.overflowing_sub(byte);
    cpu.reg.f.zero = result == 0;
    cpu.reg.f.sub = true;
    cpu.reg.f.carry = carry;
}

fn or_u8(cpu: &mut Cpu, byte: u8) {
    cpu.reg.a |= byte;
    cpu.reg.f.clear();
    cpu.reg.f.zero = cpu.reg.a == 0;
}

fn swap_u8(reg: &mut u8, flags: &mut Flags) {
    let left = *reg >> 4;
    let right = (*reg & 0xf) << 4;
    *reg = right | left;
    flags.clear();
    flags.zero = *reg == 0;
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
    cpu.cycles += 1;
} // NOP  [-/-/-/-]
fn execute_01(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
    let (left, right) = (cpu.get_op(2), cpu.get_op(1));
    cpu.reg.set_bc(word_from(left, right));
} // LD BC d16 [-/-/-/-]
fn execute_02(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (BC) A [-/-/-/-]
fn execute_03(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_bc(cpu.reg.bc().wrapping_add(1));
} // INC BC  [-/-/-/-]
fn execute_04(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.b, &mut cpu.reg.f);
} // INC B  [Z/0/H/-]
fn execute_05(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.b, &mut cpu.reg.f);
} // DEC B  [Z/1/H/-]
fn execute_06(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.b, byte);
} // LD B d8 [-/-/-/-]
fn execute_07(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.rlc(A);
    cpu.reg.f.zero = false;
} // RLCA  [0/0/0/C]
fn execute_08(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 5;
} // LD (a16) SP [-/-/-/-]
fn execute_09(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    add_hl_u16(cpu, cpu.reg.bc());
} // ADD HL BC [-/0/H/C]
fn execute_0a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (BC) [-/-/-/-]
fn execute_0b(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_bc(cpu.reg.bc().wrapping_sub(1));
} // DEC BC  [-/-/-/-]
fn execute_0c(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.c, &mut cpu.reg.f);
} // INC C  [Z/0/H/-]
fn execute_0d(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.c, &mut cpu.reg.f);
} // DEC C  [Z/1/H/-]
fn execute_0e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.mmu.get(cpu.reg.pc + 1);
    ld_d8(&mut cpu.reg.c, byte);
} // LD C d8 [-/-/-/-]
fn execute_0f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.rrc(A);
    cpu.reg.f.zero = false;
} // RRCA  [0/0/0/C]
fn execute_10(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // STOP 0  [-/-/-/-]
fn execute_11(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
    cpu.reg.d = cpu.mmu.get(cpu.reg.pc + 2);
    cpu.reg.e = cpu.mmu.get(cpu.reg.pc + 1);
} // LD DE d16 [-/-/-/-]
fn execute_12(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    ld_mem_d8(cpu, cpu.reg.de(), cpu.reg.a);
} // LD (DE) A [-/-/-/-]
fn execute_13(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_de(cpu.reg.de().wrapping_add(1));
} // INC DE  [-/-/-/-]
fn execute_14(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.d, &mut cpu.reg.f);
} // INC D  [Z/0/H/-]
fn execute_15(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.d, &mut cpu.reg.f);
} // DEC D  [Z/1/H/-]
fn execute_16(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.d, byte);
} // LD D d8 [-/-/-/-]
fn execute_17(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    rl_d8(&mut cpu.reg.a, &mut cpu.reg.f);
} // RLA  [0/0/0/C]
fn execute_18(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
    let s8 = cpu.get_op(1) as i8;
    cpu.advance_pc += s8 as i16;
} // JR r8  [-/-/-/-]
fn execute_19(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    add_hl_u16(cpu, cpu.reg.de());
} // ADD HL DE [-/0/H/C]
fn execute_1a(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.a = cpu.mmu.get(cpu.reg.de());
} // LD A (DE) [-/-/-/-]
fn execute_1b(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_de(cpu.reg.de().wrapping_sub(1));
} // DEC DE  [-/-/-/-]
fn execute_1c(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.e, &mut cpu.reg.f);
} // INC E  [Z/0/H/-]
fn execute_1d(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.e, &mut cpu.reg.f);
} // DEC E  [Z/1/H/-]
fn execute_1e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.e, byte);
} // LD E d8 [-/-/-/-]
fn execute_1f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
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
fn execute_21(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
    cpu.reg.h = cpu.get_op(2);
    cpu.reg.l = cpu.get_op(1);
} // LD HL d16 [-/-/-/-]
fn execute_22(cpu: &mut Cpu) {
    // FIXME: Might require flags being set
    // FIXME: Increment before or after?
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    ld_mem_d8(cpu, cpu.reg.hl(), cpu.reg.a);
    cpu.reg.inc_hl_nf();
} // LD (HL+) A [-/-/-/-]
fn execute_23(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.inc_hl_nf();
} // INC HL  [-/-/-/-]
fn execute_24(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.h, &mut cpu.reg.f);
} // INC H  [Z/0/H/-]
fn execute_25(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.h, &mut cpu.reg.f);
} // DEC H  [Z/1/H/-]
fn execute_26(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.h, byte);
} // LD H d8 [-/-/-/-]
fn execute_27(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
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
    cpu.cycles += 2;
    add_hl_u16(cpu, cpu.reg.hl());
} // ADD HL HL [-/0/H/C]
fn execute_2a(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    let byte = cpu.mmu.get(cpu.reg.hl());
    ld_d8(&mut cpu.reg.a, byte);
    cpu.reg.inc_hl_nf();
} // LD A (HL+) [-/-/-/-]
fn execute_2b(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_hl(cpu.reg.hl().wrapping_sub(1));
} // DEC HL  [-/-/-/-]
fn execute_2c(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.l, &mut cpu.reg.f);
} // INC L  [Z/0/H/-]
fn execute_2d(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.l, &mut cpu.reg.f);
} // DEC L  [Z/1/H/-]
fn execute_2e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.l, byte);
} // LD L d8 [-/-/-/-]
fn execute_2f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
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
fn execute_31(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
    let byte1 = cpu.mmu.get(cpu.reg.pc + 1);
    let byte2 = cpu.mmu.get(cpu.reg.pc + 2);
    cpu.reg.sp = word_from(byte2, byte1);
} // LD SP d16 [-/-/-/-]
fn execute_32(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.mmu.set(cpu.reg.hl(), cpu.reg.a);
    cpu.reg.dec_hl_nf(); // FIXME: THis is on the right track for fixing the corrupted logo
} // LD (HL-) A [-/-/-/-]
fn execute_33(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // INC SP  [-/-/-/-]
fn execute_34(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
} // INC (HL)  [Z/0/H/-]
fn execute_35(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
    let mut byte = cpu.mmu.get(cpu.reg.hl());
    dec_d8(&mut byte, &mut cpu.reg.f);
    cpu.mmu.set(cpu.reg.hl(), byte);
} // DEC (HL)  [Z/1/H/-]
fn execute_36(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
    let byte = cpu.get_op(1);
    ld_mem_d8(cpu, cpu.reg.hl(), byte);
} // LD (HL) d8 [-/-/-/-]
fn execute_37(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.reg.f.sub = false;
    cpu.reg.f.half_carry = false;
    cpu.reg.f.carry = true;
} // SCF  [-/0/0/1]
fn execute_38(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles = 2;
    if cpu.reg.f.carry {
        cpu.advance_pc = 0;
        cpu.cycles = 3;
        cpu.advance_pc += cpu.get_op(1) as i16;
    }
} // JR C r8 [-/-/-/-]
fn execute_39(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    add_hl_u16(cpu, cpu.reg.sp);
} // ADD HL SP [-/0/H/C]
fn execute_3a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (HL-) [-/-/-/-]
fn execute_3b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // DEC SP  [-/-/-/-]
fn execute_3c(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.a, &mut cpu.reg.f);
} // INC A  [Z/0/H/-]
fn execute_3d(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    dec_d8(&mut cpu.reg.a, &mut cpu.reg.f);
} // DEC A  [Z/1/H/-]
fn execute_3e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    ld_d8(&mut cpu.reg.a, byte);
} // LD A d8 [-/-/-/-]
fn execute_3f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.reg.f.sub = false;
    cpu.reg.f.half_carry = false;
    cpu.reg.f.carry = !cpu.reg.f.carry;
} // CCF  [-/0/0/C]
fn execute_80(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.b);
} // ADD A B [Z/0/H/C]
fn execute_81(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.c);
} // ADD A C [Z/0/H/C]
fn execute_82(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.d);
} // ADD A D [Z/0/H/C]
fn execute_83(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.e);
} // ADD A E [Z/0/H/C]
fn execute_84(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.h);
} // ADD A H [Z/0/H/C]
fn execute_85(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.l);
} // ADD A L [Z/0/H/C]
fn execute_86(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    let byte = cpu.mmu.get(cpu.reg.hl());
    add_a_u8(cpu, byte);
} // ADD A (HL) [Z/0/H/C]
fn execute_87(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    add_a_u8(cpu, cpu.reg.a);
} // ADD A A [Z/0/H/C]
fn execute_88(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.b);
} // ADC A B [Z/0/H/C]
fn execute_89(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.c);
} // ADC A C [Z/0/H/C]
fn execute_8a(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.d);
} // ADC A D [Z/0/H/C]
fn execute_8b(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.e);
} // ADC A E [Z/0/H/C]
fn execute_8c(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.h);
} // ADC A H [Z/0/H/C]
fn execute_8d(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.l);
} // ADC A L [Z/0/H/C]
fn execute_8e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    adc_a_u8(cpu, cpu.mmu.get(cpu.reg.hl()));
} // ADC A (HL) [Z/0/H/C]
fn execute_8f(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    adc_a_u8(cpu, cpu.reg.a);
} // ADC A A [Z/0/H/C]
fn execute_90(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.b);
} // SUB B  [Z/1/H/C]
fn execute_91(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.c);
} // SUB C  [Z/1/H/C]
fn execute_92(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.d);
} // SUB D  [Z/1/H/C]
fn execute_93(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.e);
} // SUB E  [Z/1/H/C]
fn execute_94(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.h);
} // SUB H  [Z/1/H/C]
fn execute_95(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.l);
} // SUB L  [Z/1/H/C]
fn execute_96(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    sub_u8(cpu, cpu.mmu.get(cpu.reg.hl()));
} // SUB (HL)  [Z/1/H/C]
fn execute_97(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    sub_u8(cpu, cpu.reg.a);
} // SUB A  [Z/1/H/C]
fn execute_98(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A B [Z/1/H/C]
fn execute_99(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A C [Z/1/H/C]
fn execute_9a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A D [Z/1/H/C]
fn execute_9b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A E [Z/1/H/C]
fn execute_9c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A H [Z/1/H/C]
fn execute_9d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A L [Z/1/H/C]
fn execute_9e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // SBC A (HL) [Z/1/H/C]
fn execute_9f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SBC A A [Z/1/H/C]
fn execute_a0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND B  [Z/0/1/0]
fn execute_a1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND C  [Z/0/1/0]
fn execute_a2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND D  [Z/0/1/0]
fn execute_a3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND E  [Z/0/1/0]
fn execute_a4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND H  [Z/0/1/0]
fn execute_a5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND L  [Z/0/1/0]
fn execute_a6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // AND (HL)  [Z/0/1/0]
fn execute_a7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // AND A  [Z/0/1/0]
fn execute_a8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.b);
} // XOR B  [Z/0/0/0]
fn execute_a9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.c);
} // XOR C  [Z/0/0/0]
fn execute_aa(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.d);
} // XOR D  [Z/0/0/0]
fn execute_ab(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.e);
} // XOR E  [Z/0/0/0]
fn execute_ac(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.h);
} // XOR H  [Z/0/0/0]
fn execute_ad(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.l);
} // XOR L  [Z/0/0/0]
fn execute_ae(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.xor(cpu.mmu.get(cpu.reg.hl()));
} // XOR (HL)  [Z/0/0/0]
fn execute_af(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.xor(cpu.reg.a);
} // XOR A  [Z/0/0/0]
fn execute_b0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.b);
} // OR B  [Z/0/0/0]
fn execute_b1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.c);
} // OR C  [Z/0/0/0]
fn execute_b2(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.d);
} // OR D  [Z/0/0/0]
fn execute_b3(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.e);
} // OR E  [Z/0/0/0]
fn execute_b4(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.h);
} // OR H  [Z/0/0/0]
fn execute_b5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.l);
} // OR L  [Z/0/0/0]
fn execute_b6(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    or_u8(cpu, cpu.mmu.get(cpu.reg.hl()));
} // OR (HL)  [Z/0/0/0]
fn execute_b7(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    or_u8(cpu, cpu.reg.a);
} // OR A  [Z/0/0/0]
fn execute_b8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.b);
} // CP B  [Z/1/H/C]
fn execute_b9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.c);
} // CP C  [Z/1/H/C]
fn execute_ba(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.d);
} // CP D  [Z/1/H/C]
fn execute_bb(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.e);
} // CP E  [Z/1/H/C]
fn execute_bc(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.h);
} // CP H  [Z/1/H/C]
fn execute_bd(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.l);
} // CP L  [Z/1/H/C]
fn execute_be(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cp_d8(cpu, cpu.mmu.get(cpu.reg.hl()));
} // CP (HL)  [Z/1/H/C]
fn execute_bf(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cp_d8(cpu, cpu.reg.a);
} // CP A  [Z/1/H/C]
fn execute_c0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles = 2;
    if !cpu.reg.f.zero {
        cpu.cycles = 5;
        cpu.reg.pc = pop_word(cpu);
    }
} // RET NZ  [-/-/-/-]
fn execute_c1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
    let word = pop_word(cpu);
    cpu.reg.set_bc(word);
} // POP BC  [-/-/-/-]
fn execute_c2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [4, 3];
} // JP NZ a16 [-/-/-/-]
fn execute_c3(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles += 4;
    let (left, right) = (cpu.get_op(1), cpu.get_op(2));
    cpu.reg.pc = word_from(right, left);
} // JP a16  [-/-/-/-]
fn execute_c4(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles = 3;
    if !cpu.reg.f.zero {
        cpu.cycles = 6;
        call_a16(cpu);
    }
} // CALL NZ a16 [-/-/-/-]
fn execute_c5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
    cpu.push_word(cpu.reg.bc());
} // PUSH BC  [-/-/-/-]
fn execute_c6(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    add_a_u8(cpu, cpu.get_op(1));
} // ADD A d8 [Z/0/H/C]
fn execute_c7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 00H  [-/-/-/-]
fn execute_c8(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles = 2;
    if cpu.reg.f.zero {
        cpu.cycles = 5;
        cpu.reg.pc = pop_word(cpu);
    }
} // RET Z  [-/-/-/-]
fn execute_c9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles += 4;
    cpu.reg.pc = pop_word(cpu);
} // RET  [-/-/-/-]
fn execute_ca(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [4, 3];
} // JP Z a16 [-/-/-/-]
fn execute_cb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // PREFIX CB  [-/-/-/-]
fn execute_cc(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles = 3;
    if cpu.reg.f.zero {
        cpu.cycles = 6;
        call_a16(cpu);
    }
} // CALL Z a16 [-/-/-/-]
fn execute_cd(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 6;
    call_a16(cpu);
} // CALL a16  [-/-/-/-]
fn execute_ce(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    adc_a_u8(cpu, cpu.get_op(1));
} // ADC A d8 [Z/0/H/C]
fn execute_cf(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 08H  [-/-/-/-]
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
    cpu.cycles += 3;
    let word = pop_word(cpu);
    cpu.reg.set_de(word);
} // POP DE  [-/-/-/-]
fn execute_d2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [4, 3];
} // JP NC a16 [-/-/-/-]
fn execute_d4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [6, 3];
} // CALL NC a16 [-/-/-/-]
fn execute_d5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
    cpu.push_word(cpu.reg.de());
} // PUSH DE  [-/-/-/-]
fn execute_d6(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    sub_u8(cpu, cpu.get_op(1));
} // SUB d8  [Z/1/H/C]
fn execute_d7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 10H  [-/-/-/-]
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RETI  [-/-/-/-]
fn execute_da(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [4, 3];
} // JP C a16 [-/-/-/-]
fn execute_dc(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [6, 3];
} // CALL C a16 [-/-/-/-]
fn execute_de(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SBC A d8 [Z/1/H/C]
fn execute_df(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 18H  [-/-/-/-]
fn execute_e0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
    let address = word_from(0xFF, cpu.get_op(1));
    ld_mem_d8(cpu, address, cpu.reg.a);
} // LDH (a8) A [-/-/-/-]
fn execute_e1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
    let word = pop_word(cpu);
    cpu.reg.set_hl(word);
} // POP HL  [-/-/-/-]
fn execute_e2(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    let address = word_from(0xFF, cpu.reg.c);
    cpu.mmu.set(address, cpu.reg.a);
} // LD (C) A [-/-/-/-]
fn execute_e5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
    cpu.push_word(cpu.reg.hl());
} // PUSH HL  [-/-/-/-]
fn execute_e6(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    cpu.and(cpu.get_op(1));
} // AND d8  [Z/0/1/0]
fn execute_e7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 20H  [-/-/-/-]
fn execute_e8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // ADD SP r8 [0/0/H/C]
fn execute_e9(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 0;
    cpu.cycles += 1;
    cpu.reg.pc = cpu.reg.hl();
} // JP (HL)  [-/-/-/-]
fn execute_ea(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 4;
    let address = word_from(cpu.get_op(2), cpu.get_op(1));
    ld_mem_d8(cpu, address, cpu.reg.a);
} // LD (a16) A [-/-/-/-]
fn execute_ee(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    cpu.xor(cpu.get_op(1));
} // XOR d8  [Z/0/0/0]
fn execute_ef(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 28H  [-/-/-/-]
fn execute_f0(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
    let address = word_from(0xFF, cpu.get_op(1));
    cpu.reg.a = cpu.mmu.get(address);
} // LDH A (a8) [-/-/-/-]
fn execute_f1(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
    let word = pop_word(cpu);
    cpu.reg.set_af(word);
} // POP AF  [Z/N/H/C]
fn execute_f2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (C) [-/-/-/-]
fn execute_f3(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.ime = false;
} // DI  [-/-/-/-]
fn execute_f5(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
    cpu.push_word(cpu.reg.af());
} // PUSH AF  [-/-/-/-]
fn execute_f6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // OR d8  [Z/0/0/0]
fn execute_f7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 30H  [-/-/-/-]
fn execute_f8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
} // LD HL SP+r8 [0/0/H/C]
fn execute_f9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD SP HL [-/-/-/-]
fn execute_fa(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 4;
    let address = word_from(cpu.get_op(2), cpu.get_op(1));
    ld_d8(&mut cpu.reg.a, cpu.mmu.get(address));
} // LD A (a16) [-/-/-/-]
fn execute_fb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // EI  [-/-/-/-]
fn execute_fe(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.get_op(1);
    cp_d8(cpu, byte);
} // CP d8  [Z/1/H/C]
fn execute_ff(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 38H  [-/-/-/-]

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