use std::io;

use super::{
    cartridge::load_cartridge, cpu::Cpu, joypad::Joypad, memory::Memory, ppu::Ppu, timer::Timer,
};

pub struct Gameboy {
    pub cpu: Cpu,
    pub memory: Memory,
    pub ppu: Ppu,
    pub timer: Timer,
    pub joypad: Joypad,

    pub accum_cycle: u128,
    pub can_render: bool,
}

impl Gameboy {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Gameboy {
            cpu: Cpu::new(),
            memory: Memory::new(load_cartridge(path)?),
            ppu: Ppu::default(),
            timer: Timer::default(),
            joypad: Joypad::default(),

            accum_cycle: 0u128,
            can_render: false,
        })
    }

    pub fn get_frame_buffer(&self) -> &[u8; 160 * 144] {
        &self.ppu.frame_buffer
    }

    pub fn tick(&mut self) {
        self.joypad.update(&mut self.memory);

        let cycle = self.cpu.step(&mut self.memory);

        self.timer.update(cycle, &mut self.memory);
        self.ppu.update(cycle, &mut self.memory);

        if self.memory.dma_transfer_active {
            self.memory.update_transfer(cycle);
        }

        self.accum_cycle += cycle as u128;

        if self.ppu.finish_frame {
            self.can_render = true;
            self.ppu.finish_frame = false;
        }
    }

    // TODO: Remove this
    pub fn format_flags(&self) -> String {
        let z = if self.cpu.z() { "Z" } else { "z" };
        let n = if self.cpu.n() { "N" } else { "n" };
        let h = if self.cpu.h() { "H" } else { "h" };
        let c = if self.cpu.c() { "C" } else { "c" };

        format!("{}{}{}{}", z, n, h, c)
    }

    pub fn set_gb_initial_state(&mut self) {
        self.cpu.set_af(0x01B0);
        self.cpu.set_bc(0x0013);
        self.cpu.set_de(0x00D8);
        self.cpu.set_hl(0x014D);

        self.cpu.pc = 0x100;
        self.cpu.sp = 0xFFFE;

        self.memory.write_byte(0xFF05, 0x00);
        self.memory.write_byte(0xFF06, 0x00);
        self.memory.write_byte(0xFF07, 0x00);
        self.memory.write_byte(0xFF10, 0x80);
        self.memory.write_byte(0xFF11, 0xBF);
        self.memory.write_byte(0xFF12, 0xF3);
        self.memory.write_byte(0xFF14, 0xBF);
        self.memory.write_byte(0xFF16, 0x3F);
        self.memory.write_byte(0xFF17, 0x00);
        self.memory.write_byte(0xFF19, 0xBF);
        self.memory.write_byte(0xFF1A, 0x7F);
        self.memory.write_byte(0xFF1B, 0xFF);
        self.memory.write_byte(0xFF1C, 0x9F);
        self.memory.write_byte(0xFF1E, 0xBF);
        self.memory.write_byte(0xFF20, 0xFF);
        self.memory.write_byte(0xFF21, 0x00);
        self.memory.write_byte(0xFF22, 0x00);
        self.memory.write_byte(0xFF23, 0xBF);
        self.memory.write_byte(0xFF24, 0x77);
        self.memory.write_byte(0xFF26, 0xF1);
        self.memory.write_byte(0xFF25, 0xF3);

        self.memory.write_byte(0xFF40, 0x91);
        self.memory.write_byte(0xFF42, 0x00);
        self.memory.write_byte(0xFF43, 0x00);
        self.memory.write_byte(0xFF45, 0x00);
        self.memory.write_byte(0xFF47, 0xFC);
        self.memory.write_byte(0xFF48, 0xFF);
        self.memory.write_byte(0xFF49, 0xFF);
        self.memory.write_byte(0xFF4A, 0);
        self.memory.write_byte(0xFF4B, 0);
        self.memory.write_byte(0xFFFF, 0x00);

        self.memory.write_byte(0xFF50, 1);
    }
}
