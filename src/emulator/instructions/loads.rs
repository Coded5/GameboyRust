use crate::emulator::{cpu::{Cpu, Z, N, H, C}, memory::Memory};
use super::operand::Operands;

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
        Operands::AddrBC  => memory.get_byte(cpu.bc()),
        Operands::AddrHL  => memory.get_byte(cpu.hl()),
        Operands::AddrDE  => memory.get_byte(cpu.de()),
        Operands::AddrHLI => {
            cpu.set_bc(cpu.bc() + 1);
            memory.get_byte(cpu.bc() - 1)
        },
        Operands::AddrHLD => {
            cpu.set_bc(cpu.bc() - 1);
            memory.get_byte(cpu.bc() + 1)
        },
        Operands::AddrFF00_C  => memory.get_byte(cpu.c as u16),
        Operands::AddrFF00_U8 => {
            let addr: u16 = 0xFF00 + cpu.next_byte(memory) as u16;
            memory.get_byte(addr)
        },
        Operands::AddrU16 => {
            let addr: u16 = cpu.next_short(memory);
            memory.get_byte(addr)
        },
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
        },
        Operands::AddrHLD => {
            cpu.set_hl(cpu.hl() - 1);
            memory.get_mut_byte(cpu.hl() + 1)
        },
        Operands::AddrDE => memory.get_mut_byte(cpu.de()),
        Operands::AddrFF00_C => memory.get_mut_byte(0xFF00 + cpu.c as u16),
        Operands::AddrFF00_U8 => {
            let addr: u16 = 0xFF00 + cpu.next_byte(memory) as u16;
            memory.get_mut_byte(addr)
        },
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
        Operands::HL  => cpu.hl(),
        Operands::SP_i8 => {
            let offset: u8 = cpu.next_byte(memory);
            let (val, carry) = cpu.sp.overflowing_add_signed(offset as i16);

            //HACK: this is so bad :(
            let half_carry: bool = if ((offset as i8).is_positive()) {
                ((cpu.sp & 0xF) + (offset as u16 & 0xF)) > 0xF
            }
            else {
                (cpu.sp & 0xF) < (offset as u16 & 0xF)
            };

            cpu.set(Z, false);
            cpu.set(N, false);
            cpu.set(H, half_carry);
            cpu.set(C, carry);

            val
        },
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
            let lo: u8 = (src & 0xF) as u8;
            let hi: u8 = ((src >> 8) & 0xF) as u8;

            *memory.get_mut_byte(address) = lo;
            *memory.get_mut_byte(address + 1) = hi;
        }
        _ => panic!(),
    }
}
