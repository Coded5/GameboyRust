use crate::emulator::{cpu::{Cpu, Z, N, H}, memory::Memory};

use super::operand::Operands;

pub fn bit(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {

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
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        _ => panic!(),
    };

    let bit = (test_byte >> b) & 1;

    cpu.set(Z, bit == 0);
    cpu.set(N, false);
    cpu.set(H, true);
}

pub fn set(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {

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
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        _ => panic!(),
    };

    let res = src | (1 << b);

    let dest: &mut u8 = match operand2 {
        Operands::A => &mut cpu.a,
        Operands::B => &mut cpu.b,
        Operands::C => &mut cpu.c,
        Operands::D => &mut cpu.d,
        Operands::E => &mut cpu.e,
        Operands::H => &mut cpu.h,
        Operands::L => &mut cpu.l,
        Operands::AddrHL => memory.get_mut_byte(cpu.hl()),
        _ => panic!(),
    };

    *dest = res;
}

pub fn res(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {

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
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        _ => panic!(),
    };

    let res = src & !(1 << b);

    let dest: &mut u8 = match operand2 {
        Operands::A => &mut cpu.a,
        Operands::B => &mut cpu.b,
        Operands::C => &mut cpu.c,
        Operands::D => &mut cpu.d,
        Operands::E => &mut cpu.e,
        Operands::H => &mut cpu.h,
        Operands::L => &mut cpu.l,
        Operands::AddrHL => memory.get_mut_byte(cpu.hl()),
        _ => panic!(),
    };

    *dest = res;
}
