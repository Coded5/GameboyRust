use log::{info, warn};

use super::mbc::MBC;

pub struct MBC1 {
    rom_bank: u8,
    ram_bank: u8,
    rom_banking: bool,

    rom: Vec<u8>,
    ram: [u8; 0x8000],

    ram_enable: bool,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, _ram_size: u32) -> Self {
        Self {
            rom,
            ram: [0u8; 0x8000],
            ram_bank: 0,
            rom_bank: 1,
            ram_enable: false,
            rom_banking: false,
        }
    }

    pub fn change_mode(&mut self, value: u8) {
        self.rom_banking = (value & 1) == 0;

        if self.rom_banking {
            self.ram_bank = 0;
        }
    }

    pub fn change_ram_bank(&mut self, value: u8) {
        self.ram_bank = value & 3;

        info!(target: "MBC", "Change ram bank to {:02X}", self.ram_bank);
    }

    pub fn change_rom_bank_hi(&mut self, value: u8) {
        self.rom_bank &= 0x1F;
        self.rom_bank |= value & 0xE0;

        if self.rom_bank == 0 {
            self.rom_bank = 1;
        }

        info!(target: "MBC", "Changing to rom bank {:02X}", self.rom_bank);
    }

    pub fn change_rom_bank_lo(&mut self, value: u8) {
        let masked_byte = value & 0x1f;

        self.rom_bank &= 0xE0;
        self.rom_bank |= masked_byte;
        if self.rom_bank == 0 {
            self.rom_bank = 1;
        }

        info!(target: "MBC", "Changing to rom bank {:02X}", self.rom_bank);
    }

    pub fn enable_ram(&mut self, value: u8) {
        self.ram_enable = (value & 0x0F) == 0x0A;
    }
}

impl MBC for MBC1 {
    fn handle_banking(&mut self, address: u16, value: u8) {
        match address {
            0x0000..0x2000 => self.enable_ram(value),
            0x2000..0x4000 => self.change_rom_bank_lo(value),
            0x4000..0x6000 => {
                if self.rom_banking {
                    self.change_rom_bank_hi(value);
                } else {
                    self.change_ram_bank(value);
                }
            }
            0x6000..0x8000 => self.change_mode(value),
            _ => (),
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        // debug!(target: "Memory", "Reading MBC at {:04X}", address);

        if (0x4000..=0x7FFF).contains(&address) {
            let local_address = (address - 0x4000) as usize;
            return self.rom[local_address + (self.rom_bank as usize * 0x4000)];
        } else if (0xA000..=0xBFFF).contains(&address) {
            if !self.ram_enable {
                return 0xFF;
            }

            let local_address = (address - 0xA000) as usize;
            return self.ram[local_address + (self.ram_bank as u16 * 0x2000) as usize];
        }

        self.rom[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if !(0xA000..=0xBFFF).contains(&address) {
            self.handle_banking(address, value);
            return;
        }

        let local_address = (address - 0xA000) as usize;
        self.ram[local_address + (self.ram_bank as u16 * 0x2000) as usize] = value;
    }
}
