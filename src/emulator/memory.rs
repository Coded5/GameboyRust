use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Memory {
    memory: [u8; 0x10000],
    locked_byte: u8,
    pub lock_vram: bool,
    pub lock_oam: bool,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0_u8; 0x10000],
            lock_vram: false,
            lock_oam: false,
            locked_byte: 0u8,
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

    pub fn get_mut_byte(&mut self, address: u16) -> &mut u8 {
        if self.lock_oam && (0xFE00..=0xFE9F).contains(&address) {
            println!("OAM Accessed during lock!");
            return &mut self.locked_byte;
        } else if self.lock_vram && (0x8000..=0x9FFF).contains(&address) {
            println!("VARM Accessed during lock!");
            return &mut self.locked_byte;
        }

        &mut self.memory[address as usize]
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

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
