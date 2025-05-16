use crate::emulator::cpu::{self, Cpu};

//TODO: Delay this by 1 instruction
pub fn ei(cpu: &mut Cpu) {
    cpu.ime = true;
}

pub fn di(cpu: &mut Cpu) {
    cpu.ime = false;
}
