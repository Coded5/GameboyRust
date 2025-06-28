use crate::emulator::{
    bus::Bus,
    cpu::{Cpu, H, N, Z},
};

use super::operand::Operands;

pub fn bit(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, operand2: Operands) {
    let b: u8 = match operand1 {
        Operands::I(val) => val,
        _ => panic!(),
    };

    let test_byte = match operand2 {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => bus.read_byte(cpu.hl()),
        _ => panic!(),
    };

    let bit = (test_byte >> b) & 1;

    cpu.set(Z, bit == 0);
    cpu.set(N, false);
    cpu.set(H, true);
}

pub fn set(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, operand2: Operands) {
    let b: u8 = match operand1 {
        Operands::I(val) => val,
        _ => panic!(),
    };

    let src: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => bus.read_byte(cpu.hl()),
        _ => panic!(),
    };

    let res = src | (1 << b);

    match operand2 {
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        Operands::AddrHL => bus.write_byte(cpu.hl(), res),
        _ => panic!(),
    };
}

pub fn res(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, operand2: Operands) {
    let b: u8 = match operand1 {
        Operands::I(val) => val,
        _ => panic!(),
    };

    let src: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => bus.read_byte(cpu.hl()),
        _ => panic!(),
    };

    let res = src & !(1 << b);

    match operand2 {
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        Operands::AddrHL => bus.write_byte(cpu.hl(), res),
        _ => panic!(),
    };
}
