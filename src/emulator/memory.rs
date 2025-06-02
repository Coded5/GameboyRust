use std::fs::{self, File};
use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct Memory {
    memory: [u8; 0x10000],
    locked_byte: u8,
    lock_vram: bool,
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
        if (self.lock_vram) {
            &mut self.locked_byte
        } else {
            &mut self.memory[address as usize]
        }
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
