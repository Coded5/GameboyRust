use crate::devices::screen::Screen;

use super::{
    cpu::{Cpu, ADDRESS_IE},
    memory::Memory,
    ppu::{Ppu, LCDC, LCDC_OBJ_SIZE, LCDC_WIN_ENABLE, LCDC_WIN_TILEMAP, SCX, SCY, WX, WY},
};

#[derive(Debug)]
pub struct Gameboy {
    pub cpu: Cpu,
    pub memory: Memory,
    pub ppu: Ppu,
    pub accum_cycle: u128,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            cpu: Cpu::new(),
            memory: Memory::new(),
            ppu: Ppu::default(),
            accum_cycle: 0u128,
        }
    }

    pub fn get_frame_buffer(&self) -> &[u8; 160 * 144] {
        &self.ppu.frame_buffer
    }

    pub fn tick(&mut self) {
        let cycle = self.cpu.step(&mut self.memory);
        //TODO:
        // self.timer.update(cycle, &mut self.memory);

        self.ppu.update(cycle, &mut self.memory);
        self.cpu.perform_interrupt(&mut self.memory);

        self.accum_cycle += cycle as u128;
    }

    //HACK: Remove this
    pub fn load_test_ram(&mut self) {
        let tile_0: [u8; 16] = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ];
        let tile_1: [u8; 16] = [
            0xFF, 0x00, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97,
            0x7E, 0xFF,
        ];

        let tile_00: [u8; 16] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        let tile_spr: [u8; 16] = [
            0x00, 0xFF, 0x02, 0x81, 0x26, 0xA5, 0x26, 0xA5, 0x02, 0x81, 0x42, 0xC3, 0x3E, 0xBD,
            0x00, 0xFF,
        ];

        for i in 0..16 {
            *(self.memory.get_mut_byte(0x8000 + i as u16)) = tile_00[i];
            *(self.memory.get_mut_byte(0x8000 + (i + 16) as u16)) = tile_1[i];
            *(self.memory.get_mut_byte(0x8000 + (i + 32) as u16)) = tile_spr[i];
            *(self.memory.get_mut_byte(0x8000 + (i + 48) as u16)) = tile_00[i];
        }

        for i in (0x9C00..=0x9FFF) {
            *(self.memory.get_mut_byte(i)) = 1u8;
        }

        let flip_x = 5u8;
        let flip_y = 6u8;

        //OAM
        let y = 20u8;
        let x = 16u8;
        let tile_number = 2u8;
        let spr_flags = (1 << flip_x) | (1 << flip_y);

        *(self.memory.get_mut_byte(0xFE00)) = y;
        *(self.memory.get_mut_byte(0xFE00 + 1)) = x;
        *(self.memory.get_mut_byte(0xFE00 + 2)) = tile_number;
        *(self.memory.get_mut_byte(0xFE00 + 3)) = spr_flags;

        *(self.memory.get_mut_byte(0xFF40)) =
            0x91 | (1 << LCDC_WIN_TILEMAP) | (1 << LCDC_WIN_ENABLE) | (1 << LCDC_OBJ_SIZE);

        *(self.memory.get_mut_byte(0xFF42)) = 6;
        *(self.memory.get_mut_byte(0xFF43)) = 6;

        *(self.memory.get_mut_byte(WX)) = 20;
        *(self.memory.get_mut_byte(WY)) = 20;

        *(self.memory.get_mut_byte(0xFF47)) = 0xE4;
        *(self.memory.get_mut_byte(0xFF48)) = 0xE4;
        *(self.memory.get_mut_byte(0xFF49)) = 0xE4;
    }

    pub fn set_gb_initial_state(&mut self) {
        self.cpu.set_af(0x01B0);
        self.cpu.set_bc(0x0013);
        self.cpu.set_de(0x00D8);
        self.cpu.set_hl(0x014D);

        self.cpu.pc = 0x100;
        self.cpu.sp = 0xFFFE;

        *(self.memory.get_mut_byte(0xFF05)) = 0x00;
        *(self.memory.get_mut_byte(0xFF06)) = 0x00;
        *(self.memory.get_mut_byte(0xFF07)) = 0x00;
        *(self.memory.get_mut_byte(0xFF10)) = 0x80;
        *(self.memory.get_mut_byte(0xFF11)) = 0xBF;
        *(self.memory.get_mut_byte(0xFF12)) = 0xF3;
        *(self.memory.get_mut_byte(0xFF14)) = 0xBF;
        *(self.memory.get_mut_byte(0xFF16)) = 0x3F;
        *(self.memory.get_mut_byte(0xFF17)) = 0x00;
        *(self.memory.get_mut_byte(0xFF19)) = 0xBF;
        *(self.memory.get_mut_byte(0xFF1A)) = 0x7F;
        *(self.memory.get_mut_byte(0xFF1B)) = 0xFF;
        *(self.memory.get_mut_byte(0xFF1C)) = 0x9F;
        *(self.memory.get_mut_byte(0xFF1E)) = 0xBF;
        *(self.memory.get_mut_byte(0xFF20)) = 0xFF;
        *(self.memory.get_mut_byte(0xFF21)) = 0x00;
        *(self.memory.get_mut_byte(0xFF22)) = 0x00;
        *(self.memory.get_mut_byte(0xFF23)) = 0xBF;
        *(self.memory.get_mut_byte(0xFF24)) = 0x77;
        *(self.memory.get_mut_byte(0xFF25)) = 0xF3;
        *(self.memory.get_mut_byte(0xFF26)) = 0xF1;

        //LCDC
        *(self.memory.get_mut_byte(0xFF40)) = 0x91;

        *(self.memory.get_mut_byte(0xFF42)) = 0x00;
        *(self.memory.get_mut_byte(0xFF43)) = 0x00;
        *(self.memory.get_mut_byte(0xFF45)) = 0x00;

        *(self.memory.get_mut_byte(0xFF47)) = 0xFC;
        *(self.memory.get_mut_byte(0xFF48)) = 0xFF;
        *(self.memory.get_mut_byte(0xFF49)) = 0xFF;

        *(self.memory.get_mut_byte(0xFF4A)) = 0;
        *(self.memory.get_mut_byte(0xFF4B)) = 0;

        *(self.memory.get_mut_byte(0xFFFF)) = 0x00;
    }
}

impl Default for Gameboy {
    fn default() -> Self {
        Self::new()
    }
}
