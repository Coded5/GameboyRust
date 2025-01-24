use crate::emulator::{cpu::Cpu, memory::Memory};

use super::operand::Operands;

pub fn jpnn(cpu: &mut Cpu, memory: &mut Memory) { 
    let address = cpu.next_short(memory);
    cpu.pc = address;
}

pub fn jphl(cpu: &mut Cpu, _memory: &mut Memory) { 
    cpu.pc = cpu.hl();
}

pub fn jpccnn(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, _operand2: Operands) {
    let condition = match operand1 {
        Operands::JR_Z =>   cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C =>   cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => panic!(),
    };

    if (condition) {
        let address = cpu.next_short(memory);
        cpu.pc = address;
    }
}

pub fn jr(cpu: &mut Cpu, memory: &mut Memory) {
    let offset = cpu.next_byte(memory) as i16;

    let (res, _carry) = cpu.pc.overflowing_add_signed(offset);

    cpu.pc = res;
}

pub fn jrccn(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {

    let condition = match operand1 {
        Operands::JR_Z =>   cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C =>   cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => panic!(),
    };

    if (condition) {
        jr(cpu, memory);
    }

}
