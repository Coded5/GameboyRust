use std::process::exit;

use log::{debug, info};

use super::{
    bus::Bus,
    gameboy::Shared,
    instructions::{
        opcode::Opcode,
        opcode_table::{execute_opcode, get_opcode, get_prefixed_opcode},
    },
    interrupt::{
        InterruptState, ADDRESS_IE, ADDRESS_IF, INT_JOYPAD, INT_LCD, INT_SERIAL, INT_TIMER,
        INT_VBLANK,
    },
};

pub const Z: u8 = 7;
pub const N: u8 = 6;
pub const H: u8 = 5;
pub const C: u8 = 4;

#[derive(Debug)]
pub struct Cpu {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub halt: bool,
    pub halt_bug: bool,
    pub i_enable_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0u8,
            f: 0u8,
            b: 0u8,
            c: 0u8,
            d: 0u8,
            e: 0u8,
            h: 0u8,
            l: 0u8,
            sp: 0u16,
            pc: 0u16,

            halt: false,
            halt_bug: false,

            // ime: false,
            i_enable_flag: false,
            //
            // interrupt_enable: 0u8,
            // interrupt_flags: 0u8,
        }
    }

    pub fn next_byte(&mut self, bus: &mut Bus) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        bus.read_byte(self.pc.wrapping_sub(1))
    }

    pub fn next_short(&mut self, bus: &mut Bus) -> u16 {
        let lo = self.next_byte(bus) as u16;
        let hi = self.next_byte(bus) as u16;

        (hi << 8) | lo
    }

    pub fn disassemble_opcode(opcode: &Opcode, data: Vec<u8>) -> String {
        let mut line = String::new();
        line.push_str(&opcode.mnemonic);

        if data.len() != opcode.length - 1 {
            panic!(
                "Invalid opcode (Unequal length) [{}, {}]",
                opcode.length,
                data.len()
            );
        }

        let mut byte: u8 = 0;
        let mut short: u16 = 0;

        if !data.is_empty() {
            byte = data[0];
        }

        if data.len() == 2 {
            short = ((data[1] as u16) << 8) | (data[0] as u16);
        }

        if let Some(op1) = &opcode.operand1 {
            line.push(' ');
            line.push_str(op1.get_str_format(byte, short).as_str());
        }

        if let Some(op2) = &opcode.operand2 {
            line.push_str(", ");
            line.push_str(op2.get_str_format(byte, short).as_str());
        }

        line
    }

    pub fn step(&mut self, bus: &mut Bus, interrupt: Shared<InterruptState>) -> i32 {
        let interrupted = self.perform_interrupt(bus, interrupt.clone());

        if self.halt {
            if interrupted {
                self.halt = false;
                return 20;
            } else {
                return 4;
            }
        }

        let cycle = self.run(bus, interrupt, !self.halt_bug);
        self.halt_bug = false;

        cycle
    }

    #[allow(dead_code)]
    fn format_flag(&self) -> String {
        let z = if self.z() { "Z" } else { "z" };
        let n = if self.n() { "N" } else { "n" };
        let c = if self.h() { "H" } else { "h" };
        let h = if self.c() { "C" } else { "c" };

        format!("{}{}{}{}", z, n, h, c)
    }

    pub fn run(
        &mut self,
        bus: &mut Bus,
        interrupt: Shared<InterruptState>,
        increment_pc: bool,
    ) -> i32 {
        if self.i_enable_flag {
            interrupt.borrow_mut().ime = true;
            self.i_enable_flag = false;
        }

        //Fetch
        let opcode_byte = self.next_byte(bus);

        //Decode
        let opcode = if opcode_byte == 0xCB {
            // let cb_opcode_byte = bus.read_byte(self.pc);
            let cb_opcode_byte = self.next_byte(bus);

            get_prefixed_opcode(cb_opcode_byte)
        } else {
            get_opcode(opcode_byte)
                .unwrap_or_else(|_| panic!("Invalid opcode reached: {:02X}", opcode_byte))
        };

        // let mut data: Vec<u8> = vec![0u8; opcode.length - 1];
        // let tpc = self.pc;
        // (0..opcode.length - 1).for_each(|i| {
        //     data[i] = bus.read_byte(tpc + i as u16);
        // });
        // debug!(
        //     target: "CPU",
        //     "{:04X} {: <20}| AF={:04X}, BC={:04X}, DE={:04X}, HL={:04X} SP={:04X}",
        //     self.pc - 1,
        //     Cpu::disassemble_opcode(&opcode, data),
        //     self.af(),
        //     self.bc(),
        //     self.de(),
        //     self.hl(),
        //     self.sp
        // );

        //Execute
        let time = execute_opcode(self, bus, interrupt, opcode.clone());
        self.f &= 0xF0;

        if !increment_pc {
            self.pc -= opcode.length as u16;
        }

        time
    }

    pub fn perform_interrupt(&mut self, bus: &mut Bus, int: Shared<InterruptState>) -> bool {
        let mut interrupt = int.borrow_mut();
        if interrupt.interrupt_enable & interrupt.interrupt_flag == 0 {
            return false;
        } else {
            self.halt = false;
        }

        if !interrupt.ime {
            return false;
        }

        //Interrupt are disabled
        interrupt.ime = false;

        //Calling interrupt
        //Push PC to stack
        let hi_byte = ((self.pc >> 8) & 0xFF) as u8;
        let lo_byte = (self.pc & 0xFF) as u8;

        self.sp -= 1;
        bus.write_byte(self.sp, hi_byte);
        self.sp -= 1;
        bus.write_byte(self.sp, lo_byte);

        //Get interrupt address
        let (address, clear_bit): (u16, u8) = if interrupt.is_requested(INT_VBLANK) {
            (0x40, INT_VBLANK)
        } else if interrupt.is_requested(INT_LCD) {
            (0x48, INT_LCD)
        } else if interrupt.is_requested(INT_TIMER) {
            (0x50, INT_TIMER)
        } else if interrupt.is_requested(INT_SERIAL) {
            (0x58, INT_SERIAL)
        } else if interrupt.is_requested(INT_JOYPAD) {
            (0x60, INT_JOYPAD)
        } else {
            panic!("Unknown requested interrupt!")
        };

        // bus.write_byte(ADDRESS_IF, bus.read_byte(ADDRESS_IF) & !(1 << clear_bit));
        interrupt.interrupt_flag &= !(1 << clear_bit);

        self.pc = address;
        true
    }

    pub fn z(&self) -> bool {
        ((self.f >> 7) & 1) == 1
    }
    pub fn n(&self) -> bool {
        ((self.f >> 6) & 1) == 1
    }
    pub fn h(&self) -> bool {
        ((self.f >> 5) & 1) == 1
    }
    pub fn c(&self) -> bool {
        ((self.f >> 4) & 1) == 1
    }

    pub fn set(&mut self, flag: u8, value: bool) {
        let bit: u8 = if value { 1 } else { 0 };

        let mask: u8 = !(1 << flag);
        self.f &= mask;
        self.f |= bit << flag;
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = ((val >> 8) & 0xff) as u8;
        self.f = (val & 0xff) as u8;
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val >> 8) & 0xff) as u8;
        self.c = (val & 0xff) as u8;
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = ((val >> 8) & 0xff) as u8;
        self.e = (val & 0xff) as u8;
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val >> 8) & 0xff) as u8;
        self.l = (val & 0xff) as u8;
    }

    pub fn halt(&mut self, interrupt: Shared<InterruptState>) {
        let int = interrupt.borrow();
        if !int.ime && int.have_pending() {
            self.halt = false;
            self.halt_bug = true;
        } else {
            self.halt = true;
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
