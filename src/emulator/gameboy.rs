use super::{cpu::Cpu, memory::Memory};

#[derive(Debug)]
pub struct Gameboy {
        
    pub cpu: Cpu,
    pub memory: Memory,

}

impl Gameboy {
    
    pub fn new() -> Self {
        Gameboy {
            cpu: Cpu::new(),
            memory: Memory::new()
        }
    }

}

impl Default for Gameboy {
    fn default() -> Self {
        Self::new()
    }
}
