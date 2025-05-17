use crate::emulator::{
    cpu::{Cpu, C, H, N},
    memory::Memory,
};

use super::operand::Operands;

pub fn add(cpu: &mut Cpu, _memory: &mut Memory, operand1: Operands, operand2: Operands) {
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
