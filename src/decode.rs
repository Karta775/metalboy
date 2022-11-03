use super::cpu::Cpu;
use log::{debug, warn};

// Format string representation
fn to_string(opcode: u8, mnemonic: &str, len: u8, cyc: &str, grp: &str) -> String {
    format!("| {:02X} - {:<10} | LEN: {} | CYC: {:<8} | GRP: {:<12} |", opcode, mnemonic, len, cyc, grp)
}

// Unknown opcode
fn unknown(cpu: &Cpu) -> String {
    format!("| {:02X} - {} | Unknown opcode |", cpu.opcode, "???")
}

// Log the decoded opcode appropriately
pub fn log_decode(cpu: &Cpu) {
    if let Some(decoded) = decode(cpu) {
        debug!("{}", decoded);
    } else {
        warn!("{}", unknown(cpu));
    }
}

// Match the opcode and return a formatted String, else return None
pub fn decode(cpu: &Cpu) -> Option<String> {
    match cpu.opcode {
        0x00 => Some(to_string(0x00, "NOP ", 1, "[1]", "control/misc")),
        0x01 => Some(to_string(0x01, "LD BC d16", 3, "[3]", "x16/lsm")),
        0x02 => Some(to_string(0x02, "LD (BC) A", 1, "[2]", "x8/lsm")),
        0x03 => Some(to_string(0x03, "INC BC ", 1, "[2]", "x16/alu")),
        0x04 => Some(to_string(0x04, "INC B ", 1, "[1]", "x8/alu")),
        0x05 => Some(to_string(0x05, "DEC B ", 1, "[1]", "x8/alu")),
        0x06 => Some(to_string(0x06, "LD B d8", 2, "[2]", "x8/lsm")),
        0x07 => Some(to_string(0x07, "RLCA ", 1, "[1]", "x8/rsb")),
        0x08 => Some(to_string(0x08, "LD (a16) SP", 3, "[5]", "x16/lsm")),
        0x09 => Some(to_string(0x09, "ADD HL BC", 1, "[2]", "x16/alu")),
        0x0a => Some(to_string(0x0a, "LD A (BC)", 1, "[2]", "x8/lsm")),
        0x0b => Some(to_string(0x0b, "DEC BC ", 1, "[2]", "x16/alu")),
        0x0c => Some(to_string(0x0c, "INC C ", 1, "[1]", "x8/alu")),
        0x0d => Some(to_string(0x0d, "DEC C ", 1, "[1]", "x8/alu")),
        0x0e => Some(to_string(0x0e, "LD C d8", 2, "[2]", "x8/lsm")),
        0x0f => Some(to_string(0x0f, "RRCA ", 1, "[1]", "x8/rsb")),
        0x10 => Some(to_string(0x10, "STOP 0 ", 1, "[1]", "control/misc")),
        0x11 => Some(to_string(0x11, "LD DE d16", 3, "[3]", "x16/lsm")),
        0x12 => Some(to_string(0x12, "LD (DE) A", 1, "[2]", "x8/lsm")),
        0x13 => Some(to_string(0x13, "INC DE ", 1, "[2]", "x16/alu")),
        0x14 => Some(to_string(0x14, "INC D ", 1, "[1]", "x8/alu")),
        0x15 => Some(to_string(0x15, "DEC D ", 1, "[1]", "x8/alu")),
        0x16 => Some(to_string(0x16, "LD D d8", 2, "[2]", "x8/lsm")),
        0x17 => Some(to_string(0x17, "RLA ", 1, "[1]", "x8/rsb")),
        0x18 => Some(to_string(0x18, "JR r8 ", 2, "[3]", "control/br")),
        0x19 => Some(to_string(0x19, "ADD HL DE", 1, "[2]", "x16/alu")),
        0x1a => Some(to_string(0x1a, "LD A (DE)", 1, "[2]", "x8/lsm")),
        0x1b => Some(to_string(0x1b, "DEC DE ", 1, "[2]", "x16/alu")),
        0x1c => Some(to_string(0x1c, "INC E ", 1, "[1]", "x8/alu")),
        0x1d => Some(to_string(0x1d, "DEC E ", 1, "[1]", "x8/alu")),
        0x1e => Some(to_string(0x1e, "LD E d8", 2, "[2]", "x8/lsm")),
        0x1f => Some(to_string(0x1f, "RRA ", 1, "[1]", "x8/rsb")),
        0x20 => Some(to_string(0x20, "JR NZ r8", 2, "[3, 2]", "control/br")),
        0x21 => Some(to_string(0x21, "LD HL d16", 3, "[3]", "x16/lsm")),
        0x22 => Some(to_string(0x22, "LD (HL+) A", 1, "[2]", "x8/lsm")),
        0x23 => Some(to_string(0x23, "INC HL ", 1, "[2]", "x16/alu")),
        0x24 => Some(to_string(0x24, "INC H ", 1, "[1]", "x8/alu")),
        0x25 => Some(to_string(0x25, "DEC H ", 1, "[1]", "x8/alu")),
        0x26 => Some(to_string(0x26, "LD H d8", 2, "[2]", "x8/lsm")),
        0x27 => Some(to_string(0x27, "DAA ", 1, "[1]", "x8/alu")),
        0x28 => Some(to_string(0x28, "JR Z r8", 2, "[3, 2]", "control/br")),
        0x29 => Some(to_string(0x29, "ADD HL HL", 1, "[2]", "x16/alu")),
        0x2a => Some(to_string(0x2a, "LD A (HL+)", 1, "[2]", "x8/lsm")),
        0x2b => Some(to_string(0x2b, "DEC HL ", 1, "[2]", "x16/alu")),
        0x2c => Some(to_string(0x2c, "INC L ", 1, "[1]", "x8/alu")),
        0x2d => Some(to_string(0x2d, "DEC L ", 1, "[1]", "x8/alu")),
        0x2e => Some(to_string(0x2e, "LD L d8", 2, "[2]", "x8/lsm")),
        0x2f => Some(to_string(0x2f, "CPL ", 1, "[1]", "x8/alu")),
        0x30 => Some(to_string(0x30, "JR NC r8", 2, "[3, 2]", "control/br")),
        0x31 => Some(to_string(0x31, "LD SP d16", 3, "[3]", "x16/lsm")),
        0x32 => Some(to_string(0x32, "LD (HL-) A", 1, "[2]", "x8/lsm")),
        0x33 => Some(to_string(0x33, "INC SP ", 1, "[2]", "x16/alu")),
        0x34 => Some(to_string(0x34, "INC (HL) ", 1, "[3]", "x8/alu")),
        0x35 => Some(to_string(0x35, "DEC (HL) ", 1, "[3]", "x8/alu")),
        0x36 => Some(to_string(0x36, "LD (HL) d8", 2, "[3]", "x8/lsm")),
        0x37 => Some(to_string(0x37, "SCF ", 1, "[1]", "x8/alu")),
        0x38 => Some(to_string(0x38, "JR C r8", 2, "[3, 2]", "control/br")),
        0x39 => Some(to_string(0x39, "ADD HL SP", 1, "[2]", "x16/alu")),
        0x3a => Some(to_string(0x3a, "LD A (HL-)", 1, "[2]", "x8/lsm")),
        0x3b => Some(to_string(0x3b, "DEC SP ", 1, "[2]", "x16/alu")),
        0x3c => Some(to_string(0x3c, "INC A ", 1, "[1]", "x8/alu")),
        0x3d => Some(to_string(0x3d, "DEC A ", 1, "[1]", "x8/alu")),
        0x3e => Some(to_string(0x3e, "LD A d8", 2, "[2]", "x8/lsm")),
        0x3f => Some(to_string(0x3f, "CCF ", 1, "[1]", "x8/alu")),
        0x40 => Some(to_string(0x40, "LD B B", 1, "[1]", "x8/lsm")),
        0x41 => Some(to_string(0x41, "LD B C", 1, "[1]", "x8/lsm")),
        0x42 => Some(to_string(0x42, "LD B D", 1, "[1]", "x8/lsm")),
        0x43 => Some(to_string(0x43, "LD B E", 1, "[1]", "x8/lsm")),
        0x44 => Some(to_string(0x44, "LD B H", 1, "[1]", "x8/lsm")),
        0x45 => Some(to_string(0x45, "LD B L", 1, "[1]", "x8/lsm")),
        0x46 => Some(to_string(0x46, "LD B (HL)", 1, "[2]", "x8/lsm")),
        0x47 => Some(to_string(0x47, "LD B A", 1, "[1]", "x8/lsm")),
        0x48 => Some(to_string(0x48, "LD C B", 1, "[1]", "x8/lsm")),
        0x49 => Some(to_string(0x49, "LD C C", 1, "[1]", "x8/lsm")),
        0x4a => Some(to_string(0x4a, "LD C D", 1, "[1]", "x8/lsm")),
        0x4b => Some(to_string(0x4b, "LD C E", 1, "[1]", "x8/lsm")),
        0x4c => Some(to_string(0x4c, "LD C H", 1, "[1]", "x8/lsm")),
        0x4d => Some(to_string(0x4d, "LD C L", 1, "[1]", "x8/lsm")),
        0x4e => Some(to_string(0x4e, "LD C (HL)", 1, "[2]", "x8/lsm")),
        0x4f => Some(to_string(0x4f, "LD C A", 1, "[1]", "x8/lsm")),
        0x50 => Some(to_string(0x50, "LD D B", 1, "[1]", "x8/lsm")),
        0x51 => Some(to_string(0x51, "LD D C", 1, "[1]", "x8/lsm")),
        0x52 => Some(to_string(0x52, "LD D D", 1, "[1]", "x8/lsm")),
        0x53 => Some(to_string(0x53, "LD D E", 1, "[1]", "x8/lsm")),
        0x54 => Some(to_string(0x54, "LD D H", 1, "[1]", "x8/lsm")),
        0x55 => Some(to_string(0x55, "LD D L", 1, "[1]", "x8/lsm")),
        0x56 => Some(to_string(0x56, "LD D (HL)", 1, "[2]", "x8/lsm")),
        0x57 => Some(to_string(0x57, "LD D A", 1, "[1]", "x8/lsm")),
        0x58 => Some(to_string(0x58, "LD E B", 1, "[1]", "x8/lsm")),
        0x59 => Some(to_string(0x59, "LD E C", 1, "[1]", "x8/lsm")),
        0x5a => Some(to_string(0x5a, "LD E D", 1, "[1]", "x8/lsm")),
        0x5b => Some(to_string(0x5b, "LD E E", 1, "[1]", "x8/lsm")),
        0x5c => Some(to_string(0x5c, "LD E H", 1, "[1]", "x8/lsm")),
        0x5d => Some(to_string(0x5d, "LD E L", 1, "[1]", "x8/lsm")),
        0x5e => Some(to_string(0x5e, "LD E (HL)", 1, "[2]", "x8/lsm")),
        0x5f => Some(to_string(0x5f, "LD E A", 1, "[1]", "x8/lsm")),
        0x60 => Some(to_string(0x60, "LD H B", 1, "[1]", "x8/lsm")),
        0x61 => Some(to_string(0x61, "LD H C", 1, "[1]", "x8/lsm")),
        0x62 => Some(to_string(0x62, "LD H D", 1, "[1]", "x8/lsm")),
        0x63 => Some(to_string(0x63, "LD H E", 1, "[1]", "x8/lsm")),
        0x64 => Some(to_string(0x64, "LD H H", 1, "[1]", "x8/lsm")),
        0x65 => Some(to_string(0x65, "LD H L", 1, "[1]", "x8/lsm")),
        0x66 => Some(to_string(0x66, "LD H (HL)", 1, "[2]", "x8/lsm")),
        0x67 => Some(to_string(0x67, "LD H A", 1, "[1]", "x8/lsm")),
        0x68 => Some(to_string(0x68, "LD L B", 1, "[1]", "x8/lsm")),
        0x69 => Some(to_string(0x69, "LD L C", 1, "[1]", "x8/lsm")),
        0x6a => Some(to_string(0x6a, "LD L D", 1, "[1]", "x8/lsm")),
        0x6b => Some(to_string(0x6b, "LD L E", 1, "[1]", "x8/lsm")),
        0x6c => Some(to_string(0x6c, "LD L H", 1, "[1]", "x8/lsm")),
        0x6d => Some(to_string(0x6d, "LD L L", 1, "[1]", "x8/lsm")),
        0x6e => Some(to_string(0x6e, "LD L (HL)", 1, "[2]", "x8/lsm")),
        0x6f => Some(to_string(0x6f, "LD L A", 1, "[1]", "x8/lsm")),
        0x70 => Some(to_string(0x70, "LD (HL) B", 1, "[2]", "x8/lsm")),
        0x71 => Some(to_string(0x71, "LD (HL) C", 1, "[2]", "x8/lsm")),
        0x72 => Some(to_string(0x72, "LD (HL) D", 1, "[2]", "x8/lsm")),
        0x73 => Some(to_string(0x73, "LD (HL) E", 1, "[2]", "x8/lsm")),
        0x74 => Some(to_string(0x74, "LD (HL) H", 1, "[2]", "x8/lsm")),
        0x75 => Some(to_string(0x75, "LD (HL) L", 1, "[2]", "x8/lsm")),
        0x76 => Some(to_string(0x76, "HALT ", 1, "[1]", "control/misc")),
        0x77 => Some(to_string(0x77, "LD (HL) A", 1, "[2]", "x8/lsm")),
        0x78 => Some(to_string(0x78, "LD A B", 1, "[1]", "x8/lsm")),
        0x79 => Some(to_string(0x79, "LD A C", 1, "[1]", "x8/lsm")),
        0x7a => Some(to_string(0x7a, "LD A D", 1, "[1]", "x8/lsm")),
        0x7b => Some(to_string(0x7b, "LD A E", 1, "[1]", "x8/lsm")),
        0x7c => Some(to_string(0x7c, "LD A H", 1, "[1]", "x8/lsm")),
        0x7d => Some(to_string(0x7d, "LD A L", 1, "[1]", "x8/lsm")),
        0x7e => Some(to_string(0x7e, "LD A (HL)", 1, "[2]", "x8/lsm")),
        0x7f => Some(to_string(0x7f, "LD A A", 1, "[1]", "x8/lsm")),
        0x80 => Some(to_string(0x80, "ADD A B", 1, "[1]", "x8/alu")),
        0x81 => Some(to_string(0x81, "ADD A C", 1, "[1]", "x8/alu")),
        0x82 => Some(to_string(0x82, "ADD A D", 1, "[1]", "x8/alu")),
        0x83 => Some(to_string(0x83, "ADD A E", 1, "[1]", "x8/alu")),
        0x84 => Some(to_string(0x84, "ADD A H", 1, "[1]", "x8/alu")),
        0x85 => Some(to_string(0x85, "ADD A L", 1, "[1]", "x8/alu")),
        0x86 => Some(to_string(0x86, "ADD A (HL)", 1, "[2]", "x8/alu")),
        0x87 => Some(to_string(0x87, "ADD A A", 1, "[1]", "x8/alu")),
        0x88 => Some(to_string(0x88, "ADC A B", 1, "[1]", "x8/alu")),
        0x89 => Some(to_string(0x89, "ADC A C", 1, "[1]", "x8/alu")),
        0x8a => Some(to_string(0x8a, "ADC A D", 1, "[1]", "x8/alu")),
        0x8b => Some(to_string(0x8b, "ADC A E", 1, "[1]", "x8/alu")),
        0x8c => Some(to_string(0x8c, "ADC A H", 1, "[1]", "x8/alu")),
        0x8d => Some(to_string(0x8d, "ADC A L", 1, "[1]", "x8/alu")),
        0x8e => Some(to_string(0x8e, "ADC A (HL)", 1, "[2]", "x8/alu")),
        0x8f => Some(to_string(0x8f, "ADC A A", 1, "[1]", "x8/alu")),
        0x90 => Some(to_string(0x90, "SUB B ", 1, "[1]", "x8/alu")),
        0x91 => Some(to_string(0x91, "SUB C ", 1, "[1]", "x8/alu")),
        0x92 => Some(to_string(0x92, "SUB D ", 1, "[1]", "x8/alu")),
        0x93 => Some(to_string(0x93, "SUB E ", 1, "[1]", "x8/alu")),
        0x94 => Some(to_string(0x94, "SUB H ", 1, "[1]", "x8/alu")),
        0x95 => Some(to_string(0x95, "SUB L ", 1, "[1]", "x8/alu")),
        0x96 => Some(to_string(0x96, "SUB (HL) ", 1, "[2]", "x8/alu")),
        0x97 => Some(to_string(0x97, "SUB A ", 1, "[1]", "x8/alu")),
        0x98 => Some(to_string(0x98, "SBC A B", 1, "[1]", "x8/alu")),
        0x99 => Some(to_string(0x99, "SBC A C", 1, "[1]", "x8/alu")),
        0x9a => Some(to_string(0x9a, "SBC A D", 1, "[1]", "x8/alu")),
        0x9b => Some(to_string(0x9b, "SBC A E", 1, "[1]", "x8/alu")),
        0x9c => Some(to_string(0x9c, "SBC A H", 1, "[1]", "x8/alu")),
        0x9d => Some(to_string(0x9d, "SBC A L", 1, "[1]", "x8/alu")),
        0x9e => Some(to_string(0x9e, "SBC A (HL)", 1, "[2]", "x8/alu")),
        0x9f => Some(to_string(0x9f, "SBC A A", 1, "[1]", "x8/alu")),
        0xa0 => Some(to_string(0xa0, "AND B ", 1, "[1]", "x8/alu")),
        0xa1 => Some(to_string(0xa1, "AND C ", 1, "[1]", "x8/alu")),
        0xa2 => Some(to_string(0xa2, "AND D ", 1, "[1]", "x8/alu")),
        0xa3 => Some(to_string(0xa3, "AND E ", 1, "[1]", "x8/alu")),
        0xa4 => Some(to_string(0xa4, "AND H ", 1, "[1]", "x8/alu")),
        0xa5 => Some(to_string(0xa5, "AND L ", 1, "[1]", "x8/alu")),
        0xa6 => Some(to_string(0xa6, "AND (HL) ", 1, "[2]", "x8/alu")),
        0xa7 => Some(to_string(0xa7, "AND A ", 1, "[1]", "x8/alu")),
        0xa8 => Some(to_string(0xa8, "XOR B ", 1, "[1]", "x8/alu")),
        0xa9 => Some(to_string(0xa9, "XOR C ", 1, "[1]", "x8/alu")),
        0xaa => Some(to_string(0xaa, "XOR D ", 1, "[1]", "x8/alu")),
        0xab => Some(to_string(0xab, "XOR E ", 1, "[1]", "x8/alu")),
        0xac => Some(to_string(0xac, "XOR H ", 1, "[1]", "x8/alu")),
        0xad => Some(to_string(0xad, "XOR L ", 1, "[1]", "x8/alu")),
        0xae => Some(to_string(0xae, "XOR (HL) ", 1, "[2]", "x8/alu")),
        0xaf => Some(to_string(0xaf, "XOR A ", 1, "[1]", "x8/alu")),
        0xb0 => Some(to_string(0xb0, "OR B ", 1, "[1]", "x8/alu")),
        0xb1 => Some(to_string(0xb1, "OR C ", 1, "[1]", "x8/alu")),
        0xb2 => Some(to_string(0xb2, "OR D ", 1, "[1]", "x8/alu")),
        0xb3 => Some(to_string(0xb3, "OR E ", 1, "[1]", "x8/alu")),
        0xb4 => Some(to_string(0xb4, "OR H ", 1, "[1]", "x8/alu")),
        0xb5 => Some(to_string(0xb5, "OR L ", 1, "[1]", "x8/alu")),
        0xb6 => Some(to_string(0xb6, "OR (HL) ", 1, "[2]", "x8/alu")),
        0xb7 => Some(to_string(0xb7, "OR A ", 1, "[1]", "x8/alu")),
        0xb8 => Some(to_string(0xb8, "CP B ", 1, "[1]", "x8/alu")),
        0xb9 => Some(to_string(0xb9, "CP C ", 1, "[1]", "x8/alu")),
        0xba => Some(to_string(0xba, "CP D ", 1, "[1]", "x8/alu")),
        0xbb => Some(to_string(0xbb, "CP E ", 1, "[1]", "x8/alu")),
        0xbc => Some(to_string(0xbc, "CP H ", 1, "[1]", "x8/alu")),
        0xbd => Some(to_string(0xbd, "CP L ", 1, "[1]", "x8/alu")),
        0xbe => Some(to_string(0xbe, "CP (HL) ", 1, "[2]", "x8/alu")),
        0xbf => Some(to_string(0xbf, "CP A ", 1, "[1]", "x8/alu")),
        0xc0 => Some(to_string(0xc0, "RET NZ ", 1, "[5, 2]", "control/br")),
        0xc1 => Some(to_string(0xc1, "POP BC ", 1, "[3]", "x16/lsm")),
        0xc2 => Some(to_string(0xc2, "JP NZ a16", 3, "[4, 3]", "control/br")),
        0xc3 => Some(to_string(0xc3, "JP a16 ", 3, "[4]", "control/br")),
        0xc4 => Some(to_string(0xc4, "CALL NZ a16", 3, "[6, 3]", "control/br")),
        0xc5 => Some(to_string(0xc5, "PUSH BC ", 1, "[4]", "x16/lsm")),
        0xc6 => Some(to_string(0xc6, "ADD A d8", 2, "[2]", "x8/alu")),
        0xc7 => Some(to_string(0xc7, "RST 00H ", 1, "[4]", "control/br")),
        0xc8 => Some(to_string(0xc8, "RET Z ", 1, "[5, 2]", "control/br")),
        0xc9 => Some(to_string(0xc9, "RET ", 1, "[4]", "control/br")),
        0xca => Some(to_string(0xca, "JP Z a16", 3, "[4, 3]", "control/br")),
        0xcb => Some(to_string(0xcb, "PREFIX CB ", 1, "[1]", "control/misc")),
        0xcc => Some(to_string(0xcc, "CALL Z a16", 3, "[6, 3]", "control/br")),
        0xcd => Some(to_string(0xcd, "CALL a16 ", 3, "[6]", "control/br")),
        0xce => Some(to_string(0xce, "ADC A d8", 2, "[2]", "x8/alu")),
        0xcf => Some(to_string(0xcf, "RST 08H ", 1, "[4]", "control/br")),
        0xd0 => Some(to_string(0xd0, "RET NC ", 1, "[5, 2]", "control/br")),
        0xd1 => Some(to_string(0xd1, "POP DE ", 1, "[3]", "x16/lsm")),
        0xd2 => Some(to_string(0xd2, "JP NC a16", 3, "[4, 3]", "control/br")),
        0xd4 => Some(to_string(0xd4, "CALL NC a16", 3, "[6, 3]", "control/br")),
        0xd5 => Some(to_string(0xd5, "PUSH DE ", 1, "[4]", "x16/lsm")),
        0xd6 => Some(to_string(0xd6, "SUB d8 ", 2, "[2]", "x8/alu")),
        0xd7 => Some(to_string(0xd7, "RST 10H ", 1, "[4]", "control/br")),
        0xd8 => Some(to_string(0xd8, "RET C ", 1, "[5, 2]", "control/br")),
        0xd9 => Some(to_string(0xd9, "RETI ", 1, "[4]", "control/br")),
        0xda => Some(to_string(0xda, "JP C a16", 3, "[4, 3]", "control/br")),
        0xdc => Some(to_string(0xdc, "CALL C a16", 3, "[6, 3]", "control/br")),
        0xde => Some(to_string(0xde, "SBC A d8", 2, "[2]", "x8/alu")),
        0xdf => Some(to_string(0xdf, "RST 18H ", 1, "[4]", "control/br")),
        0xe0 => Some(to_string(0xe0, "LDH (a8) A", 2, "[3]", "x8/lsm")),
        0xe1 => Some(to_string(0xe1, "POP HL ", 1, "[3]", "x16/lsm")),
        0xe2 => Some(to_string(0xe2, "LD (C) A", 1, "[2]", "x8/lsm")),
        0xe5 => Some(to_string(0xe5, "PUSH HL ", 1, "[4]", "x16/lsm")),
        0xe6 => Some(to_string(0xe6, "AND d8 ", 2, "[2]", "x8/alu")),
        0xe7 => Some(to_string(0xe7, "RST 20H ", 1, "[4]", "control/br")),
        0xe8 => Some(to_string(0xe8, "ADD SP r8", 2, "[4]", "x16/alu")),
        0xe9 => Some(to_string(0xe9, "JP (HL) ", 1, "[1]", "control/br")),
        0xea => Some(to_string(0xea, "LD (a16) A", 3, "[4]", "x8/lsm")),
        0xee => Some(to_string(0xee, "XOR d8 ", 2, "[2]", "x8/alu")),
        0xef => Some(to_string(0xef, "RST 28H ", 1, "[4]", "control/br")),
        0xf0 => Some(to_string(0xf0, "LDH A (a8)", 2, "[3]", "x8/lsm")),
        0xf1 => Some(to_string(0xf1, "POP AF ", 1, "[3]", "x16/lsm")),
        0xf2 => Some(to_string(0xf2, "LD A (C)", 1, "[2]", "x8/lsm")),
        0xf3 => Some(to_string(0xf3, "DI ", 1, "[1]", "control/misc")),
        0xf5 => Some(to_string(0xf5, "PUSH AF ", 1, "[4]", "x16/lsm")),
        0xf6 => Some(to_string(0xf6, "OR d8 ", 2, "[2]", "x8/alu")),
        0xf7 => Some(to_string(0xf7, "RST 30H ", 1, "[4]", "control/br")),
        0xf8 => Some(to_string(0xf8, "LD HL SP+r8", 2, "[3]", "x16/lsm")),
        0xf9 => Some(to_string(0xf9, "LD SP HL", 1, "[2]", "x16/lsm")),
        0xfa => Some(to_string(0xfa, "LD A (a16)", 3, "[4]", "x8/lsm")),
        0xfb => Some(to_string(0xfb, "EI ", 1, "[1]", "control/misc")),
        0xfe => Some(to_string(0xfe, "CP d8 ", 2, "[2]", "x8/alu")),
        0xff => Some(to_string(0xff, "RST 38H ", 1, "[4]", "control/br")),
        _ => None
    }
}
