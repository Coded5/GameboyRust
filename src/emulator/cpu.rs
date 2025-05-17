use super::{
    instructions::{
        opcode::Opcode,
        opcode_table::{execute_opcode, get_opcode, get_prefixed_opcode},
        stack,
    },
    memory::{self, Memory},
};

pub const Z: u8 = 7;
pub const N: u8 = 6;
pub const H: u8 = 5;
pub const C: u8 = 4;

pub const INT_JOYPAD: u8 = 4;
pub const INT_SERIAL: u8 = 3;
pub const INT_TIMER: u8 = 2;
pub const INT_LCD: u8 = 1;
pub const INT_VBLANK: u8 = 0;

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
    pub ime: bool,
    pub ei: u8,
    pub intf: u8,
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

            ime: false,
            ei: 0u8,
            intf: 0u8,
        }
    }

    pub fn is_interrupt_enabled(&self, interrupt: u8) -> bool {
        (self.ei >> interrupt) & 1 == 1
    }

    pub fn is_interrupt_requested(&self, interrupt: u8) -> bool {
        (self.intf >> interrupt) & 1 == 1
    }

    pub fn next_byte(&mut self, memory: &mut Memory) -> u8 {
        self.pc += 1;
        memory.get_byte(self.pc)
    }

    pub fn next_short(&mut self, memory: &mut Memory) -> u16 {
        let lo = self.next_byte(memory) as u16;
        let hi = self.next_byte(memory) as u16;

        (hi << 8) | lo
    }

    pub fn disassemble_opcode(opcode: Opcode, data: Vec<u8>) -> String {
        let mut line = String::new();
        line.push_str(&opcode.mnemonic);

        if (data.len() != opcode.length - 1) {
            panic!(
                "Invalid opcode (Unequal length) [{}, {}]",
                opcode.length,
                data.len()
            );
        }

        let mut byte: u8 = 0;
        let mut short: u16 = 0;

        if (!data.is_empty()) {
            byte = data[0];
        }

        if (data.len() == 2) {
            short = ((data[1] as u16) << 8) | (data[0] as u16);
        }

        if let Some(op1) = opcode.operand1 {
            line.push(' ');
            line.push_str(op1.get_str_format(byte, short).as_str());
        }

        if let Some(op2) = opcode.operand2 {
            line.push_str(", ");
            line.push_str(op2.get_str_format(byte, short).as_str());
        }

        // println!("{}", line);
        line
    }

    pub fn run(&mut self, memory: &mut Memory) -> i32 {
        //Fetch
        let opcode_byte = memory.get_byte(self.pc);
        println!("Opcode {:X} at {:X}", opcode_byte, self.pc);

        //Decode
        let opcode = if (opcode_byte == 0xCB) {
            self.pc += 1;
            let cb_opcode_byte = memory.get_byte(self.pc);

            get_prefixed_opcode(cb_opcode_byte)
        } else {
            get_opcode(opcode_byte)
                .unwrap_or_else(|_| panic!("Invalid opcode reached: {:02X}", opcode_byte))
        };

        //Execute
        let time = execute_opcode(self, memory, opcode);
        self.f &= 0xF0;
        self.pc = self.pc.wrapping_add(1);

        time
    }

    pub fn perform_interrupt(&mut self, memory: &mut Memory) {
        if (!self.ime) {
            return;
        }

        //Check if any interrupts are requested
        if (self.intf & self.ei != 0) {
            return;
        }

        //Interrupt are disabled
        self.ime = false;

        //Calling interrupt
        //Push PC to stack
        let hi_byte = ((self.pc >> 8) & 0xFF) as u8;
        let lo_byte = (self.pc & 0xFF) as u8;

        self.sp -= 1;
        *memory.get_mut_byte(self.sp) = hi_byte;
        self.sp -= 1;
        *memory.get_mut_byte(self.sp) = lo_byte;

        //Get interrupt address
        let address: u16 = if (self.is_interrupt_requested(INT_VBLANK)) {
            0x40
        } else if (self.is_interrupt_requested(INT_LCD)) {
            0x48
        } else if (self.is_interrupt_requested(INT_TIMER)) {
            0x50
        } else if (self.is_interrupt_requested(INT_SERIAL)) {
            0x58
        } else if (self.is_interrupt_requested(INT_JOYPAD)) {
            0x60
        } else {
            panic!("Unknown requested interrupt!")
        };

        self.pc = address;
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
        self.f |= (bit << flag);
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
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
