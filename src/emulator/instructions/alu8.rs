use crate::emulator::{
    cpu::{Cpu, C, H, N, Z},
    memory::Memory,
};

use super::operand::{self, Operands};

pub fn ccf(cpu: &mut Cpu) {
    cpu.set(H, false);
    cpu.set(N, false);
    cpu.set(C, !cpu.c());
}

pub fn scf(cpu: &mut Cpu) {
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, true);
}

pub fn add(cpu: &mut Cpu, memory: &mut Memory, _operand1: Operands, operand2: Operands) {
    let rhs: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let (res, carry) = cpu.a.overflowing_add(rhs);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, ((cpu.a & 0xF) + (rhs & 0xF)) > 0xF);
    cpu.set(C, carry);

    cpu.a = res;
}

//ZNHC
//00100000
//00110000
pub fn adc(cpu: &mut Cpu, memory: &mut Memory, _operand1: Operands, operand2: Operands) {
    let rhs: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let c: u8 = if cpu.c() { 1 } else { 0 };
    let (res, carry_0) = cpu.a.overflowing_add(rhs);
    let (res, carry_1) = res.overflowing_add(c);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, ((cpu.a & 0xF) + (rhs & 0xF) + c) > 0xF);
    cpu.set(C, carry_0 || carry_1);

    cpu.a = res;
}

//ZNHC
//01110000
//01010000
pub fn sub(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let (res, borrow) = cpu.a.overflowing_sub(rhs);

    cpu.set(Z, res == 0);
    cpu.set(N, true);
    cpu.set(H, (cpu.a & 0xF) < (rhs & 0xF));
    cpu.set(C, borrow);

    cpu.a = res;
}

//01110000
//01000000
pub fn sbc(cpu: &mut Cpu, memory: &mut Memory, _operand0: Operands, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let c = if cpu.c() { 1 } else { 0 };
    let (res, borrow0) = cpu.a.overflowing_sub(rhs);
    let (res, borrow1) = res.overflowing_sub(c);

    cpu.set(Z, res == 0);
    cpu.set(N, true);
    cpu.set(H, (cpu.a & 0xF) < (rhs & 0xF) + c);
    cpu.set(C, borrow0 || borrow1);

    cpu.a = res;
}

pub fn and(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let res = cpu.a & rhs;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, true);
    cpu.set(C, false);

    cpu.a = res;
}

pub fn or(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let res = cpu.a | rhs;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, false);

    cpu.a = res;
}

pub fn xor(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    let res = cpu.a ^ rhs;

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, false);
    cpu.set(C, false);

    cpu.a = res;
}

pub fn cp(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!(),
    };

    cpu.set(Z, cpu.a == rhs);
    cpu.set(N, true);
    cpu.set(H, (cpu.a & 0xF) < (rhs & 0xF));
    cpu.set(C, cpu.a < rhs);
}

pub fn inc(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
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

    let (res, _carry) = rhs.overflowing_add(1);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, ((1 & 0xF) + (rhs & 0xF)) > 0xF);

    let lhs: &mut u8 = match operand {
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

    *lhs = res;
}

pub fn dec(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
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

    let (res, _carry) = rhs.overflowing_sub(1);

    cpu.set(Z, res == 0);
    cpu.set(N, true);
    cpu.set(H, (rhs & 0xF) < (1 & 0xF));

    let lhs: &mut u8 = match operand {
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

    *lhs = res;
}
