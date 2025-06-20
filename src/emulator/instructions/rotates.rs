use crate::emulator::{
    cpu::{Cpu, C, H, N, Z},
    memory::Memory,
};

use super::operand::Operands;

pub fn rlca(cpu: &mut Cpu) {
    let msb = (cpu.a >> 7) & 1;

    let res = (cpu.a << 1) | msb;

    cpu.set(Z, false);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    cpu.a = res;
}

pub fn rla(cpu: &mut Cpu) {
    let msb = (cpu.a >> 7) & 1;
    let c = if cpu.c() { 1 } else { 0 };

    let res = (cpu.a << 1) | c;

    cpu.set(Z, false);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    cpu.a = res;
}

pub fn rrca(cpu: &mut Cpu) {
    let lsb = cpu.a & 1;

    let res = (cpu.a >> 1) | (lsb << 7);

    cpu.set(Z, false);
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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}

pub fn rrc(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u8 = match operand {
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

    let lsb = src & 0x1;
    let res = (src >> 1) | (lsb << 7);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, lsb == 1);

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}

pub fn rr(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u8 = match operand {
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

    let lsb = src & 0x1;
    let c = if cpu.c() { 1 } else { 0 };
    let res = (src >> 1) | (c << 7);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, lsb == 1);

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}

pub fn rl(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u8 = match operand {
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

    let msb = (src >> 7) & 0x1;
    let c = if cpu.c() { 1 } else { 0 };
    let res = (src << 1) | c;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}

pub fn sla(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u8 = match operand {
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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
    let res = src << 1;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, msb == 1);

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}

pub fn sra(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u8 = match operand {
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

    //NOTE: Because rust perform LSR on unsigned variable
    //So I have to add msb back manually to make it ASR
    let msb = (src >> 7) & 1;
    let lsb = src & 1;
    let res = (src >> 1) | (msb << 7);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, lsb == 1);

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}

pub fn srl(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let src: u8 = match operand {
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        _ => panic!(),
    };

    let lsb = src & 1;
    let res = src >> 1;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, lsb == 1);

    match operand {
        Operands::AddrHL => memory.write_byte(cpu.hl(), res),
        Operands::A => cpu.a = res,
        Operands::B => cpu.b = res,
        Operands::C => cpu.c = res,
        Operands::D => cpu.d = res,
        Operands::E => cpu.e = res,
        Operands::H => cpu.h = res,
        Operands::L => cpu.l = res,
        _ => panic!(),
    };
}
