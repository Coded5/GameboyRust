use crate::emulator::{bus::Bus, cpu::Cpu};

use super::operand::Operands;

pub fn jpnn(cpu: &mut Cpu, bus: &mut Bus) {
    let address = cpu.next_short(bus);
    cpu.pc = address;
}

pub fn jphl(cpu: &mut Cpu, _bus: &mut Bus) -> bool {
    cpu.pc = cpu.hl();

    //NOTE: JP HL always branch
    false
}

pub fn jpccnn(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, _operand2: Operands) -> bool {
    let condition = match operand1 {
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_Z => cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => true,
    };

    let address = cpu.next_short(bus);
    if condition {
        cpu.pc = address;

        return true;
    }

    false
}

pub fn jr(cpu: &mut Cpu, bus: &mut Bus) {
    let offset = cpu.next_byte(bus) as i8;

    let (res, _carry) = cpu.pc.overflowing_add_signed(offset as i16);

    cpu.pc = res;
}

pub fn jrccn(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, _operand2: Operands) -> bool {
    let condition = match operand1 {
        Operands::JR_Z => cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => true,
    };

    if condition {
        jr(cpu, bus);
        true
    } else {
        cpu.pc += 1;
        false
    }
}
