use log::debug;

use super::{
    bus::Bus,
    interrupt::{INT_LCD, INT_VBLANK},
};

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;

pub const TILEDATA_START_ADDR: u16 = 0x8000;
pub const TILEDATA_END_ADDR: u16 = 0x97FF;

pub const TILEMAP_START_ADDR: u16 = 0x9800;
pub const TILEMAP_END_ADDR: u16 = 0x9FFF;

pub const OAM_START_ADDR: u16 = 0xFE00;
pub const OAM_END_ADDR: u16 = 0xFE9F;

pub const CYCLES_PER_SCANLINE: i32 = 456;

pub const LCDC: u16 = 0xFF40; //LCD Control
pub const STAT: u16 = 0xFF41; //LCD Status (mode, coincidence, interrupt flags)
pub const SCY: u16 = 0xFF42; //Scroll Y
pub const SCX: u16 = 0xFF43; //Scroll X
pub const LY: u16 = 0xFF44; //Current scanline
pub const LYC: u16 = 0xFF45; //Compare LY to trigger STAT interrupt (???)
pub const DMA_TRANSFER: u16 = 0xFF46;
pub const BGP: u16 = 0xFF47; //Background Palette
pub const OBP0: u16 = 0xFF48; //Obj palette 0
pub const OBP1: u16 = 0xFF49; //Obj palette 1
pub const WY: u16 = 0xFF4A;
pub const WX: u16 = 0xFF4B;

pub const LCDC_PRIORITY: u8 = 0;
pub const LCDC_OBJ_ENABLE: u8 = 1;
pub const LCDC_OBJ_SIZE: u8 = 2;
pub const LCDC_BG_TILEMAP: u8 = 3;
pub const LCDC_BG_WIN_TILE: u8 = 4;
pub const LCDC_WIN_ENABLE: u8 = 5;
pub const LCDC_WIN_TILEMAP: u8 = 6;
pub const LCDC_PPU_ENABLE: u8 = 7;

pub const STAT_LYC_INT: u8 = 6;
pub const STAT_MODE2_INT: u8 = 5;
pub const STAT_MODE1_INT: u8 = 4;
pub const STAT_MODE0_INT: u8 = 3;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PpuMode {
    OAM_SCAN, //Mode 2
    DRAW,     //Mode 3
    HBLANK,   //Mode 0
    VBLANK,   //Mode 1
}

#[derive(Debug)]
pub struct Ppu {
    pub frame_buffer: [u8; 160 * 144],
    pub current_cycle: i32,

    window_line: u8,

    oam_buffer: Vec<usize>,
    pub mode: PpuMode,

    pub finish_frame: bool,

    dma_transfer: u8,
    dma_transfer_active: bool,
    dma_transfer_cycle: i32,

    ly: u8,
    vram: [u8; 0x2000],
    oam: [u8; 160],

    pub lcdc: u8,
    pub stat: u8,
    pub lyc: u8,
    pub scx: u8,
    pub scy: u8,
    pub wx: u8,
    pub wy: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
}

impl Default for Ppu {
    fn default() -> Ppu {
        Ppu {
            frame_buffer: [0; 160 * 144],
            mode: PpuMode::OAM_SCAN,
            current_cycle: 0,
            oam_buffer: Vec::new(),

            finish_frame: false,
            window_line: 0,

            ly: 0,
            vram: [0u8; 0x2000],
            oam: [0u8; 160],

            dma_transfer: 0,
            dma_transfer_active: false,
            dma_transfer_cycle: 0,

            lcdc: 0,
            stat: 0,
            lyc: 0,
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
        }
    }
}

impl Ppu {
    fn oam_scan(&mut self) {
        let is_tall_sprite = (self.lcdc >> 2) & 0x1 == 1;

        self.oam_buffer = Vec::new();

        for addr in (0..self.oam.len()).step_by(4) {
            let y_pos = self.oam[addr];
            let x_pos = self.oam[addr + 1];

            let sprite_height = if is_tall_sprite { 16 } else { 8 };

            if x_pos > 0
                && self.ly + 16 >= y_pos
                && self.ly + 16 < y_pos + sprite_height
                && self.oam_buffer.len() < 10
            {
                self.oam_buffer.push(addr);
            }
        }

        self.oam_buffer
            .sort_by(|a, b| self.oam[b + 1].cmp(&self.oam[a + 1]).then_with(|| b.cmp(a)));
    }

    fn next_scanline(&mut self, bus: &mut Bus) {
        self.ly = self.ly.wrapping_add(1);

        if self.ly > 154 {
            self.ly = 0;
            self.mode = PpuMode::OAM_SCAN;
            self.finish_frame = true;
        } else if self.ly >= 144 {
            self.mode = PpuMode::VBLANK;
            self.window_line = 0;
            bus.request_interrupt(INT_VBLANK);
        } else {
            self.mode = PpuMode::OAM_SCAN;

            if self.test_stat(STAT_MODE2_INT) {
                bus.request_interrupt(INT_LCD);
            }
        }

        if self.lyc == self.ly {
            self.set_stat(2);

            if self.test_stat(STAT_LYC_INT) {
                // debug!(
                //     target: "PPU",
                //     "Calling LY == LYC Interrupt during {:?} in LY({} / {:02X})",
                //     self.mode, self.ly, self.ly
                // );
                bus.request_interrupt(INT_LCD);
            }
        } else {
            self.reset_stat(2);
        }
    }

    fn draw_lcd(&mut self) -> [u8; 160] {
        if !self.get_lcdc(LCDC_PRIORITY) {
            for x in 0..160 {
                self.frame_buffer[x + (self.ly as usize) * 160] = 0;
            }

            return [0u8; 160];
        }

        let is_8000 = self.get_lcdc(LCDC_BG_WIN_TILE);

        let mut window_visible = false;

        let mut color_index = [0u8; 160];

        for x in 0..160 {
            let is_window =
                self.get_lcdc(LCDC_WIN_ENABLE) && (x + 7 >= self.wx) && (self.ly >= self.wy);

            if is_window && !window_visible {
                window_visible = true;
            }

            let (tile_index, line_offset, shift) = if is_window {
                self.fetch_window_tile(x, self.ly, self.wx, self.wy)
            } else {
                self.fetch_background_tile(x, self.ly, self.scx, self.scy)
            };

            let tile_address = if is_8000 {
                TILEDATA_START_ADDR + tile_index * 16 + line_offset * 2
            } else {
                let index = tile_index as i8 as i16;
                0x9000u16.wrapping_add_signed(index * 16) + line_offset * 2
            };

            let lo = self.read_vram(tile_address);
            let hi = self.read_vram(tile_address + 1);

            //HACK: naive render
            let lo_bit = (lo >> shift) & 1;
            let hi_bit = (hi >> shift) & 1;

            let pixel = (hi_bit << 1) | lo_bit;

            let color = (self.bgp >> (pixel * 2)) & 0x03;

            color_index[x as usize] = pixel;
            self.frame_buffer[(x as usize) + (self.ly as usize) * 160] = color;
        }

        if window_visible {
            self.window_line += 1;
        }

        color_index
    }

    fn fetch_window_tile(&self, x: u8, y: u8, wx: u8, wy: u8) -> (u16, u16, u8) {
        let relative_x = x + 7 - wx;
        let relative_y = y - wy;

        let line_offset = (relative_y % 8) as u16;

        let window_tilemap_addr = if self.get_lcdc(LCDC_WIN_TILEMAP) {
            0x9C00
        } else {
            0x9800
        };

        let shift = 7 - (relative_x % 8);

        let tile_x = ((relative_x / 8) & 0x1F) as u16;
        let tile_y = (self.window_line / 8) as u16;
        (
            self.read_vram(window_tilemap_addr + tile_x + tile_y * 32) as u16,
            line_offset,
            shift,
        )
    }

    fn fetch_background_tile(&self, x: u8, y: u8, scx: u8, scy: u8) -> (u16, u16, u8) {
        let tile_x = ((scx.wrapping_add(x) / 8) & 0x1F) as u16;
        let tile_y = (scy.wrapping_add(y) / 8) as u16;

        let line_offset = (scy.wrapping_add(y) % 8) as u16;

        let bg_tilemap_addr = if self.get_lcdc(LCDC_BG_TILEMAP) {
            0x9C00
        } else {
            0x9800
        };

        (
            self.read_vram(bg_tilemap_addr + tile_y * 32 + tile_x) as u16,
            line_offset,
            7 - (scx.wrapping_add(x) % 8),
        )
    }

    fn draw_lcd_sprite(&mut self, color_index: [u8; 160]) {
        if !self.get_lcdc(LCDC_OBJ_ENABLE) {
            return;
        }

        let sprite_height = if self.get_lcdc(LCDC_OBJ_SIZE) {
            16u8
        } else {
            8u8
        };
        for &oam_entry_addr in self.oam_buffer.iter() {
            let sprite_y = self.oam[oam_entry_addr];
            let sprite_x = self.oam[oam_entry_addr + 1];

            let mut tile_index = self.oam[oam_entry_addr + 2] as u16;
            let obj_flags = self.oam[oam_entry_addr + 3];

            let priority = (obj_flags >> 7) & 1 == 0;
            let flip_x = (obj_flags >> 5) & 1 == 1;
            let flip_y = (obj_flags >> 6) & 1 == 1;

            let mut pixel_y = self.ly.wrapping_sub(sprite_y).wrapping_add(16);

            if flip_y {
                pixel_y = sprite_height.wrapping_sub(1u8).wrapping_sub(pixel_y);
            }

            tile_index &= if self.get_lcdc(LCDC_OBJ_SIZE) {
                0xFE
            } else {
                0xFF
            };

            tile_index += if pixel_y >= 8 { 1 } else { 0 };

            let line_offset = (pixel_y % 8) as u16;

            let line_address = TILEDATA_START_ADDR + tile_index * 16 + line_offset * 2;

            let lo = self.read_vram(line_address);
            let hi = self.read_vram(line_address + 1);

            let pallete = if (obj_flags >> 4) & 1 == 0 {
                self.obp0
            } else {
                self.obp1
            };

            //HACK: naive render!
            for bit in 0..8 {
                let shift = if flip_x { bit } else { 7 - bit };
                let lo_bit = (lo >> shift) & 1;
                let hi_bit = (hi >> shift) & 1;
                let pixel = (hi_bit << 1) | lo_bit;

                if pixel == 0 {
                    continue;
                }

                let screen_x = sprite_x.wrapping_sub(8).wrapping_add(bit);
                if screen_x < 160 {
                    if !priority && color_index[screen_x as usize] != 0 {
                        continue;
                    }

                    let color = (pallete >> (pixel * 2)) & 0x3;

                    self.frame_buffer[(screen_x as usize) + (self.ly as usize) * 160] = color;
                }
            }
        }
    }

    pub fn update(&mut self, cycles: i32, bus: &mut Bus) {
        if !self.get_lcdc(LCDC_PPU_ENABLE) {
            return;
        }

        self.current_cycle += cycles;

        match self.mode {
            PpuMode::OAM_SCAN => {
                // memory.lock_oam = true;

                if self.current_cycle >= 80 {
                    self.current_cycle -= 80;
                    self.oam_scan();
                    self.mode = PpuMode::DRAW;
                }
            }
            PpuMode::DRAW => {
                // memory.lock_vram = true;

                // let mut penalties = 0u8;
                //
                // penalties += memory.read_byte(SCX) % 8;
                // penalties += if self.get_lcdc(LCDC_WIN_ENABLE) { 6 } else { 0 };

                if self.current_cycle >= 172 {
                    self.current_cycle -= 172;
                    // debug!(target: "PPU", "{:#8b}", self.lcdc);
                    let color_index = self.draw_lcd();
                    self.draw_lcd_sprite(color_index);
                    self.mode = PpuMode::HBLANK;

                    if self.test_stat(STAT_MODE0_INT) {
                        bus.request_interrupt(INT_LCD);
                    }
                }
            }
            PpuMode::HBLANK => {
                // memory.lock_vram = false;
                // memory.lock_oam = false;

                if self.current_cycle >= 204 {
                    self.current_cycle -= 204;
                    self.next_scanline(bus);
                }
            }
            PpuMode::VBLANK => {
                // memory.lock_vram = false;
                // memory.lock_oam = false;
                if self.current_cycle >= 456 {
                    self.current_cycle -= 456;
                    self.next_scanline(bus);
                }
            }
        }

        let ppu_mode = match self.mode {
            PpuMode::VBLANK => 1,
            PpuMode::HBLANK => 0,
            PpuMode::OAM_SCAN => 2,
            PpuMode::DRAW => 3,
        };

        self.stat &= !3;
        self.stat |= ppu_mode;

        self.update_dma_transfer(cycles, bus);
    }

    pub fn update_dma_transfer(&mut self, cycle: i32, bus: &mut Bus) {
        if !self.dma_transfer_active {
            return;
        }

        self.dma_transfer_cycle += cycle;

        if self.dma_transfer_cycle >= 160 * 4 {
            let start_addr: u16 = (self.dma_transfer as u16) << 8;
            let end_addr: u16 = ((self.dma_transfer as u16) << 8) | 0x9F;

            for (offset, addr) in (start_addr..=end_addr).enumerate() {
                self.oam[offset] = bus.read_byte(addr);
            }

            self.dma_transfer_cycle = 0;
            self.dma_transfer_active = false;
        }
    }

    pub fn start_dma_transfer(&mut self, value: u8) {
        self.dma_transfer = value;
        self.dma_transfer_active = true;
        self.dma_transfer_cycle = 0;
    }

    pub fn get_lcdc(&self, control: u8) -> bool {
        ((self.lcdc >> control) & 1) == 1
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram[(address - 0x8000) as usize]
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.vram[(address - 0x8000) as usize] = value;
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        // debug!(target: "PPU", "Reading OAM: {address:04X}");
        self.oam[(address - 0xFE00) as usize]
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        self.oam[(address - 0xFE00) as usize] = value;
    }

    pub fn ly(&self) -> u8 {
        self.ly
    }

    pub fn reset_stat(&mut self, stat: u8) {
        self.stat &= !(1 << stat);
    }

    pub fn set_stat(&mut self, stat: u8) {
        self.stat |= 1 << stat;
    }

    pub fn test_stat(&self, stat: u8) -> bool {
        let mask = (1 << STAT_LYC_INT)
            | (1 << STAT_MODE0_INT)
            | (1 << STAT_MODE1_INT)
            | (1 << STAT_MODE2_INT)
            | 7;
        ((self.stat & mask) >> stat) & 1 == 1
    }
}
