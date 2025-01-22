use super::{instructions::{opcode::{self, Opcode}, opcode_table::{get_opcode, get_prefixed_opcode}}, memory::Memory};

pub const Z: u8 = 7;
pub const N: u8 = 6;
pub const H: u8 = 5;
pub const C: u8 = 4;

#[allow(dead_code)]
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
}

#[allow(dead_code)]
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
        }
    }

    pub fn next_byte(&mut self, memory: &mut Memory) -> u8 {
        self.pc += 1;
        memory.get_byte(self.pc - 1)
    }

    pub fn next_short(&mut self, memory: &mut Memory) -> u16 {
        let hi = self.next_byte(memory) as u16;
        let lo = self.next_byte(memory) as u16;

        (hi << 8) | lo
    }

    pub fn run_rom(&self, memory: &mut Memory, rom_size: usize) {
        let mut current_address: u16 = 0;

        while (current_address < rom_size as u16) {
            print!("{:04x} ", current_address);
            let mut all_bytes: Vec<u8> = Vec::new();

            let opcode_byte: u8 = memory.get_byte(current_address);

            //TODO: Temporary
            all_bytes.push(opcode_byte);

            let opcode = if (opcode_byte == 0xCB) {
                current_address += 1;
                let cb_opcode_byte: u8 = memory.get_byte(current_address);

                all_bytes.push(cb_opcode_byte);
                get_prefixed_opcode(cb_opcode_byte)
            } else {
                match get_opcode(opcode_byte) {
                    Ok(opcode) => opcode,
                    _ => {
                        current_address += 1;
                        continue;
                    }
                }
            };

            let mut opcode_data: Vec<u8> = vec![0u8; opcode.length - 1];

            (0..opcode.length - 1).for_each(|i| {
                current_address += 1;
                opcode_data[i] = memory.get_byte(current_address);

                //TODO: Temporary
                all_bytes.push(opcode_data[i]);
            });

            print!("{:02X?} ", all_bytes);
            Self::disassemble_opcode(opcode, opcode_data);

            current_address += 1;

        }
    }

    fn disassemble_opcode(opcode: Opcode, data: Vec<u8>) {
        let mut line = String::new();
        line.push_str(&opcode.mnemonic);

        if (data.len() != opcode.length - 1) {
            panic!("Invalid opcode (Unequal length) [{}, {}]", opcode.length, data.len());
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

        println!("{}", line);
    }

    //TODO: implement this
    fn execute(&mut self, opcode: Opcode, memory: &mut Memory) -> i32 {
        unimplemented!();
    }

    pub fn z(&self) -> bool { ((self.f >> 7) & 1) == 1 }
    pub fn n(&self) -> bool { ((self.f >> 6) & 1) == 1 }
    pub fn h(&self) -> bool { ((self.f >> 5) & 1) == 1 }
    pub fn c(&self) -> bool { ((self.f >> 4) & 1) == 1 }

    pub fn set(&mut self, flag: u8, value: bool) {
        let bit: u8 = if value { 1 } else { 0 };

        let mask: u8 = !(1 << flag);
        self.f &= mask;
        self.f |= (bit << flag);
    }

    pub fn af(&self) -> u16 { (self.a as u16) << 8 | (self.f as u16) }
    pub fn bc(&self) -> u16 { (self.b as u16) << 8 | (self.c as u16) }
    pub fn de(&self) -> u16 { (self.d as u16) << 8 | (self.e as u16) }
    pub fn hl(&self) -> u16 { (self.h as u16) << 8 | (self.l as u16) }

    pub fn set_af(&mut self, val: u16) { self.a = ((val >> 8) & 0xff) as u8; self.f = (val & 0xff) as u8; }
    pub fn set_bc(&mut self, val: u16) { self.b = ((val >> 8) & 0xff) as u8; self.c = (val & 0xff) as u8; }
    pub fn set_de(&mut self, val: u16) { self.d = ((val >> 8) & 0xff) as u8; self.e = (val & 0xff) as u8; }
    pub fn set_hl(&mut self, val: u16) { self.e = ((val >> 8) & 0xff) as u8; self.l = (val & 0xff) as u8; }

}
