use crate::emulator::{bus::Bus, cpu::Cpu, gameboy::Shared, interrupt::InterruptState};

use super::operand::Operands;

pub fn push(cpu: &mut Cpu, bus: &mut Bus, operand: Operands) {
    let src: u16 = match operand {
        Operands::AF => cpu.af(),
        Operands::BC => cpu.bc(),
        Operands::DE => cpu.de(),
        Operands::HL => cpu.hl(),
        _ => panic!(),
    };

    let lo: u8 = (src & 0xFF) as u8;
    let hi: u8 = ((src >> 8) & 0xFF) as u8;

    cpu.sp -= 1;
    bus.write_byte(cpu.sp, hi);
    cpu.sp -= 1;
    bus.write_byte(cpu.sp, lo);
}

pub fn pop(cpu: &mut Cpu, bus: &mut Bus, operand: Operands) {
    let lo = bus.read_byte(cpu.sp) as u16;
    cpu.sp += 1;
    let hi = bus.read_byte(cpu.sp) as u16;
    cpu.sp += 1;

    let res = (hi << 8) | lo;

    match operand {
        Operands::AF => cpu.set_af(res),
        Operands::BC => cpu.set_bc(res),
        Operands::DE => cpu.set_de(res),
        Operands::HL => cpu.set_hl(res),
        _ => panic!(),
    }
}

pub fn call(cpu: &mut Cpu, bus: &mut Bus) {
    let address = cpu.pc;
    let lo_byte: u8 = (address & 0xFF) as u8;
    let hi_byte: u8 = ((address >> 8) & 0xFF) as u8;

    cpu.sp -= 1;
    bus.write_byte(cpu.sp, hi_byte);
    cpu.sp -= 1;
    bus.write_byte(cpu.sp, lo_byte);

    let new_address = cpu.next_short(bus);

    cpu.pc = new_address;
}

pub fn call_cc(cpu: &mut Cpu, bus: &mut Bus, operand: Operands) -> bool {
    let condition = match operand {
        Operands::JR_Z => cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        Operands::U16 => true,
        _ => panic!(),
    };

    let new_address = cpu.next_short(bus);

    if !condition {
        return false;
    }

    let address = cpu.pc;
    let lo_byte: u8 = (address & 0xFF) as u8;
    let hi_byte: u8 = ((address >> 8) & 0xFF) as u8;

    cpu.sp -= 1;
    bus.write_byte(cpu.sp, hi_byte);
    cpu.sp -= 1;
    bus.write_byte(cpu.sp, lo_byte);

    cpu.pc = new_address;

    true
}

pub fn ret(cpu: &mut Cpu, bus: &mut Bus) {
    let lo = bus.read_byte(cpu.sp) as u16;
    cpu.sp += 1;
    let hi = bus.read_byte(cpu.sp) as u16;
    cpu.sp += 1;

    cpu.pc = (hi << 8) | lo;
}

pub fn reti(cpu: &mut Cpu, bus: &mut Bus, interrupt: Shared<InterruptState>) -> bool {
    interrupt.borrow_mut().ime = true;
    ret(cpu, bus);

    false
}

pub fn ret_cc(cpu: &mut Cpu, bus: &mut Bus, operand: Operands) -> bool {
    let condition = match operand {
        Operands::JR_Z => cpu.z(),
        Operands::JR_NZ => !cpu.z(),
        Operands::JR_C => cpu.c(),
        Operands::JR_NC => !cpu.c(),
        _ => true,
    };

    if !condition {
        return false;
    }

    ret(cpu, bus);
    true
}

pub fn rst(cpu: &mut Cpu, bus: &mut Bus, operand: Operands) -> bool {
    let address = match operand {
        Operands::H28 => 0x28,
        Operands::H00 => 0x00,
        Operands::H08 => 0x08,
        Operands::H20 => 0x20,
        Operands::H18 => 0x18,
        Operands::H38 => 0x38,
        Operands::H30 => 0x30,
        Operands::H10 => 0x10,
        _ => panic!(),
    };

    cpu.sp = cpu.sp.wrapping_sub(1);
    bus.write_byte(cpu.sp, ((cpu.pc >> 8) & 0xFF) as u8);
    cpu.sp = cpu.sp.wrapping_sub(1);
    bus.write_byte(cpu.sp, (cpu.pc & 0xFF) as u8);

    cpu.pc = address;
    false
}
