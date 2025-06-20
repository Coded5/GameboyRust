use crate::emulator::{
    cpu::{Cpu, C, H, N, Z},
    memory::Memory,
};

use super::operand::Operands;

pub fn swap(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        _ => panic!(),
    };

    let lo_nib = src & 0xF;
    let hi_nib = (src >> 4) & 0xF;
    let res = (lo_nib << 4) | hi_nib;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, false);

    match operand {
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        _ => panic!(),
    };
}

pub fn cpl(cpu: &mut Cpu) {
    cpu.a = !cpu.a;
}

pub fn ccf(cpu: &mut Cpu) {
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, !cpu.c());
}

pub fn scf(cpu: &mut Cpu) {
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, true);
}
