use crate::emulator::{cpu::Cpu, memory::Memory};

use super::operand::Operands;

pub fn jpnn(cpu: &mut Cpu, memory: &mut Memory) {
    let address = cpu.next_short(memory);
    cpu.pc = address;
}

pub fn jphl(cpu: &mut Cpu, _memory: &mut Memory) -> bool {
    cpu.pc = cpu.hl();

    //NOTE: JP HL always branch
    false
}

pub fn jpccnn(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, _operand2: Operands) -> bool {
    let condition = match operand1 {
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_Z => cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => true,
    };

    let address = cpu.next_short(memory);
    if (condition) {
        cpu.pc = address;

        return true;
    }

    false
}

pub fn jr(cpu: &mut Cpu, memory: &mut Memory) {
    let offset = cpu.next_byte(memory) as i8;

    let (res, _carry) = cpu.pc.overflowing_add_signed(offset as i16);

    cpu.pc = res;
}

pub fn jrccn(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) -> bool {
    let condition = match operand1 {
        Operands::JR_Z => cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => true,
    };

    if (condition) {
        jr(cpu, memory);
        true
    } else {
        //TODO: For some reason when relative jump condition is not met cpu sp doesn't increment
        cpu.pc += 1;
        false
    }
}
