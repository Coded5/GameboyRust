use std::fs::{self, File};
use std::io::Read;
use std::io::{self};

use log::warn;

use super::gameboy::Shared;
use super::interrupt::{InterruptState, ADDRESS_IF};
use super::joypad::{Joypad, JOYPAD};
use super::mbcs::mbc::MBC;
use super::ppu::{Ppu, BGP, DMA_TRANSFER, LCDC, LY, LYC, OBP0, OBP1, SCX, SCY, STAT, WX, WY};
use super::timer::{Timer, DIV, TAC, TIMA, TMA};

pub struct Bus {
    mbc: Box<dyn MBC>,

    pub dma_transfer_active: bool,
    dma_transfer_cycle: i32,
    dma_transfer: u8,

    pub interrupt: Shared<InterruptState>,
    pub ppu: Shared<Ppu>,
    pub timer: Shared<Timer>,
    pub joypad: Shared<Joypad>,

    bootrom_enable: bool,
    bootrom: [u8; 0x100],
    wram: [u8; 0x2000],
    hram: [u8; 127],
}

impl Bus {
    pub fn new(
        mbc: Box<dyn MBC>,
        interrupt: Shared<InterruptState>,
        ppu: Shared<Ppu>,
        timer: Shared<Timer>,
        joypad: Shared<Joypad>,
    ) -> Bus {
        Bus {
            dma_transfer_active: false,
            dma_transfer_cycle: 0,
            dma_transfer: 0,

            interrupt,
            ppu,
            timer,
            joypad,

            bootrom_enable: false,
            bootrom: [0u8; 0x100],
            wram: [0u8; 0x2000],
            hram: [0u8; 127],

            mbc,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.read_byte(address),
            0x8000..=0x9FFF => self.ppu.borrow().read_vram(address),
            0xA000..=0xBFFF => self.mbc.read_byte(address),
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => 0,
            0xFE00..=0xFE9F => self.ppu.borrow().read_oam(address),
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.read_register(address),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt.borrow().interrupt_enable,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.ppu.borrow_mut().write_vram(address, value),
            0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => (),
            0xFE00..=0xFE9F => self.ppu.borrow_mut().write_oam(address, value),
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => self.write_register(address, value),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt.borrow_mut().interrupt_enable = value,
        }
    }

    pub fn read_register(&self, address: u16) -> u8 {
        match address {
            JOYPAD => self.joypad.borrow().read_joypad(),
            DIV => self.timer.borrow().div(),
            TIMA => self.timer.borrow().tima,
            TMA => self.timer.borrow().tma,
            TAC => self.timer.borrow().tac,
            ADDRESS_IF => self.interrupt.borrow().interrupt_flag,
            LCDC => self.ppu.borrow().lcdc,
            STAT => self.ppu.borrow().stat,
            SCY => self.ppu.borrow().scy,
            SCX => self.ppu.borrow().scx,
            LY => self.ppu.borrow().ly(),
            LYC => self.ppu.borrow().lyc,
            BGP => self.ppu.borrow().bgp,
            OBP0 => self.ppu.borrow().obp0,
            OBP1 => self.ppu.borrow().obp1,
            WY => self.ppu.borrow().wy,
            WX => self.ppu.borrow().wx,
            0xFF50 => !self.bootrom_enable as u8,
            _ => {
                warn!(target: "Bus","Invalid or unimplement IO register {:04X}", address);
                0
            }
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        match address {
            JOYPAD => self.joypad.borrow_mut().write_joypad(value),
            DIV => self.timer.borrow_mut().div_reset(),
            TIMA => self.timer.borrow_mut().tima = value,
            TMA => self.timer.borrow_mut().tma = value,
            TAC => self.timer.borrow_mut().tac = value,
            ADDRESS_IF => self.interrupt.borrow_mut().interrupt_flag = value,
            LCDC => self.ppu.borrow_mut().lcdc = value,
            STAT => self.ppu.borrow_mut().stat = value,
            SCY => self.ppu.borrow_mut().scy = value,
            SCX => self.ppu.borrow_mut().scx = value,
            LY => (), //Read only
            LYC => self.ppu.borrow_mut().lyc = value,
            DMA_TRANSFER => self.ppu.borrow_mut().start_dma_transfer(value),
            BGP => self.ppu.borrow_mut().bgp = value,
            OBP0 => self.ppu.borrow_mut().obp0 = value,
            OBP1 => self.ppu.borrow_mut().obp1 = value,
            WY => self.ppu.borrow_mut().wy = value,
            WX => self.ppu.borrow_mut().wx = value,
            0xFF50 => self.bootrom_enable = value == 0,
            _ => warn!(target: "Bus","Invalid or unimplement IO register {:04X}", address),
        }
    }

    pub fn load_bootrom(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let length: usize = fs::metadata(path)?.len() as usize;
        let mut bootrom: Vec<u8> = vec![0u8; length];
        file.read_exact(&mut bootrom)?;

        self.bootrom_enable = true;

        for (i, &byte) in bootrom.iter().enumerate() {
            self.bootrom[i] = byte;
        }

        Ok(())
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        self.interrupt.borrow_mut().interrupt_flag |= 1 << interrupt;
    }
}
