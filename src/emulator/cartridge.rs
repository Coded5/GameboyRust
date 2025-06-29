use std::{
    fs::{self, File},
    io::{self, Read},
};

use log::info;

use crate::emulator::mbcs::mbc5::MBC5;

use super::mbcs::{mbc::MBC, mbc1::MBC1, rom::Rom};

pub const CART_HEADER_TYPE: u16 = 0x147;
pub const CART_HEADER_RAM_SIZE: u16 = 0x148;

pub fn load_cartridge(path: &str) -> io::Result<Box<dyn MBC>> {
    let mut file = File::open(path)?;
    let length: usize = fs::metadata(path)?.len() as usize;
    let mut cartridge_memory: Vec<u8> = vec![0u8; length];
    file.read_exact(&mut cartridge_memory)?;

    info!("Load cartridge with size: {}", length);

    let rom_size = (32 * 1024) * (1 << cartridge_memory[0x148]);
    let mbc_type = cartridge_memory[CART_HEADER_TYPE as usize];
    let mbc: Box<dyn MBC> = match mbc_type {
        0 => Box::new(Rom::new(cartridge_memory)),
        1..=3 => Box::new(MBC1::new(cartridge_memory, rom_size)),
        4..=6 => unimplemented!(),
        0x19..=0x1E => Box::new(MBC5::new(cartridge_memory)),

        _ => panic!("Unsupported cartridge type: {}", mbc_type),
    };

    Ok(mbc)
}
