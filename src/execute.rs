use super::cpu::Cpu;
use super::decode::decode;
use log::{debug, warn};

fn op_implemented(cpu: &Cpu) {
    debug!("{}", decode(cpu).expect("Unknown opcode"));
}

fn op_unimplemented(cpu: &Cpu) {
    warn!("{}", decode(cpu).expect("Unknown opcode"));
}

pub fn execute(cpu: &mut Cpu) {
    match cpu.opcode {
        0x00 => {
            op_implemented(cpu);
        },
        _ => op_unimplemented(cpu)
    }
}
