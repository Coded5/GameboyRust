use std::{fs::{self, File}, io::{self, Read}};

use crate::emulator::{cpu::Cpu, instructions::opcode_table::{get_opcode, get_prefixed_opcode}};

pub fn load_rom_as_buffer(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let length: usize = fs::metadata(path)?.len() as usize;
    let mut buffer: Vec<u8> = vec![0u8; length];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn disassemble_rom(path: &str) -> io::Result<Vec<String>> {
    let mut disassembled_rom: Vec<String> = Vec::new();
    let rom: Vec<u8> = load_rom_as_buffer(path)?;
    let rom_size = rom.len() as u16;
    let mut current_address: u16 = 0;

    while (current_address < rom_size) {
        let opcode_byte = rom[current_address as usize];

        if (opcode_byte == 0xCB) {
            current_address += 1;
            let cb_opcode_byte = rom[current_address as usize];

            let opcode = get_prefixed_opcode(cb_opcode_byte);

            let inst = Cpu::disassemble_opcode(opcode, vec![]);            
            disassembled_rom.push(inst);
            current_address += 1;
            continue;
        }

        let opcode = match get_opcode(opcode_byte) {
            Ok(o) => o,
            _ => {
                current_address += 1;
                continue;
            }
        };

        let mut opcode_data = vec![0u8; opcode.length - 1];

        (0..opcode.length-1).for_each(|i| {
            current_address += 1;
            opcode_data[i] = rom[current_address as usize];
        });

        let inst = Cpu::disassemble_opcode(opcode, opcode_data);
        disassembled_rom.push(inst);
        current_address += 1;
    }

    Ok(disassembled_rom)
}
