use super::cpu::Cpu;
use super::decode::decode;
use log::{debug, warn};
use crate::registers::Flags;
use crate::{bytes_from, word_from};

// TODO: Flag representation
fn op_implemented(cpu: &Cpu) {
    debug!("I PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
        cpu.reg.pc, decode(cpu).expect("Unknown opcode"),
        cpu.reg.a, cpu.reg.f.to_string(), cpu.reg.b, cpu.reg.c, cpu.reg.d, cpu.reg.e, cpu.reg.h, cpu.reg.l, cpu.reg.sp,
    );
}

fn op_unimplemented(cpu: &Cpu) {
    warn!("U PC: {:04x} {} [A:{:02X} F:{}] [B:{:02X} C:{:02X}] [D:{:02X} E:{:02X}] [H:{:02X} L:{:02X}] [SP:{:04X}] |",
        cpu.reg.pc, decode(cpu).expect(&format!("Unknown opcode : {:02X}", cpu.opcode)),
        cpu.reg.a, cpu.reg.f.to_string(), cpu.reg.b, cpu.reg.c, cpu.reg.d, cpu.reg.e, cpu.reg.h, cpu.reg.l, cpu.reg.sp,
    );
}

pub fn execute(cpu: &mut Cpu) {
    if cpu.opcode == 0xCB {
        cpu.cb_prefix = true;
        cpu.opcode = cpu.mmu.get(cpu.reg.pc + 1);
        match cpu.opcode {
            0x00 => cb_execute_00(cpu),
            0x01 => cb_execute_01(cpu),
            0x02 => cb_execute_02(cpu),
            0x03 => cb_execute_03(cpu),
            0x04 => cb_execute_04(cpu),
            0x05 => cb_execute_05(cpu),
            0x06 => cb_execute_06(cpu),
            0x07 => cb_execute_07(cpu),
            0x08 => cb_execute_08(cpu),
            0x09 => cb_execute_09(cpu),
            0x0a => cb_execute_0a(cpu),
            0x0b => cb_execute_0b(cpu),
            0x0c => cb_execute_0c(cpu),
            0x0d => cb_execute_0d(cpu),
            0x0e => cb_execute_0e(cpu),
            0x0f => cb_execute_0f(cpu),
            0x10 => cb_execute_10(cpu),
            0x11 => cb_execute_11(cpu),
            0x12 => cb_execute_12(cpu),
            0x13 => cb_execute_13(cpu),
            0x14 => cb_execute_14(cpu),
            0x15 => cb_execute_15(cpu),
            0x16 => cb_execute_16(cpu),
            0x17 => cb_execute_17(cpu),
            0x18 => cb_execute_18(cpu),
            0x19 => cb_execute_19(cpu),
            0x1a => cb_execute_1a(cpu),
            0x1b => cb_execute_1b(cpu),
            0x1c => cb_execute_1c(cpu),
            0x1d => cb_execute_1d(cpu),
            0x1e => cb_execute_1e(cpu),
            0x1f => cb_execute_1f(cpu),
            0x20 => cb_execute_20(cpu),
            0x21 => cb_execute_21(cpu),
            0x22 => cb_execute_22(cpu),
            0x23 => cb_execute_23(cpu),
            0x24 => cb_execute_24(cpu),
            0x25 => cb_execute_25(cpu),
            0x26 => cb_execute_26(cpu),
            0x27 => cb_execute_27(cpu),
            0x28 => cb_execute_28(cpu),
            0x29 => cb_execute_29(cpu),
            0x2a => cb_execute_2a(cpu),
            0x2b => cb_execute_2b(cpu),
            0x2c => cb_execute_2c(cpu),
            0x2d => cb_execute_2d(cpu),
            0x2e => cb_execute_2e(cpu),
            0x2f => cb_execute_2f(cpu),
            0x30 => cb_execute_30(cpu),
            0x31 => cb_execute_31(cpu),
            0x32 => cb_execute_32(cpu),
            0x33 => cb_execute_33(cpu),
            0x34 => cb_execute_34(cpu),
            0x35 => cb_execute_35(cpu),
            0x36 => cb_execute_36(cpu),
            0x37 => cb_execute_37(cpu),
            0x38 => cb_execute_38(cpu),
            0x39 => cb_execute_39(cpu),
            0x3a => cb_execute_3a(cpu),
            0x3b => cb_execute_3b(cpu),
            0x3c => cb_execute_3c(cpu),
            0x3d => cb_execute_3d(cpu),
            0x3e => cb_execute_3e(cpu),
            0x3f => cb_execute_3f(cpu),
            0x40 => cb_execute_40(cpu),
            0x41 => cb_execute_41(cpu),
            0x42 => cb_execute_42(cpu),
            0x43 => cb_execute_43(cpu),
            0x44 => cb_execute_44(cpu),
            0x45 => cb_execute_45(cpu),
            0x46 => cb_execute_46(cpu),
            0x47 => cb_execute_47(cpu),
            0x48 => cb_execute_48(cpu),
            0x49 => cb_execute_49(cpu),
            0x4a => cb_execute_4a(cpu),
            0x4b => cb_execute_4b(cpu),
            0x4c => cb_execute_4c(cpu),
            0x4d => cb_execute_4d(cpu),
            0x4e => cb_execute_4e(cpu),
            0x4f => cb_execute_4f(cpu),
            0x50 => cb_execute_50(cpu),
            0x51 => cb_execute_51(cpu),
            0x52 => cb_execute_52(cpu),
            0x53 => cb_execute_53(cpu),
            0x54 => cb_execute_54(cpu),
            0x55 => cb_execute_55(cpu),
            0x56 => cb_execute_56(cpu),
            0x57 => cb_execute_57(cpu),
            0x58 => cb_execute_58(cpu),
            0x59 => cb_execute_59(cpu),
            0x5a => cb_execute_5a(cpu),
            0x5b => cb_execute_5b(cpu),
            0x5c => cb_execute_5c(cpu),
            0x5d => cb_execute_5d(cpu),
            0x5e => cb_execute_5e(cpu),
            0x5f => cb_execute_5f(cpu),
            0x60 => cb_execute_60(cpu),
            0x61 => cb_execute_61(cpu),
            0x62 => cb_execute_62(cpu),
            0x63 => cb_execute_63(cpu),
            0x64 => cb_execute_64(cpu),
            0x65 => cb_execute_65(cpu),
            0x66 => cb_execute_66(cpu),
            0x67 => cb_execute_67(cpu),
            0x68 => cb_execute_68(cpu),
            0x69 => cb_execute_69(cpu),
            0x6a => cb_execute_6a(cpu),
            0x6b => cb_execute_6b(cpu),
            0x6c => cb_execute_6c(cpu),
            0x6d => cb_execute_6d(cpu),
            0x6e => cb_execute_6e(cpu),
            0x6f => cb_execute_6f(cpu),
            0x70 => cb_execute_70(cpu),
            0x71 => cb_execute_71(cpu),
            0x72 => cb_execute_72(cpu),
            0x73 => cb_execute_73(cpu),
            0x74 => cb_execute_74(cpu),
            0x75 => cb_execute_75(cpu),
            0x76 => cb_execute_76(cpu),
            0x77 => cb_execute_77(cpu),
            0x78 => cb_execute_78(cpu),
            0x79 => cb_execute_79(cpu),
            0x7a => cb_execute_7a(cpu),
            0x7b => cb_execute_7b(cpu),
            0x7c => cb_execute_7c(cpu),
            0x7d => cb_execute_7d(cpu),
            0x7e => cb_execute_7e(cpu),
            0x7f => cb_execute_7f(cpu),
            0x80 => cb_execute_80(cpu),
            0x81 => cb_execute_81(cpu),
            0x82 => cb_execute_82(cpu),
            0x83 => cb_execute_83(cpu),
            0x84 => cb_execute_84(cpu),
            0x85 => cb_execute_85(cpu),
            0x86 => cb_execute_86(cpu),
            0x87 => cb_execute_87(cpu),
            0x88 => cb_execute_88(cpu),
            0x89 => cb_execute_89(cpu),
            0x8a => cb_execute_8a(cpu),
            0x8b => cb_execute_8b(cpu),
            0x8c => cb_execute_8c(cpu),
            0x8d => cb_execute_8d(cpu),
            0x8e => cb_execute_8e(cpu),
            0x8f => cb_execute_8f(cpu),
            0x90 => cb_execute_90(cpu),
            0x91 => cb_execute_91(cpu),
            0x92 => cb_execute_92(cpu),
            0x93 => cb_execute_93(cpu),
            0x94 => cb_execute_94(cpu),
            0x95 => cb_execute_95(cpu),
            0x96 => cb_execute_96(cpu),
            0x97 => cb_execute_97(cpu),
            0x98 => cb_execute_98(cpu),
            0x99 => cb_execute_99(cpu),
            0x9a => cb_execute_9a(cpu),
            0x9b => cb_execute_9b(cpu),
            0x9c => cb_execute_9c(cpu),
            0x9d => cb_execute_9d(cpu),
            0x9e => cb_execute_9e(cpu),
            0x9f => cb_execute_9f(cpu),
            0xa0 => cb_execute_a0(cpu),
            0xa1 => cb_execute_a1(cpu),
            0xa2 => cb_execute_a2(cpu),
            0xa3 => cb_execute_a3(cpu),
            0xa4 => cb_execute_a4(cpu),
            0xa5 => cb_execute_a5(cpu),
            0xa6 => cb_execute_a6(cpu),
            0xa7 => cb_execute_a7(cpu),
            0xa8 => cb_execute_a8(cpu),
            0xa9 => cb_execute_a9(cpu),
            0xaa => cb_execute_aa(cpu),
            0xab => cb_execute_ab(cpu),
            0xac => cb_execute_ac(cpu),
            0xad => cb_execute_ad(cpu),
            0xae => cb_execute_ae(cpu),
            0xaf => cb_execute_af(cpu),
            0xb0 => cb_execute_b0(cpu),
            0xb1 => cb_execute_b1(cpu),
            0xb2 => cb_execute_b2(cpu),
            0xb3 => cb_execute_b3(cpu),
            0xb4 => cb_execute_b4(cpu),
            0xb5 => cb_execute_b5(cpu),
            0xb6 => cb_execute_b6(cpu),
            0xb7 => cb_execute_b7(cpu),
            0xb8 => cb_execute_b8(cpu),
            0xb9 => cb_execute_b9(cpu),
            0xba => cb_execute_ba(cpu),
            0xbb => cb_execute_bb(cpu),
            0xbc => cb_execute_bc(cpu),
            0xbd => cb_execute_bd(cpu),
            0xbe => cb_execute_be(cpu),
            0xbf => cb_execute_bf(cpu),
            0xc0 => cb_execute_c0(cpu),
            0xc1 => cb_execute_c1(cpu),
            0xc2 => cb_execute_c2(cpu),
            0xc3 => cb_execute_c3(cpu),
            0xc4 => cb_execute_c4(cpu),
            0xc5 => cb_execute_c5(cpu),
            0xc6 => cb_execute_c6(cpu),
            0xc7 => cb_execute_c7(cpu),
            0xc8 => cb_execute_c8(cpu),
            0xc9 => cb_execute_c9(cpu),
            0xca => cb_execute_ca(cpu),
            0xcb => cb_execute_cb(cpu),
            0xcc => cb_execute_cc(cpu),
            0xcd => cb_execute_cd(cpu),
            0xce => cb_execute_ce(cpu),
            0xcf => cb_execute_cf(cpu),
            0xd0 => cb_execute_d0(cpu),
            0xd1 => cb_execute_d1(cpu),
            0xd2 => cb_execute_d2(cpu),
            0xd3 => cb_execute_d3(cpu),
            0xd4 => cb_execute_d4(cpu),
            0xd5 => cb_execute_d5(cpu),
            0xd6 => cb_execute_d6(cpu),
            0xd7 => cb_execute_d7(cpu),
            0xd8 => cb_execute_d8(cpu),
            0xd9 => cb_execute_d9(cpu),
            0xda => cb_execute_da(cpu),
            0xdb => cb_execute_db(cpu),
            0xdc => cb_execute_dc(cpu),
            0xdd => cb_execute_dd(cpu),
            0xde => cb_execute_de(cpu),
            0xdf => cb_execute_df(cpu),
            0xe0 => cb_execute_e0(cpu),
            0xe1 => cb_execute_e1(cpu),
            0xe2 => cb_execute_e2(cpu),
            0xe3 => cb_execute_e3(cpu),
            0xe4 => cb_execute_e4(cpu),
            0xe5 => cb_execute_e5(cpu),
            0xe6 => cb_execute_e6(cpu),
            0xe7 => cb_execute_e7(cpu),
            0xe8 => cb_execute_e8(cpu),
            0xe9 => cb_execute_e9(cpu),
            0xea => cb_execute_ea(cpu),
            0xeb => cb_execute_eb(cpu),
            0xec => cb_execute_ec(cpu),
            0xed => cb_execute_ed(cpu),
            0xee => cb_execute_ee(cpu),
            0xef => cb_execute_ef(cpu),
            0xf0 => cb_execute_f0(cpu),
            0xf1 => cb_execute_f1(cpu),
            0xf2 => cb_execute_f2(cpu),
            0xf3 => cb_execute_f3(cpu),
            0xf4 => cb_execute_f4(cpu),
            0xf5 => cb_execute_f5(cpu),
            0xf6 => cb_execute_f6(cpu),
            0xf7 => cb_execute_f7(cpu),
            0xf8 => cb_execute_f8(cpu),
            0xf9 => cb_execute_f9(cpu),
            0xfa => cb_execute_fa(cpu),
            0xfb => cb_execute_fb(cpu),
            0xfc => cb_execute_fc(cpu),
            0xfd => cb_execute_fd(cpu),
            0xfe => cb_execute_fe(cpu),
            0xff => cb_execute_ff(cpu),
            // _ => op_unimplemented(cpu)
        }
    } else {
        cpu.cb_prefix = false;
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
}

fn inc_d8(reg: &mut u8, flags: &mut Flags) {
    (*reg, flags.carry) = u8::overflowing_add(*reg, 1);
}

fn call_a16(cpu: &mut Cpu) {
    // Store PC on stack
    // TODO: Create a stack_push function
    let (left, right) = bytes_from(cpu.reg.pc + 3);
    cpu.mmu.set(cpu.reg.sp - 1, left);
    cpu.mmu.set(cpu.reg.sp - 2, right);
    cpu.reg.sp -= 2;
    // Set PC to address
    let left = cpu.mmu.get(cpu.reg.pc + 1);
    let right = cpu.mmu.get(cpu.reg.pc + 2);
    cpu.reg.pc = word_from(right, left);
}

fn ld_d8(reg: &mut u8, byte: u8) {
    *reg = byte;
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
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    inc_d8(&mut cpu.reg.c, &mut cpu.reg.f);
} // INC C  [Z/0/H/-]
fn execute_0d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
} // DEC C  [Z/1/H/-]
fn execute_0e(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    let byte = cpu.mmu.get(cpu.reg.pc + 1);
    ld_d8(&mut cpu.reg.c, byte);
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
    op_implemented(cpu);
    cpu.advance_pc = 3;
    cpu.cycles += 3;
    cpu.reg.d = cpu.mmu.get(cpu.reg.pc + 2);
    cpu.reg.e = cpu.mmu.get(cpu.reg.pc + 1);
} // LD DE d16 [-/-/-/-]
fn execute_12(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (DE) A [-/-/-/-]
fn execute_13(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_de(word_from(cpu.reg.d, cpu.reg.e) + 1);
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
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.a = cpu.mmu.get(cpu.reg.de());
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
    op_implemented(cpu);
    cpu.advance_pc = 2;
    // FIXME: Two possible CPU cycles: [3, 2];
    if !cpu.reg.f.zero {
        let s8 = cpu.get_op(1) as i8;
        cpu.advance_pc += s8 as i16;
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
    op_unimplemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
} // LD (HL+) A [-/-/-/-]
fn execute_23(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    cpu.reg.set_hl(cpu.reg.hl() + 1);
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
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    cpu.reg.a = cpu.mmu.get(cpu.reg.pc + 1);
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
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    ld_d8(&mut cpu.reg.b, cpu.reg.a);
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
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.reg.a = cpu.reg.h;
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
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 1;
    cpu.reg.a ^= cpu.reg.b;
    if cpu.reg.a == 0 {
        cpu.reg.f.set_from_bool(true, false, false, false);
    }
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
    cpu.reg.f.set_from_bool(true, false, false, false);
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
fn execute_cd(cpu: &mut Cpu) { // TODO: Missing test
    op_implemented(cpu);
    // cpu.advance_pc = 3;
    cpu.advance_pc = 0;
    cpu.cycles += 6;
    call_a16(cpu);
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
    op_implemented(cpu);
    cpu.advance_pc = 1;
    cpu.cycles += 2;
    let address = word_from(0xFF, cpu.reg.c);
    cpu.mmu.set(address, cpu.reg.a);
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

fn cb_execute_00(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC B  [Z/0/0/C]
fn cb_execute_01(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC C  [Z/0/0/C]
fn cb_execute_02(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC D  [Z/0/0/C]
fn cb_execute_03(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC E  [Z/0/0/C]
fn cb_execute_04(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC H  [Z/0/0/C]
fn cb_execute_05(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC L  [Z/0/0/C]
fn cb_execute_06(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RLC (HL)  [Z/0/0/C]
fn cb_execute_07(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RLC A  [Z/0/0/C]
fn cb_execute_08(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC B  [Z/0/0/C]
fn cb_execute_09(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC C  [Z/0/0/C]
fn cb_execute_0a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC D  [Z/0/0/C]
fn cb_execute_0b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC E  [Z/0/0/C]
fn cb_execute_0c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC H  [Z/0/0/C]
fn cb_execute_0d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC L  [Z/0/0/C]
fn cb_execute_0e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RRC (HL)  [Z/0/0/C]
fn cb_execute_0f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RRC A  [Z/0/0/C]
fn cb_execute_10(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL B  [Z/0/0/C]
fn cb_execute_11(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL C  [Z/0/0/C]
fn cb_execute_12(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL D  [Z/0/0/C]
fn cb_execute_13(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL E  [Z/0/0/C]
fn cb_execute_14(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL H  [Z/0/0/C]
fn cb_execute_15(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL L  [Z/0/0/C]
fn cb_execute_16(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RL (HL)  [Z/0/0/C]
fn cb_execute_17(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RL A  [Z/0/0/C]
fn cb_execute_18(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR B  [Z/0/0/C]
fn cb_execute_19(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR C  [Z/0/0/C]
fn cb_execute_1a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR D  [Z/0/0/C]
fn cb_execute_1b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR E  [Z/0/0/C]
fn cb_execute_1c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR H  [Z/0/0/C]
fn cb_execute_1d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR L  [Z/0/0/C]
fn cb_execute_1e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RR (HL)  [Z/0/0/C]
fn cb_execute_1f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RR A  [Z/0/0/C]
fn cb_execute_20(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA B  [Z/0/0/C]
fn cb_execute_21(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA C  [Z/0/0/C]
fn cb_execute_22(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA D  [Z/0/0/C]
fn cb_execute_23(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA E  [Z/0/0/C]
fn cb_execute_24(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA H  [Z/0/0/C]
fn cb_execute_25(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA L  [Z/0/0/C]
fn cb_execute_26(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SLA (HL)  [Z/0/0/C]
fn cb_execute_27(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SLA A  [Z/0/0/C]
fn cb_execute_28(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA B  [Z/0/0/0]
fn cb_execute_29(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA C  [Z/0/0/0]
fn cb_execute_2a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA D  [Z/0/0/0]
fn cb_execute_2b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA E  [Z/0/0/0]
fn cb_execute_2c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA H  [Z/0/0/0]
fn cb_execute_2d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA L  [Z/0/0/0]
fn cb_execute_2e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SRA (HL)  [Z/0/0/0]
fn cb_execute_2f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRA A  [Z/0/0/0]
fn cb_execute_30(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP B  [Z/0/0/0]
fn cb_execute_31(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP C  [Z/0/0/0]
fn cb_execute_32(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP D  [Z/0/0/0]
fn cb_execute_33(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP E  [Z/0/0/0]
fn cb_execute_34(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP H  [Z/0/0/0]
fn cb_execute_35(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP L  [Z/0/0/0]
fn cb_execute_36(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SWAP (HL)  [Z/0/0/0]
fn cb_execute_37(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SWAP A  [Z/0/0/0]
fn cb_execute_38(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL B  [Z/0/0/C]
fn cb_execute_39(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL C  [Z/0/0/C]
fn cb_execute_3a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL D  [Z/0/0/C]
fn cb_execute_3b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL E  [Z/0/0/C]
fn cb_execute_3c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL H  [Z/0/0/C]
fn cb_execute_3d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL L  [Z/0/0/C]
fn cb_execute_3e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SRL (HL)  [Z/0/0/C]
fn cb_execute_3f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SRL A  [Z/0/0/C]
fn cb_execute_40(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 B [Z/0/1/-]
fn cb_execute_41(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 C [Z/0/1/-]
fn cb_execute_42(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 D [Z/0/1/-]
fn cb_execute_43(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 E [Z/0/1/-]
fn cb_execute_44(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 H [Z/0/1/-]
fn cb_execute_45(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 L [Z/0/1/-]
fn cb_execute_46(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 0 (HL) [Z/0/1/-]
fn cb_execute_47(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 0 A [Z/0/1/-]
fn cb_execute_48(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 B [Z/0/1/-]
fn cb_execute_49(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 C [Z/0/1/-]
fn cb_execute_4a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 D [Z/0/1/-]
fn cb_execute_4b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 E [Z/0/1/-]
fn cb_execute_4c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 H [Z/0/1/-]
fn cb_execute_4d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 L [Z/0/1/-]
fn cb_execute_4e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 1 (HL) [Z/0/1/-]
fn cb_execute_4f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 1 A [Z/0/1/-]
fn cb_execute_50(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 B [Z/0/1/-]
fn cb_execute_51(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 C [Z/0/1/-]
fn cb_execute_52(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 D [Z/0/1/-]
fn cb_execute_53(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 E [Z/0/1/-]
fn cb_execute_54(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 H [Z/0/1/-]
fn cb_execute_55(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 L [Z/0/1/-]
fn cb_execute_56(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 2 (HL) [Z/0/1/-]
fn cb_execute_57(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 2 A [Z/0/1/-]
fn cb_execute_58(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 B [Z/0/1/-]
fn cb_execute_59(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 C [Z/0/1/-]
fn cb_execute_5a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 D [Z/0/1/-]
fn cb_execute_5b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 E [Z/0/1/-]
fn cb_execute_5c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 H [Z/0/1/-]
fn cb_execute_5d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 L [Z/0/1/-]
fn cb_execute_5e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 3 (HL) [Z/0/1/-]
fn cb_execute_5f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 3 A [Z/0/1/-]
fn cb_execute_60(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 B [Z/0/1/-]
fn cb_execute_61(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 C [Z/0/1/-]
fn cb_execute_62(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 D [Z/0/1/-]
fn cb_execute_63(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 E [Z/0/1/-]
fn cb_execute_64(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 H [Z/0/1/-]
fn cb_execute_65(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 L [Z/0/1/-]
fn cb_execute_66(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 4 (HL) [Z/0/1/-]
fn cb_execute_67(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 4 A [Z/0/1/-]
fn cb_execute_68(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 B [Z/0/1/-]
fn cb_execute_69(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 C [Z/0/1/-]
fn cb_execute_6a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 D [Z/0/1/-]
fn cb_execute_6b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 E [Z/0/1/-]
fn cb_execute_6c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 H [Z/0/1/-]
fn cb_execute_6d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 L [Z/0/1/-]
fn cb_execute_6e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 5 (HL) [Z/0/1/-]
fn cb_execute_6f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 5 A [Z/0/1/-]
fn cb_execute_70(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 B [Z/0/1/-]
fn cb_execute_71(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 C [Z/0/1/-]
fn cb_execute_72(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 D [Z/0/1/-]
fn cb_execute_73(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 E [Z/0/1/-]
fn cb_execute_74(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 H [Z/0/1/-]
fn cb_execute_75(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 L [Z/0/1/-]
fn cb_execute_76(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 6 (HL) [Z/0/1/-]
fn cb_execute_77(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 6 A [Z/0/1/-]
fn cb_execute_78(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 7 B [Z/0/1/-]
fn cb_execute_79(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 7 C [Z/0/1/-]
fn cb_execute_7a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 7 D [Z/0/1/-]
fn cb_execute_7b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 7 E [Z/0/1/-]
fn cb_execute_7c(cpu: &mut Cpu) {
    op_implemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
    cpu.reg.f.zero = (cpu.reg.h & 0b10000000) >> 7 == 0;
    cpu.reg.f.sub = false;
    cpu.reg.f.half_carry = true;
} // BIT 7 H [Z/0/1/-]
fn cb_execute_7d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 7 L [Z/0/1/-]
fn cb_execute_7e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // BIT 7 (HL) [Z/0/1/-]
fn cb_execute_7f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // BIT 7 A [Z/0/1/-]
fn cb_execute_80(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 B [-/-/-/-]
fn cb_execute_81(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 C [-/-/-/-]
fn cb_execute_82(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 D [-/-/-/-]
fn cb_execute_83(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 E [-/-/-/-]
fn cb_execute_84(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 H [-/-/-/-]
fn cb_execute_85(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 L [-/-/-/-]
fn cb_execute_86(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 0 (HL) [-/-/-/-]
fn cb_execute_87(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 0 A [-/-/-/-]
fn cb_execute_88(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 B [-/-/-/-]
fn cb_execute_89(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 C [-/-/-/-]
fn cb_execute_8a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 D [-/-/-/-]
fn cb_execute_8b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 E [-/-/-/-]
fn cb_execute_8c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 H [-/-/-/-]
fn cb_execute_8d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 L [-/-/-/-]
fn cb_execute_8e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 1 (HL) [-/-/-/-]
fn cb_execute_8f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 1 A [-/-/-/-]
fn cb_execute_90(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 B [-/-/-/-]
fn cb_execute_91(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 C [-/-/-/-]
fn cb_execute_92(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 D [-/-/-/-]
fn cb_execute_93(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 E [-/-/-/-]
fn cb_execute_94(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 H [-/-/-/-]
fn cb_execute_95(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 L [-/-/-/-]
fn cb_execute_96(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 2 (HL) [-/-/-/-]
fn cb_execute_97(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 2 A [-/-/-/-]
fn cb_execute_98(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 B [-/-/-/-]
fn cb_execute_99(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 C [-/-/-/-]
fn cb_execute_9a(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 D [-/-/-/-]
fn cb_execute_9b(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 E [-/-/-/-]
fn cb_execute_9c(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 H [-/-/-/-]
fn cb_execute_9d(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 L [-/-/-/-]
fn cb_execute_9e(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 3 (HL) [-/-/-/-]
fn cb_execute_9f(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 3 A [-/-/-/-]
fn cb_execute_a0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 B [-/-/-/-]
fn cb_execute_a1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 C [-/-/-/-]
fn cb_execute_a2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 D [-/-/-/-]
fn cb_execute_a3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 E [-/-/-/-]
fn cb_execute_a4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 H [-/-/-/-]
fn cb_execute_a5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 L [-/-/-/-]
fn cb_execute_a6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 4 (HL) [-/-/-/-]
fn cb_execute_a7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 4 A [-/-/-/-]
fn cb_execute_a8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 B [-/-/-/-]
fn cb_execute_a9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 C [-/-/-/-]
fn cb_execute_aa(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 D [-/-/-/-]
fn cb_execute_ab(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 E [-/-/-/-]
fn cb_execute_ac(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 H [-/-/-/-]
fn cb_execute_ad(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 L [-/-/-/-]
fn cb_execute_ae(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 5 (HL) [-/-/-/-]
fn cb_execute_af(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 5 A [-/-/-/-]
fn cb_execute_b0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 B [-/-/-/-]
fn cb_execute_b1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 C [-/-/-/-]
fn cb_execute_b2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 D [-/-/-/-]
fn cb_execute_b3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 E [-/-/-/-]
fn cb_execute_b4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 H [-/-/-/-]
fn cb_execute_b5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 L [-/-/-/-]
fn cb_execute_b6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 6 (HL) [-/-/-/-]
fn cb_execute_b7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 6 A [-/-/-/-]
fn cb_execute_b8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 B [-/-/-/-]
fn cb_execute_b9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 C [-/-/-/-]
fn cb_execute_ba(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 D [-/-/-/-]
fn cb_execute_bb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 E [-/-/-/-]
fn cb_execute_bc(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 H [-/-/-/-]
fn cb_execute_bd(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 L [-/-/-/-]
fn cb_execute_be(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // RES 7 (HL) [-/-/-/-]
fn cb_execute_bf(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // RES 7 A [-/-/-/-]
fn cb_execute_c0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 B [-/-/-/-]
fn cb_execute_c1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 C [-/-/-/-]
fn cb_execute_c2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 D [-/-/-/-]
fn cb_execute_c3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 E [-/-/-/-]
fn cb_execute_c4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 H [-/-/-/-]
fn cb_execute_c5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 L [-/-/-/-]
fn cb_execute_c6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 0 (HL) [-/-/-/-]
fn cb_execute_c7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 0 A [-/-/-/-]
fn cb_execute_c8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 B [-/-/-/-]
fn cb_execute_c9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 C [-/-/-/-]
fn cb_execute_ca(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 D [-/-/-/-]
fn cb_execute_cb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 E [-/-/-/-]
fn cb_execute_cc(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 H [-/-/-/-]
fn cb_execute_cd(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 L [-/-/-/-]
fn cb_execute_ce(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 1 (HL) [-/-/-/-]
fn cb_execute_cf(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 1 A [-/-/-/-]
fn cb_execute_d0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 B [-/-/-/-]
fn cb_execute_d1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 C [-/-/-/-]
fn cb_execute_d2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 D [-/-/-/-]
fn cb_execute_d3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 E [-/-/-/-]
fn cb_execute_d4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 H [-/-/-/-]
fn cb_execute_d5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 L [-/-/-/-]
fn cb_execute_d6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 2 (HL) [-/-/-/-]
fn cb_execute_d7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 2 A [-/-/-/-]
fn cb_execute_d8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 B [-/-/-/-]
fn cb_execute_d9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 C [-/-/-/-]
fn cb_execute_da(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 D [-/-/-/-]
fn cb_execute_db(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 E [-/-/-/-]
fn cb_execute_dc(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 H [-/-/-/-]
fn cb_execute_dd(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 L [-/-/-/-]
fn cb_execute_de(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 3 (HL) [-/-/-/-]
fn cb_execute_df(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 3 A [-/-/-/-]
fn cb_execute_e0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 B [-/-/-/-]
fn cb_execute_e1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 C [-/-/-/-]
fn cb_execute_e2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 D [-/-/-/-]
fn cb_execute_e3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 E [-/-/-/-]
fn cb_execute_e4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 H [-/-/-/-]
fn cb_execute_e5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 L [-/-/-/-]
fn cb_execute_e6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 4 (HL) [-/-/-/-]
fn cb_execute_e7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 4 A [-/-/-/-]
fn cb_execute_e8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 B [-/-/-/-]
fn cb_execute_e9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 C [-/-/-/-]
fn cb_execute_ea(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 D [-/-/-/-]
fn cb_execute_eb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 E [-/-/-/-]
fn cb_execute_ec(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 H [-/-/-/-]
fn cb_execute_ed(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 L [-/-/-/-]
fn cb_execute_ee(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 5 (HL) [-/-/-/-]
fn cb_execute_ef(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 5 A [-/-/-/-]
fn cb_execute_f0(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 B [-/-/-/-]
fn cb_execute_f1(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 C [-/-/-/-]
fn cb_execute_f2(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 D [-/-/-/-]
fn cb_execute_f3(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 E [-/-/-/-]
fn cb_execute_f4(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 H [-/-/-/-]
fn cb_execute_f5(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 L [-/-/-/-]
fn cb_execute_f6(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 6 (HL) [-/-/-/-]
fn cb_execute_f7(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 6 A [-/-/-/-]
fn cb_execute_f8(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 B [-/-/-/-]
fn cb_execute_f9(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 C [-/-/-/-]
fn cb_execute_fa(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 D [-/-/-/-]
fn cb_execute_fb(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 E [-/-/-/-]
fn cb_execute_fc(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 H [-/-/-/-]
fn cb_execute_fd(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 L [-/-/-/-]
fn cb_execute_fe(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 4;
} // SET 7 (HL) [-/-/-/-]
fn cb_execute_ff(cpu: &mut Cpu) {
    op_unimplemented(cpu);
    cpu.advance_pc = 2;
    cpu.cycles += 2;
} // SET 7 A [-/-/-/-]

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use crate::execute::*;

    #[test]
    fn execute_0c_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x20;
        execute_0c(&mut cpu);
        assert_eq!(cpu.reg.c, 0x21);
    }

    #[test]
    fn execute_0e_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x0e, 0xBE];
        execute_0e(&mut cpu);
        assert_eq!(cpu.reg.c, 0xBE);
    }

    #[test]
    fn execute_11_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x11, 0xBE, 0xEF];
        execute_11(&mut cpu);
        assert_eq!(cpu.reg.de(), 0xEFBE);
    }

    #[test]
    fn execute_20_jmp() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x20, 0x06];
        cpu.reg.f.zero = false;
        execute_20(&mut cpu);
        assert_eq!(cpu.advance_pc, 6);
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
    fn execute_3e_ok() {
        let mut cpu = Cpu::new();
        cpu.mmu.cartridge.data = vec![0x3E, 0xBE];
        execute_3e(&mut cpu);
        assert_eq!(cpu.reg.a, 0xBE);
    }

    #[test]
    fn execute_47_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 20;
        execute_47(&mut cpu);
        assert_eq!(cpu.reg.b, 20);
    }

    #[test]
    fn execute_7c_ok() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 5;
        assert_ne!(cpu.reg.a, cpu.reg.h);
        execute_7c(&mut cpu);
        assert_eq!(cpu.reg.a, cpu.reg.h);
        assert_eq!(cpu.reg.a, 5);
    }

    #[test]
    fn execute_cb_7c_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0b01010101;
        cb_execute_7c(&mut cpu);
        assert_eq!(cpu.reg.f.zero, true);
    }

    #[test]
    fn execute_cb_7c_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0b10101010;
        cb_execute_7c(&mut cpu);
        assert_eq!(cpu.reg.f.zero, false);
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