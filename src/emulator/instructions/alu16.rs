use crate::emulator::{
    cpu::{Cpu, C, H, N, Z},
    memory::Memory,
};

use super::operand::Operands;

pub fn add_sp_i8(cpu: &mut Cpu, memory: &mut Memory) {
    let rhs_raw = cpu.next_byte(memory);
    let rhs = rhs_raw as i8;

    let (res, carry) = cpu.sp.overflowing_add_signed(rhs as i16);

    let sp_lo = cpu.sp as u8;

    cpu.set(Z, false);
    cpu.set(N, false);
    cpu.set(H, ((sp_lo & 0xF) + (rhs_raw & 0xF)) > 0x0F);
    cpu.set(C, ((sp_lo as u16) + (rhs_raw as u16)) > 0xFF);

    cpu.sp = res;
}

pub fn add(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {
    if (operand1 == Operands::SP && operand2 == Operands::I8) {
        add_sp_i8(cpu, memory);
        return;
    }

    let rhs = match operand2 {
        Operands::BC => cpu.bc(),
        Operands::DE => cpu.de(),
        Operands::HL => cpu.hl(),
        Operands::SP => cpu.sp,
        _ => panic!(),
    };

    let lhs = match operand1 {
        Operands::HL => cpu.hl(),
        Operands::SP => cpu.sp,
        _ => panic!(),
    };

    let (res, carry) = lhs.overflowing_add(rhs);

    cpu.set(N, false);
    cpu.set(H, (((lhs & 0xFFF) + (rhs & 0xFFF)) & 0x1000) == 0x1000);
    cpu.set(C, carry);

    match operand1 {
        Operands::HL => cpu.set_hl(res),
        Operands::SP => cpu.sp = res,
        _ => panic!(),
    }
}

pub fn inc(cpu: &mut Cpu, _memory: &mut Memory, operand1: Operands) {
    let rhs = match operand1 {
        Operands::BC => cpu.bc(),
        Operands::DE => cpu.de(),
        Operands::HL => cpu.hl(),
        Operands::SP => cpu.sp,
        _ => panic!(),
    };

    match operand1 {
        Operands::BC => cpu.set_bc(rhs.overflowing_add(1).0),
        Operands::DE => cpu.set_de(rhs.overflowing_add(1).0),
        Operands::HL => cpu.set_hl(rhs.overflowing_add(1).0),
        Operands::SP => cpu.sp = rhs.overflowing_add(1).0,
        _ => panic!(),
    };
}

pub fn dec(cpu: &mut Cpu, _memory: &mut Memory, operand1: Operands) {
    let rhs = match operand1 {
        Operands::BC => cpu.bc(),
        Operands::DE => cpu.de(),
        Operands::HL => cpu.hl(),
        Operands::SP => cpu.sp,
        _ => panic!(),
    };

    match operand1 {
        Operands::BC => cpu.set_bc(rhs.overflowing_sub(1).0),
        Operands::DE => cpu.set_de(rhs.overflowing_sub(1).0),
        Operands::HL => cpu.set_hl(rhs.overflowing_sub(1).0),
        Operands::SP => cpu.sp = rhs.overflowing_sub(1).0,
        _ => panic!(),
    };
}
