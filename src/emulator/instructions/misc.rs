use crate::emulator::{cpu::{Cpu, Z, N, H, C}, memory::Memory};

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
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        _ => panic!(),
    };

    let lo_nib = src & 0xF;
    let hi_nib = (src >> 4) & 0xF;
    let res = (lo_nib << 4) | hi_nib;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, false);

    let dest: &mut u8 = match operand {
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
