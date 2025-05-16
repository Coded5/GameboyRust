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
            memory: Memory::new(),
        }
    }

    pub fn tick(&mut self) {
        let max_cycle = 69905;
        let mut current_cycle = 0;

        while (current_cycle < max_cycle) {
            let cycles = self.cpu.run(&mut self.memory);
            current_cycle += cycles;

            //update_timer(cycles);
            //update_ppu(cycles);
            //perform_interrupt();
            //render();
        }
    }
}

impl Default for Gameboy {
    fn default() -> Self {
        Self::new()
    }
}
