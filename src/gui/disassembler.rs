use crate::emulator::{
    cpu::Cpu,
    instructions::opcode_table::{get_opcode, get_prefixed_opcode},
    memory::Memory,
};

pub fn disassemble_rom(memory: &Memory) -> Vec<(u16, String)> {
    let mut disassembled_rom: Vec<(u16, String)> = Vec::new();
    let mut current_address: u16 = 0;

    while (current_address < 0x4000) {
        let initial_address = current_address;
        let opcode_byte = memory.read_byte(current_address);

        if (opcode_byte == 0xCB) {
            current_address += 1;
            let cb_opcode_byte = memory.read_byte(current_address);

            let opcode = get_prefixed_opcode(cb_opcode_byte);

            let inst = Cpu::disassemble_opcode(&opcode, vec![]);
            disassembled_rom.push((initial_address, inst));
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

        (0..opcode.length - 1).for_each(|i| {
            current_address += 1;
            opcode_data[i] = memory.read_byte(current_address);
        });

        let inst = Cpu::disassemble_opcode(&opcode, opcode_data);
        disassembled_rom.push((initial_address, inst));
        current_address += 1;
    }

    disassembled_rom
}
