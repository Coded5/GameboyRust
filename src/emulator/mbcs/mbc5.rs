use super::mbc::MBC;

pub struct MBC5 {
    //NOTE: Unsigned 9-bit
    rom_bank: u16,
    ram_bank: u8,

    rom: Vec<u8>,
    ram: [u8; 0x20000],

    ram_enable: bool,
}

impl MBC5 {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            rom_bank: 1,
            ram_bank: 0,
            ram_enable: false,

            rom: data,
            ram: [0u8; 0x20000],
        }
    }

    fn handle_ram_enable(&mut self, value: u8) {
        self.ram_enable = value & 0b1111 == 0xA;
    }

    fn change_rom_bank_low(&mut self, value: u8) {
        self.rom_bank &= 0b100000000;
        self.rom_bank |= value as u16;
    }

    fn change_rom_bank_hi(&mut self, value: u8) {
        self.rom_bank &= 0b011111111;
        self.rom_bank |= (value & 1) as u16;
    }

    fn change_ram_bank(&mut self, value: u8) {
        self.ram_bank = value & 0b1111;
    }
}

impl MBC for MBC5 {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                let local_address =
                    (0x4000u32 * (self.rom_bank as u32) + ((address as u32) - 0x4000)) as usize;
                self.rom[local_address]
            }
            0xA000..=0xBFFF => {
                let local_address = (0x2000 * (self.ram_bank as u16) + (address - 0xA000)) as usize;
                self.ram[local_address]
            }
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.handle_ram_enable(value),
            0x2000..=0x2FFF => self.change_rom_bank_low(value),
            0x3000..=0x3FFF => self.change_rom_bank_hi(value),
            0x4000..=0x4FFF => self.change_ram_bank(value),
            0xA000..=0xBFFF => {
                let local_address = (0x2000 * (self.ram_bank as u16) + (address - 0xA000)) as usize;
                self.ram[local_address] = value;
            }
            _ => (),
        }
    }
}
