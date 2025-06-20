use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};

use log::warn;

use super::mbcs::mbc::MBC;
use super::timer::DIV;

pub struct Memory {
    mbc: Box<dyn MBC>,

    memory: [u8; 0x10000],
    locked_byte: u8,

    pub lock_vram: bool,
    pub lock_oam: bool,
}

impl Memory {
    pub fn new(mbc: Box<dyn MBC>) -> Memory {
        Memory {
            memory: [0_u8; 0x10000],
            lock_vram: false,
            lock_oam: false,
            locked_byte: 0u8,

            mbc,
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<usize, io::Error> {
        let mut file = File::open(path)?;
        let length: usize = fs::metadata(path)?.len() as usize;
        let mut buffer: Vec<u8> = vec![0u8; length];
        file.read_exact(&mut buffer)?;

        for (addr, &byte) in buffer.iter().enumerate() {
            if addr >= self.memory.len() {
                panic!("Rom is too big");
            }

            self.memory[addr] = byte;
        }

        Ok(length)
    }

    // pub fn get_mut_byte(&mut self, address: u16) -> Option<&mut u8> {
    //     Some(&mut self.memory[address as usize])
    // }

    pub fn get_mut_bytee(&mut self, address: u16) -> &mut u8 {
        if self.lock_oam && (0xFE00..=0xFE9F).contains(&address) {
            warn!("OAM Accessed during lock");
            return &mut self.locked_byte;
        }

        if self.lock_vram && (0x8000..=0x9FFF).contains(&address) {
            warn!("VRAM Accessed during lock");
            return &mut self.locked_byte;
        }

        if address == DIV {
            self.memory[DIV as usize] = 0u8;
            return &mut self.locked_byte;
        }

        if address <= 0x7FFF {
            warn!("Attempted to write to ROM");
            return &mut self.locked_byte;
        }

        &mut self.memory[address as usize]
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.mbc.read_byte(address),
            0x4000..=0x7FFF => self.mbc.read_byte(address),
            0xA000..=0xBFFF => self.mbc.read_byte(address),

            //TODO: Seggregate memory map
            _ => self.memory[address as usize],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address < 8000 {
            self.mbc.handle_banking(address, value);
            return;
        }

        if (0xA000..=0xBFFF).contains(&address) {
            self.mbc.write_byte(address, value);
            return;
        }

        if self.lock_oam && (0xFE00..=0xFE9F).contains(&address) {
            warn!("OAM Accessed during lock");
            // return;
        }

        if self.lock_vram && (0x8000..=0x9FFF).contains(&address) {
            warn!("VRAM Accessed during lock");
            // return;
        }

        if address == DIV {
            self.memory[DIV as usize] = 0u8;
            return;
        }

        if address <= 0x7FFF {
            warn!("Attempted to write to ROM");
            return;
        }

        self.memory[address as usize] = value;
    }

    // pub fn get_byte(&self, address: u16) -> u8 {
    //     self.memory[address as usize]
    // }

    pub fn dump_memory_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.memory)?;
        Ok(())
    }

    pub fn dump_vram_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.memory[0x8000..=0x97FF])?;
        Ok(())
    }
}
