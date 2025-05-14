use super::memory::Memory;

#[allow(non_camel_case_types)]
pub enum PpuMode {
    OAM_SCAN, //Mode 2
    DRAW,     //Mode 3
    HBLANK,   //Mode 0
    VBLANK,   //Mode 1
}

pub struct Ppu {
    screen: [u8; 160 * 144],
    current_dot: u32,
    mode: PpuMode,
}

impl Ppu {
    fn tick(&mut self, memory: &mut Memory) {}

    pub fn update(&mut self, cycles: i32, memory: &mut Memory) {
        for _ in 0..cycles {
            self.tick(memory);
        }
    }
}

impl Default for Ppu {
    fn default() -> Ppu {
        Ppu {
            screen: [0; 160 * 144],
            current_dot: 0,
            mode: PpuMode::OAM_SCAN,
        }
    }
}
