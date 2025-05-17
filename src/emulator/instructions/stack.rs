use crate::emulator::{cpu::Cpu, memory::Memory};

use super::operand::Operands;

pub fn push(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u16 = match operand {
        Operands::AF => cpu.af(),
        Operands::BC => cpu.bc(),
        Operands::DE => cpu.de(),
        Operands::HL => cpu.hl(),
        _ => panic!(),
    };

    let lo: u8 = (src & 0xFF) as u8;
    let hi: u8 = ((src >> 8) & 0xFF) as u8;

    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = hi;
    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = lo;
}

pub fn pop(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let lo = memory.get_byte(cpu.sp) as u16;
    cpu.sp += 1;
    let hi = memory.get_byte(cpu.sp) as u16;
    cpu.sp += 1;

    let res = (hi << 8) | lo;

    match operand {
        Operands::AF => cpu.set_af(res),
        Operands::BC => cpu.set_bc(res),
        Operands::DE => cpu.set_de(res),
        Operands::HL => cpu.set_hl(res),
        _ => panic!(),
    }
}

pub fn call(cpu: &mut Cpu, memory: &mut Memory) {
    let address = cpu.pc;
    let lo_byte: u8 = (address & 0xFF) as u8;
    let hi_byte: u8 = ((address >> 8) & 0xFF) as u8;

    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = hi_byte;
    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = lo_byte;

    let new_address = cpu.next_short(memory);
    cpu.pc = new_address;
}

pub fn call_cc(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) -> bool {
    let condition = match operand {
        Operands::JR_Z => cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        Operands::U16 => true,
        _ => panic!(),
    };

    let new_address = cpu.next_short(memory);

    if (!condition) {
        return false;
    }

    let address = cpu.pc;
    let lo_byte: u8 = (address & 0xFF) as u8;
    let hi_byte: u8 = ((address >> 8) & 0xFF) as u8;

    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = hi_byte;
    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = lo_byte;

    cpu.pc = new_address;

    true
}

pub fn ret(cpu: &mut Cpu, memory: &mut Memory) {
    let lo = memory.get_byte(cpu.sp) as u16;
    cpu.sp += 1;
    let hi = memory.get_byte(cpu.sp) as u16;
    cpu.sp += 1;

    cpu.pc = (hi << 8) | lo;
}

pub fn ret_cc(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) -> bool {
    let condition = match operand {
        Operands::JR_Z => cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => true,
    };

    if (!condition) {
        return false;
    }

    ret(cpu, memory);
    true
}

pub fn rst(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) -> bool {
    let address = match operand {
        Operands::H28 => 0x28,
        Operands::H00 => 0x00,
        Operands::H08 => 0x08,
        Operands::H20 => 0x20,
        Operands::H18 => 0x18,
        Operands::H38 => 0x38,
        Operands::H30 => 0x30,
        Operands::H10 => 0x10,
        _ => panic!(),
    };

    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = ((cpu.pc >> 8) & 0xFF) as u8;
    cpu.sp -= 1;
    *memory.get_mut_byte(cpu.sp) = (cpu.pc & 0xFF) as u8;

    cpu.pc = address;
    false
}
