use crate::emulator::{cpu::Cpu, gameboy::Shared, interrupt::InterruptState};

pub fn ei(cpu: &mut Cpu) {
    cpu.i_enable_flag = true;
}

pub fn di(interrupt: Shared<InterruptState>) {
    interrupt.borrow_mut().ime = false;
}
