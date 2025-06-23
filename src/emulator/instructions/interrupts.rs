use crate::emulator::cpu::Cpu;

pub fn ei(cpu: &mut Cpu) {
    cpu.i_enable_flag = true;
}

pub fn di(cpu: &mut Cpu) {
    cpu.ime = false;
}
