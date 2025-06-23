use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};

use super::mbcs::mbc::MBC;
use super::timer::DIV;

pub struct Memory {
    mbc: Box<dyn MBC>,

    memory: [u8; 0x10000],

    pub div_reset: bool,

    pub lock_vram: bool,
    pub lock_oam: bool,
    pub dma_transfer_active: bool,
    dma_transfer_cycle: i32,
    dma_transfer: u8,
}

impl Memory {
    pub fn new(mbc: Box<dyn MBC>) -> Memory {
        Memory {
            memory: [0_u8; 0x10000],
            lock_vram: false,
            lock_oam: false,

            div_reset: false,

            dma_transfer_active: false,
            dma_transfer_cycle: 0,
            dma_transfer: 0,

            mbc,
        }
    }

    //NOTE: Reserved for loading bootrom only for now
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

    pub fn update_transfer(&mut self, cycle: i32) {
        self.dma_transfer_cycle += cycle;

        if self.dma_transfer_cycle >= 160 * 4 {
            let start_addr: u16 = (self.dma_transfer as u16) << 8;
            let end_addr: u16 = ((self.dma_transfer as u16) << 8) | 0x9F;

            for (offset, addr) in (start_addr..=end_addr).enumerate() {
                self.memory[0xFE00 + offset] = self.memory[addr as usize];
            }

            self.dma_transfer_cycle = 0;
            self.dma_transfer_active = false;
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address == 0xFF01 {
            return 0xFF;
        }

        if address < 0x100 && self.memory[0xFF50] == 0 {
            return self.memory[address as usize];
        }

        match address {
            0x0000..=0x3FFF => self.mbc.read_byte(address),
            0x4000..=0x7FFF => self.mbc.read_byte(address),
            0xA000..=0xBFFF => self.mbc.read_byte(address),

            //TODO: Seggregate memory map
            _ => self.memory[address as usize],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.mbc.handle_banking(address, value);
            return;
        }

        if address == 0xFF46 {
            self.dma_transfer_active = true;
            self.dma_transfer = value;
        }

        if (0xA000..=0xBFFF).contains(&address) {
            self.mbc.write_byte(address, value);
            return;
        }

        if (0xFE00..=0xFE9F).contains(&address) {
            // warn!("OAM Accessed during lock");
            // return;
        }

        if (0x8000..=0x9FFF).contains(&address) {
            // warn!("VRAM Accessed during lock");
            // return;
        }

        if address == DIV {
            self.memory[DIV as usize] = 0u8;
            self.div_reset = true;
            return;
        }

        self.memory[address as usize] = value;
    }

    //HACK: is there a better way?
    pub fn write_byte_uncheck(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn dump_memory_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.memory)?;
        Ok(())
    }

    pub fn dump_oam_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.memory[0xFE00..=0xFE9F])?;
        Ok(())
    }

    pub fn dump_vram_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.memory[0x8000..=0x97FF])?;
        Ok(())
    }
}
