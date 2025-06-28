use super::operand::Operands;
use crate::emulator::{
    bus::Bus,
    cpu::{Cpu, C, H, N, Z},
};

#[allow(dead_code, unused)]
pub fn load8(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, operand2: Operands) {
    let src: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::F => cpu.f,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrBC => bus.read_byte(cpu.bc()),
        Operands::AddrHL => bus.read_byte(cpu.hl()),
        Operands::AddrDE => bus.read_byte(cpu.de()),
        Operands::AddrHLI => {
            cpu.set_hl(cpu.hl() + 1);
            bus.read_byte(cpu.hl() - 1)
        }
        Operands::AddrHLD => {
            cpu.set_hl(cpu.hl() - 1);
            bus.read_byte(cpu.hl() + 1)
        }
        Operands::AddrFF00_C => bus.read_byte(0xFF00 + (cpu.c as u16)),
        Operands::AddrFF00_U8 => {
            let addr: u16 = 0xFF00 + cpu.next_byte(bus) as u16;
            bus.read_byte(addr)
        }
        Operands::AddrU16 => {
            let addr: u16 = cpu.next_short(bus);
            bus.read_byte(addr)
        }
        Operands::U8 => cpu.next_byte(bus),
        _ => panic!("Invalid source : {:?}", operand2),
    };

    match operand1 {
        Operands::A => cpu.a = src,
        Operands::F => cpu.f = src,
        Operands::B => cpu.b = src,
        Operands::C => cpu.c = src,
        Operands::D => cpu.d = src,
        Operands::E => cpu.e = src,
        Operands::H => cpu.h = src,
        Operands::L => cpu.l = src,
        Operands::AddrBC => bus.write_byte(cpu.bc(), src),
        Operands::AddrHL => bus.write_byte(cpu.hl(), src),
        Operands::AddrHLI => {
            cpu.set_hl(cpu.hl().wrapping_add(1));
            bus.write_byte(cpu.hl() - 1, src);
        }
        Operands::AddrHLD => {
            cpu.set_hl(cpu.hl().wrapping_sub(1));
            bus.write_byte(cpu.hl() + 1, src);
        }
        Operands::AddrDE => bus.write_byte(cpu.de(), src),
        Operands::AddrFF00_C => bus.write_byte(0xFF00 + cpu.c as u16, src),
        Operands::AddrFF00_U8 => {
            let addr: u16 = 0xFF00 + cpu.next_byte(bus) as u16;
            bus.write_byte(addr, src);
        }
        Operands::AddrU16 => {
            let addr: u16 = cpu.next_short(bus);
            bus.write_byte(addr, src);
        }
        _ => panic!(),
    };
}

pub fn load16(cpu: &mut Cpu, bus: &mut Bus, operand1: Operands, operand2: Operands) {
    let src: u16 = match operand2 {
        Operands::U16 => cpu.next_short(bus),
        Operands::HL => cpu.hl(),
        Operands::SP_i8 => {
            let rhs_raw = cpu.next_byte(bus);
            let rhs = rhs_raw as i8;

            let (res, _carry) = cpu.sp.overflowing_add_signed(rhs as i16);

            let sp_lo = cpu.sp as u8;

            cpu.set(Z, false);
            cpu.set(N, false);
            cpu.set(H, ((sp_lo & 0xF) + (rhs_raw & 0xF)) > 0x0F);
            cpu.set(C, ((sp_lo as u16) + (rhs_raw as u16)) > 0xFF);

            res
        }
        Operands::SP => cpu.sp,
        _ => panic!("Invalid source : {:?}", operand2),
    };

    match operand1 {
        Operands::BC => cpu.set_bc(src),
        Operands::DE => cpu.set_de(src),
        Operands::HL => cpu.set_hl(src),
        Operands::SP => cpu.sp = src,
        Operands::AddrU16 => {
            let address = cpu.next_short(bus);
            let lo: u8 = (src & 0xFF) as u8;
            let hi: u8 = ((src >> 8) & 0xFF) as u8;

            bus.write_byte(address, lo);
            bus.write_byte(address + 1, hi);
        }
        _ => panic!(),
    }
}
