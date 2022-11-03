use super::cpu::Cpu;
use super::decode::decode;
use log::{debug, warn};
use crate::word_from;

fn op_implemented(cpu: &Cpu) {
    debug!("I PC: {:04x} {}", cpu.reg.pc, decode(cpu).expect("Unknown opcode"));
}

fn op_unimplemented(cpu: &Cpu) {
    warn!("U PC: {:04x} {}", cpu.reg.pc, decode(cpu).expect("Unknown opcode"));
}

pub fn execute(cpu: &mut Cpu) {
    match cpu.opcode {
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
        0x40 => execute_40(cpu),
        0x41 => execute_41(cpu),
        0x42 => execute_42(cpu),
        0x43 => execute_43(cpu),
        0x44 => execute_44(cpu),
        0x45 => execute_45(cpu),
        0x46 => execute_46(cpu),
        0x47 => execute_47(cpu),
        0x48 => execute_48(cpu),
        0x49 => execute_49(cpu),
        0x4a => execute_4a(cpu),
        0x4b => execute_4b(cpu),
        0x4c => execute_4c(cpu),
        0x4d => execute_4d(cpu),
        0x4e => execute_4e(cpu),
        0x4f => execute_4f(cpu),
        0x50 => execute_50(cpu),
        0x51 => execute_51(cpu),
        0x52 => execute_52(cpu),
        0x53 => execute_53(cpu),
        0x54 => execute_54(cpu),
        0x55 => execute_55(cpu),
        0x56 => execute_56(cpu),
        0x57 => execute_57(cpu),
        0x58 => execute_58(cpu),
        0x59 => execute_59(cpu),
        0x5a => execute_5a(cpu),
        0x5b => execute_5b(cpu),
        0x5c => execute_5c(cpu),
        0x5d => execute_5d(cpu),
        0x5e => execute_5e(cpu),
        0x5f => execute_5f(cpu),
        0x60 => execute_60(cpu),
        0x61 => execute_61(cpu),
        0x62 => execute_62(cpu),
        0x63 => execute_63(cpu),
        0x64 => execute_64(cpu),
        0x65 => execute_65(cpu),
        0x66 => execute_66(cpu),
        0x67 => execute_67(cpu),
        0x68 => execute_68(cpu),
        0x69 => execute_69(cpu),
        0x6a => execute_6a(cpu),
        0x6b => execute_6b(cpu),
        0x6c => execute_6c(cpu),
        0x6d => execute_6d(cpu),
        0x6e => execute_6e(cpu),
        0x6f => execute_6f(cpu),
        0x70 => execute_70(cpu),
        0x71 => execute_71(cpu),
        0x72 => execute_72(cpu),
        0x73 => execute_73(cpu),
        0x74 => execute_74(cpu),
        0x75 => execute_75(cpu),
        0x76 => execute_76(cpu),
        0x77 => execute_77(cpu),
        0x78 => execute_78(cpu),
        0x79 => execute_79(cpu),
        0x7a => execute_7a(cpu),
        0x7b => execute_7b(cpu),
        0x7c => execute_7c(cpu),
        0x7d => execute_7d(cpu),
        0x7e => execute_7e(cpu),
        0x7f => execute_7f(cpu),
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

fn execute_00(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // NOP  [-/-/-/-]
fn execute_01(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
} // LD BC d16 [-/-/-/-]
fn execute_02(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (BC) A [-/-/-/-]
fn execute_03(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // INC BC  [-/-/-/-]
fn execute_04(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC B  [Z/0/H/-]
fn execute_05(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC B  [Z/1/H/-]
fn execute_06(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD B d8 [-/-/-/-]
fn execute_07(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // RLCA  [0/0/0/C]
fn execute_08(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 5;
} // LD (a16) SP [-/-/-/-]
fn execute_09(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // ADD HL BC [-/0/H/C]
fn execute_0a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (BC) [-/-/-/-]
fn execute_0b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // DEC BC  [-/-/-/-]
fn execute_0c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC C  [Z/0/H/-]
fn execute_0d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC C  [Z/1/H/-]
fn execute_0e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD C d8 [-/-/-/-]
fn execute_0f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // RRCA  [0/0/0/C]
fn execute_10(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // STOP 0  [-/-/-/-]
fn execute_11(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
} // LD DE d16 [-/-/-/-]
fn execute_12(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (DE) A [-/-/-/-]
fn execute_13(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // INC DE  [-/-/-/-]
fn execute_14(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC D  [Z/0/H/-]
fn execute_15(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC D  [Z/1/H/-]
fn execute_16(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD D d8 [-/-/-/-]
fn execute_17(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // RLA  [0/0/0/C]
fn execute_18(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
} // JR r8  [-/-/-/-]
fn execute_19(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // ADD HL DE [-/0/H/C]
fn execute_1a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (DE) [-/-/-/-]
fn execute_1b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // DEC DE  [-/-/-/-]
fn execute_1c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC E  [Z/0/H/-]
fn execute_1d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC E  [Z/1/H/-]
fn execute_1e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD E d8 [-/-/-/-]
fn execute_1f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // RRA  [0/0/0/C]
fn execute_20(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    // Two possible CPU cycles: [3, 2];
} // JR NZ r8 [-/-/-/-]
fn execute_21(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
    cpu.reg.h = cpu.mmu.get(cpu.reg.pc + 2);
    cpu.reg.l = cpu.mmu.get(cpu.reg.pc + 1);
} // LD HL d16 [-/-/-/-]
fn execute_22(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL+) A [-/-/-/-]
fn execute_23(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // INC HL  [-/-/-/-]
fn execute_24(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC H  [Z/0/H/-]
fn execute_25(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC H  [Z/1/H/-]
fn execute_26(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD H d8 [-/-/-/-]
fn execute_27(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DAA  [Z/-/0/C]
fn execute_28(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    // Two possible CPU cycles: [3, 2];
} // JR Z r8 [-/-/-/-]
fn execute_29(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // ADD HL HL [-/0/H/C]
fn execute_2a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (HL+) [-/-/-/-]
fn execute_2b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // DEC HL  [-/-/-/-]
fn execute_2c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC L  [Z/0/H/-]
fn execute_2d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC L  [Z/1/H/-]
fn execute_2e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD L d8 [-/-/-/-]
fn execute_2f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CPL  [-/1/1/-]
fn execute_30(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    // Two possible CPU cycles: [3, 2];
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
    cpu.reg.set_hl(cpu.reg.hl() - 1);
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
} // DEC (HL)  [Z/1/H/-]
fn execute_36(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
} // LD (HL) d8 [-/-/-/-]
fn execute_37(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SCF  [-/0/0/1]
fn execute_38(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    // Two possible CPU cycles: [3, 2];
} // JR C r8 [-/-/-/-]
fn execute_39(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // INC A  [Z/0/H/-]
fn execute_3d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC A  [Z/1/H/-]
fn execute_3e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // LD A d8 [-/-/-/-]
fn execute_3f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CCF  [-/0/0/C]
fn execute_40(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B B [-/-/-/-]
fn execute_41(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B C [-/-/-/-]
fn execute_42(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B D [-/-/-/-]
fn execute_43(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B E [-/-/-/-]
fn execute_44(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B H [-/-/-/-]
fn execute_45(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B L [-/-/-/-]
fn execute_46(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD B (HL) [-/-/-/-]
fn execute_47(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD B A [-/-/-/-]
fn execute_48(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C B [-/-/-/-]
fn execute_49(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C C [-/-/-/-]
fn execute_4a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C D [-/-/-/-]
fn execute_4b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C E [-/-/-/-]
fn execute_4c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C H [-/-/-/-]
fn execute_4d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C L [-/-/-/-]
fn execute_4e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD C (HL) [-/-/-/-]
fn execute_4f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD C A [-/-/-/-]
fn execute_50(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D B [-/-/-/-]
fn execute_51(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D C [-/-/-/-]
fn execute_52(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D D [-/-/-/-]
fn execute_53(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D E [-/-/-/-]
fn execute_54(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D H [-/-/-/-]
fn execute_55(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D L [-/-/-/-]
fn execute_56(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD D (HL) [-/-/-/-]
fn execute_57(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD D A [-/-/-/-]
fn execute_58(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E B [-/-/-/-]
fn execute_59(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E C [-/-/-/-]
fn execute_5a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E D [-/-/-/-]
fn execute_5b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E E [-/-/-/-]
fn execute_5c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E H [-/-/-/-]
fn execute_5d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E L [-/-/-/-]
fn execute_5e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD E (HL) [-/-/-/-]
fn execute_5f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD E A [-/-/-/-]
fn execute_60(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H B [-/-/-/-]
fn execute_61(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H C [-/-/-/-]
fn execute_62(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H D [-/-/-/-]
fn execute_63(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H E [-/-/-/-]
fn execute_64(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H H [-/-/-/-]
fn execute_65(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H L [-/-/-/-]
fn execute_66(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD H (HL) [-/-/-/-]
fn execute_67(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD H A [-/-/-/-]
fn execute_68(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L B [-/-/-/-]
fn execute_69(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L C [-/-/-/-]
fn execute_6a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L D [-/-/-/-]
fn execute_6b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L E [-/-/-/-]
fn execute_6c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L H [-/-/-/-]
fn execute_6d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L L [-/-/-/-]
fn execute_6e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD L (HL) [-/-/-/-]
fn execute_6f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD L A [-/-/-/-]
fn execute_70(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) B [-/-/-/-]
fn execute_71(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) C [-/-/-/-]
fn execute_72(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) D [-/-/-/-]
fn execute_73(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) E [-/-/-/-]
fn execute_74(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) H [-/-/-/-]
fn execute_75(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) L [-/-/-/-]
fn execute_76(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // HALT  [-/-/-/-]
fn execute_77(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL) A [-/-/-/-]
fn execute_78(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A B [-/-/-/-]
fn execute_79(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A C [-/-/-/-]
fn execute_7a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A D [-/-/-/-]
fn execute_7b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A E [-/-/-/-]
fn execute_7c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A H [-/-/-/-]
fn execute_7d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A L [-/-/-/-]
fn execute_7e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (HL) [-/-/-/-]
fn execute_7f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // LD A A [-/-/-/-]
fn execute_80(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A B [Z/0/H/C]
fn execute_81(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A C [Z/0/H/C]
fn execute_82(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A D [Z/0/H/C]
fn execute_83(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A E [Z/0/H/C]
fn execute_84(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A H [Z/0/H/C]
fn execute_85(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A L [Z/0/H/C]
fn execute_86(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // ADD A (HL) [Z/0/H/C]
fn execute_87(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADD A A [Z/0/H/C]
fn execute_88(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A B [Z/0/H/C]
fn execute_89(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A C [Z/0/H/C]
fn execute_8a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A D [Z/0/H/C]
fn execute_8b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A E [Z/0/H/C]
fn execute_8c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A H [Z/0/H/C]
fn execute_8d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A L [Z/0/H/C]
fn execute_8e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // ADC A (HL) [Z/0/H/C]
fn execute_8f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // ADC A A [Z/0/H/C]
fn execute_90(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SUB B  [Z/1/H/C]
fn execute_91(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SUB C  [Z/1/H/C]
fn execute_92(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SUB D  [Z/1/H/C]
fn execute_93(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SUB E  [Z/1/H/C]
fn execute_94(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SUB H  [Z/1/H/C]
fn execute_95(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // SUB L  [Z/1/H/C]
fn execute_96(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // SUB (HL)  [Z/1/H/C]
fn execute_97(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // XOR B  [Z/0/0/0]
fn execute_a9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // XOR C  [Z/0/0/0]
fn execute_aa(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // XOR D  [Z/0/0/0]
fn execute_ab(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // XOR E  [Z/0/0/0]
fn execute_ac(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // XOR H  [Z/0/0/0]
fn execute_ad(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // XOR L  [Z/0/0/0]
fn execute_ae(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // XOR (HL)  [Z/0/0/0]
fn execute_af(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.reg.a ^= cpu.reg.a;
    cpu.reg.f.zero = cpu.reg.a == 0;
} // XOR A  [Z/0/0/0]
fn execute_b0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR B  [Z/0/0/0]
fn execute_b1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR C  [Z/0/0/0]
fn execute_b2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR D  [Z/0/0/0]
fn execute_b3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR E  [Z/0/0/0]
fn execute_b4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR H  [Z/0/0/0]
fn execute_b5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR L  [Z/0/0/0]
fn execute_b6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // OR (HL)  [Z/0/0/0]
fn execute_b7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // OR A  [Z/0/0/0]
fn execute_b8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP B  [Z/1/H/C]
fn execute_b9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP C  [Z/1/H/C]
fn execute_ba(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP D  [Z/1/H/C]
fn execute_bb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP E  [Z/1/H/C]
fn execute_bc(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP H  [Z/1/H/C]
fn execute_bd(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP L  [Z/1/H/C]
fn execute_be(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // CP (HL)  [Z/1/H/C]
fn execute_bf(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // CP A  [Z/1/H/C]
fn execute_c0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    // Two possible CPU cycles: [5, 2];
} // RET NZ  [-/-/-/-]
fn execute_c1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
} // POP BC  [-/-/-/-]
fn execute_c2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [4, 3];
} // JP NZ a16 [-/-/-/-]
fn execute_c3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 4;
} // JP a16  [-/-/-/-]
fn execute_c4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [6, 3];
} // CALL NZ a16 [-/-/-/-]
fn execute_c5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // PUSH BC  [-/-/-/-]
fn execute_c6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // ADD A d8 [Z/0/H/C]
fn execute_c7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 00H  [-/-/-/-]
fn execute_c8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    // Two possible CPU cycles: [5, 2];
} // RET Z  [-/-/-/-]
fn execute_c9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    // Two possible CPU cycles: [6, 3];
} // CALL Z a16 [-/-/-/-]
fn execute_cd(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 6;
} // CALL a16  [-/-/-/-]
fn execute_ce(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // ADC A d8 [Z/0/H/C]
fn execute_cf(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 08H  [-/-/-/-]
fn execute_d0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    // Two possible CPU cycles: [5, 2];
} // RET NC  [-/-/-/-]
fn execute_d1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // PUSH DE  [-/-/-/-]
fn execute_d6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SUB d8  [Z/1/H/C]
fn execute_d7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 10H  [-/-/-/-]
fn execute_d8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    // Two possible CPU cycles: [5, 2];
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
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
} // LDH (a8) A [-/-/-/-]
fn execute_e1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
} // POP HL  [-/-/-/-]
fn execute_e2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (C) A [-/-/-/-]
fn execute_e5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // PUSH HL  [-/-/-/-]
fn execute_e6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // JP (HL)  [-/-/-/-]
fn execute_ea(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 4;
} // LD (a16) A [-/-/-/-]
fn execute_ee(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // XOR d8  [Z/0/0/0]
fn execute_ef(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
} // RST 28H  [-/-/-/-]
fn execute_f0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 3;
} // LDH A (a8) [-/-/-/-]
fn execute_f1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 3;
} // POP AF  [Z/N/H/C]
fn execute_f2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD A (C) [-/-/-/-]
fn execute_f3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DI  [-/-/-/-]
fn execute_f5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 4;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 4;
} // LD A (a16) [-/-/-/-]
fn execute_fb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // EI  [-/-/-/-]
fn execute_fe(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
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
    fn execute_21_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x21, 0xBE, 0xEF];
        execute_21(&mut cpu);
        assert_eq!(cpu.reg.hl(), 0xEFBE);
    }

    #[test]
    fn execute_31_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x31, 0xBE, 0xEF];
        execute_31(&mut cpu);
        assert_eq!(cpu.reg.sp, 0xEFBE);
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