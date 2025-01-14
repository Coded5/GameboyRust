use super::{instructions::{opcode::{self, Opcode}, opcode_table::{get_opcode, get_prefixed_opcode}}, memory::Memory};

#[allow(dead_code)]
pub struct Cpu {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16
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

}
