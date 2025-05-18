use crate::emulator::cpu::{self, Cpu};

pub fn ei(cpu: &mut Cpu) {
    //This there a better way?
    cpu.i_enable_flag = true;
}

pub fn di(cpu: &mut Cpu) {
    cpu.ime = false;
}
