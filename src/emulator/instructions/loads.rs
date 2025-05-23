use super::operand::Operands;
use crate::emulator::{
    cpu::{Cpu, C, H, N, Z},
    memory::Memory,
};

#[allow(dead_code, unused)]
pub fn load8(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {
    let src: u8 = match operand2 {
        Operands::A => cpu.a,
        Operands::F => cpu.f,
        Operands::B => cpu.b,
        Operands::C => cpu.c,
        Operands::D => cpu.d,
        Operands::E => cpu.e,
        Operands::H => cpu.h,
        Operands::L => cpu.l,
        Operands::AddrBC => memory.get_byte(cpu.bc()),
        Operands::AddrHL => memory.get_byte(cpu.hl()),
        Operands::AddrDE => memory.get_byte(cpu.de()),
        Operands::AddrHLI => {
            cpu.set_hl(cpu.hl() + 1);
            memory.get_byte(cpu.hl() - 1)
        }
        Operands::AddrHLD => {
            cpu.set_hl(cpu.hl() - 1);
            memory.get_byte(cpu.hl() + 1)
        }
        Operands::AddrFF00_C => memory.get_byte(0xFF00 + (cpu.c as u16)),
        Operands::AddrFF00_U8 => {
            let addr: u16 = 0xFF00 + cpu.next_byte(memory) as u16;
            memory.get_byte(addr)
        }
        Operands::AddrU16 => {
            let addr: u16 = cpu.next_short(memory);
            memory.get_byte(addr)
        }
        Operands::U8 => cpu.next_byte(memory),
        _ => panic!("Invalid source : {:?}", operand2),
    };

    let des: &mut u8 = match operand1 {
        Operands::A => &mut cpu.a,
        Operands::F => &mut cpu.f,
        Operands::B => &mut cpu.b,
        Operands::C => &mut cpu.c,
        Operands::D => &mut cpu.d,
        Operands::E => &mut cpu.e,
        Operands::H => &mut cpu.h,
        Operands::L => &mut cpu.l,
        Operands::AddrBC => memory.get_mut_byte(cpu.bc()),
        Operands::AddrHL => memory.get_mut_byte(cpu.hl()),
        Operands::AddrHLI => {
            cpu.set_hl(cpu.hl() + 1);
            memory.get_mut_byte(cpu.hl() - 1)
        }
        Operands::AddrHLD => {
            cpu.set_hl(cpu.hl() - 1);
            memory.get_mut_byte(cpu.hl() + 1)
        }
        Operands::AddrDE => memory.get_mut_byte(cpu.de()),
        Operands::AddrFF00_C => memory.get_mut_byte(0xFF00 + cpu.c as u16),
        Operands::AddrFF00_U8 => {
            let addr: u16 = 0xFF00 + cpu.next_byte(memory) as u16;
            memory.get_mut_byte(addr)
        }
        Operands::AddrU16 => {
            let addr: u16 = cpu.next_short(memory);
            memory.get_mut_byte(addr)
        }
        _ => panic!(),
    };

    *des = src;
}

pub fn load16(cpu: &mut Cpu, memory: &mut Memory, operand1: Operands, operand2: Operands) {
    let src: u16 = match operand2 {
        Operands::U16 => cpu.next_short(memory),
        Operands::HL => cpu.hl(),
        Operands::SP_i8 => {
            let rhs_raw = cpu.next_byte(memory);
            let rhs = rhs_raw as i8;

            let (res, carry) = cpu.sp.overflowing_add_signed(rhs as i16);

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
            let address = cpu.next_short(memory);
            let lo: u8 = (src & 0xFF) as u8;
            let hi: u8 = ((src >> 8) & 0xFF) as u8;

            println!("sp: {}, hi: {}, lo: {}", cpu.sp, hi, lo);

            *memory.get_mut_byte(address) = lo;
            *memory.get_mut_byte(address + 1) = hi;
        }
        _ => panic!(),
    }
}
