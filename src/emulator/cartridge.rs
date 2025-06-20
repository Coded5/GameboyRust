use std::{
    fmt::Debug,
    fs::{self, File},
    io::{self, Read},
};

use super::{
    mbcs::{mbc::MBC, mbc1::MBC1},
    memory::Memory,
};

pub const CART_HEADER_TYPE: u16 = 0x147;
pub const CART_HEADER_RAM_SIZE: u16 = 0x148;

// #[derive(Debug)]
// pub enum CartridgeType {
//     ROM,
//     MBC1,
//     MBC2,
// }

// #[derive(Debug)]
// pub struct Cartridge<'a> {
//     cartridge_type: CartridgeType,
//     pub cartridge_memory: Vec<u8>,
//
//     selected_rom_bank: u8,
//     selected_ram_bank: u8,
//
//     ram_enable: bool,
//
//     mbc: &'a mut dyn MBC,
//
//     cartridge_ram: [u8; 0x8000],
// }
//
// impl<'a> Cartridge<'a> {
//     pub fn from_file(path: &str) -> io::Result<Self> {
//         let mut file = File::open(path)?;
//         let length: usize = fs::metadata(path)?.len() as usize;
//         let mut cartridge_memory: Vec<u8> = vec![0u8; length];
//         file.read_exact(&mut cartridge_memory);
//
//         let cartridge_type = match cartridge_memory[CART_HEADER_TYPE as usize] {
//             1 => CartridgeType::MBC1,
//             2 => CartridgeType::MBC1,
//             3 => CartridgeType::MBC1,
//             4 => CartridgeType::MBC2,
//             5 => CartridgeType::MBC2,
//             6 => CartridgeType::MBC2,
//             _ => panic!("Unsupported cartridge type."),
//         };
//
//         let rom_size = (32 * 1024) * (1 << cartridge_memory[0x148]);
//
//         Ok(Cartridge {
//             cartridge_type,
//             cartridge_memory,
//             selected_rom_bank: 1,
//             selected_ram_bank: 0,
//             cartridge_ram: [0u8; 0x8000],
//             ram_enable: false,
//         })
//     }
//
//     pub fn enable_ram_bank(&mut self, address: u16, data: u8) {
//         if matches!(self.cartridge_type, CartridgeType::MBC2) && (address >> 4) & 1 == 1 {
//             return;
//         }
//
//         if data & 0x0F == 0x0A {
//             self.ram_enable = true;
//         } else if data & 0x0F == 0x00 {
//             self.ram_enable = false;
//         }
//     }
//
//     pub fn read_rom_bank(&self, address: u16) -> u8 {
//         let local_addr = (address - 0x4000) as usize;
//         self.cartridge_memory[local_addr + (self.selected_rom_bank * 0x2000) as usize]
//     }
//
//     pub fn read_ram_bank(&self, address: u16) -> u8 {
//         let local_addr = (address - 0xA000) as usize;
//         self.cartridge_ram[local_addr + (self.selected_ram_bank * 0x2000) as usize]
//     }
// }

pub fn load_cartridge(path: &str) -> io::Result<Box<dyn MBC>> {
    let mut file = File::open(path)?;
    let length: usize = fs::metadata(path)?.len() as usize;
    let mut cartridge_memory: Vec<u8> = vec![0u8; length];
    file.read_exact(&mut cartridge_memory);

    let rom_size = (32 * 1024) * (1 << cartridge_memory[0x148]);
    let mbc = match cartridge_memory[CART_HEADER_TYPE as usize] {
        1..=3 => MBC1::new(cartridge_memory, rom_size),
        4..=6 => !unimplemented!(),

        _ => panic!("Unsupported cartridge type."),
    };

    Ok(Box::new(mbc))
}
