use crate::emulator::{
    cpu::{Cpu, C, H, N, Z},
    memory::Memory,
};

use super::operand::Operands;

pub fn daa(cpu: &mut Cpu) {
    let mut a = cpu.a;
    let mut adjust = 0;
    let mut carry = false;

    if cpu.n() {
        if cpu.h() {
            adjust |= 0x06;
        }
        if cpu.c() {
            adjust |= 0x60;
        }
        a = a.wrapping_sub(adjust);
    } else {
        if cpu.h() || (a & 0x0F) > 0x09 {
            adjust |= 0x06;
        }
        if cpu.c() || a > 0x99 {
            adjust |= 0x60;
            carry = true;
        }
        a = a.wrapping_add(adjust);
    }

    cpu.set(Z, a == 0);
    cpu.set(H, false); // Always cleared
    if !cpu.n() {
        cpu.set(C, carry); // Only set during addition
    }

    cpu.a = a;
}

pub fn cpl(cpu: &mut Cpu) {
    cpu.a = !cpu.a;
    cpu.set(N, true);
    cpu.set(H, true);
}

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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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

pub fn adc(cpu: &mut Cpu, memory: &mut Memory, _operand1: Operands, operand2: Operands) {
    let rhs: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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

pub fn sub(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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

pub fn sbc(cpu: &mut Cpu, memory: &mut Memory, _operand0: Operands, operand: Operands) {
    let rhs: u8 = match operand {
        Operands::A => cpu.a,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
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
        Operands::AddrHL => memory.read_byte(cpu.hl()),
        _ => panic!(),
    };

    let (res, _carry) = rhs.overflowing_add(1);

    cpu.set(Z, res == 0);
    cpu.set(N, false);
    cpu.set(H, ((1 & 0xF) + (rhs & 0xF)) > 0xF);

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

pub fn dec(cpu: &mut Cpu, memory: &mut Memory, operand: Operands) {
    let rhs: u8 = match operand {
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

    let (res, _carry) = rhs.overflowing_sub(1);

    cpu.set(Z, res == 0);
    cpu.set(N, true);
    cpu.set(H, (rhs & 0xF) < (1 & 0xF));

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
