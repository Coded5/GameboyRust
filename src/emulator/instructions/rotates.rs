use crate::emulator::{cpu::{Cpu, C, H, N, Z}, memory::{self, Memory}};

use super::operand::Operands;

pub fn rlca(cpu: &mut Cpu) {
    let msb = (cpu.a >> 7) & 1;

    let res = (cpu.a << 1) | msb;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    cpu.a = res;
}

pub fn rla(cpu: &mut Cpu) {
    let msb = (cpu.a >> 7) & 1;
    let c = if cpu.c() { 1 } else { 0 };

    let res = (cpu.a << 1) | c;
    
    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    cpu.a = res;
}

pub fn rrca(cpu: &mut Cpu) {
    let lsb = cpu.a & 1;

    let res = (cpu.a >> 1) | (lsb << 7);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, lsb == 1);

    cpu.a = res;
}

pub fn rra(cpu: &mut Cpu) {
    let lsb = cpu.a & 1;
    let c = if cpu.c() { 1 } else { 0 };

    let res = (cpu.a >> 1) | (c << 7);

    cpu.set(Z, res == 0);    
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, lsb == 1);

    cpu.a = res;
}

pub fn rlc(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {

    let src: u8 = match operand {
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

    let msb = (src >> 7) & 1;
    let res = (src << 1) | msb;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    let dest: &mut u8 = match operand {
        Operands::AddrHL => memory.get_mut_byte(cpu.hl()),
        Operands::A => &mut cpu.a,
        Operands::B => &mut cpu.b,
        Operands::C => &mut cpu.c,
        Operands::D => &mut cpu.d,
        Operands::E => &mut cpu.e,
        Operands::H => &mut cpu.h,
        Operands::L => &mut cpu.l,
        _ => panic!(),
    };

    *dest = res;
}

pub fn rrc(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {

    let src: u8 = match operand {
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

}
