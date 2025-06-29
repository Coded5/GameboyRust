use super::mbc::MBC;

pub struct Rom {
    memory: [u8; 0x8000],
}

impl Rom {
    pub fn new(cartridge_memory: Vec<u8>) -> Self {
        let mut memory = [0u8; 0x8000];

        memory.copy_from_slice(&cartridge_memory[..0x8000]);
        Self { memory }
    }
}

impl MBC for Rom {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        // self.memory[address as usize] = value;
    }
}
