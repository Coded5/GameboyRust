use std::cmp::Ordering;

use log::{debug, info};

use crate::emulator::cpu::{ADDRESS_IE, ADDRESS_IF};

use super::{
    cpu::{request_interrupt, INT_LCD, INT_VBLANK},
    memory::Memory,
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

    oam_buffer: Vec<u16>,
    pub mode: PpuMode,

    mode_change: bool,
    pub finish_frame: bool,
}

pub fn get_lcdc(memory: &Memory, control: u8) -> bool {
    (memory.read_byte(LCDC) >> control) & 1 == 1
}

pub fn reset_stat(memory: &mut Memory, stat: u8) {
    memory.write_byte(STAT, memory.read_byte(STAT) & !(1 << stat));
}

pub fn set_stat(memory: &mut Memory, stat: u8) {
    memory.write_byte(STAT, memory.read_byte(STAT) | (1 << stat));
}

pub fn test_stat(memory: &Memory, stat: u8) -> bool {
    let mask = (1 << STAT_LYC_INT)
        | (1 << STAT_MODE0_INT)
        | (1 << STAT_MODE1_INT)
        | (1 << STAT_MODE2_INT)
        | 7;
    ((memory.read_byte(STAT) & mask) >> stat) & 1 == 1
}

impl Ppu {
    fn oam_scan(&mut self, memory: &mut Memory) {
        //OAM $FE00 - $FE9F

        let memory_iter = (OAM_START_ADDR..OAM_END_ADDR).step_by(4);
        let is_tall_sprite = (memory.read_byte(LCDC) >> 2) & 0x1 == 1;
        let ly = memory.read_byte(LY);

        self.oam_buffer = Vec::new();

        for addr in memory_iter {
            let y_pos = memory.read_byte(addr);
            let x_pos = memory.read_byte(addr + 1);

            let sprite_height = if is_tall_sprite { 16 } else { 8 };

            // debug!("OAM Entry {addr:04X}: y={y_pos} x={x_pos} tile_index={tile_index:02X}");

            if x_pos > 0
                && ly + 16 >= y_pos
                && ly + 16 < y_pos + sprite_height
                && self.oam_buffer.len() < 10
            {
                self.oam_buffer.push(addr);
            }
        }

        // debug!(
        //     "Found {} objects to be render in LY={}",
        //     self.oam_buffer.len(),
        //     ly
        // );

        self.oam_buffer.sort_by(|a, b| {
            memory
                .read_byte(b + 1)
                .cmp(&memory.read_byte(a + 1))
                .then_with(|| b.cmp(a))
        });
    }

    fn next_scanline(&mut self, memory: &mut Memory) {
        let mut current_scanline = memory.read_byte(LY);
        current_scanline = current_scanline.wrapping_add(1);

        memory.write_byte(LY, current_scanline);

        if current_scanline > 154 {
            memory.write_byte(LY, 0);
            self.mode = PpuMode::OAM_SCAN;
            self.finish_frame = true;
        } else if current_scanline >= 144 {
            self.mode = PpuMode::VBLANK;
            self.window_line = 0;
            request_interrupt(INT_VBLANK, memory);
        } else {
            self.mode = PpuMode::OAM_SCAN;

            if test_stat(memory, STAT_MODE2_INT) {
                request_interrupt(INT_LCD, memory);
            }
        }

        let lyc = memory.read_byte(LYC);

        if lyc == current_scanline {
            set_stat(memory, 2);

            if test_stat(memory, STAT_LYC_INT) {
                // debug!(
                //     "Calling LY == LYC Interrupt during {:?} in LY({} / {:02X})",
                //     self.mode, current_scanline, current_scanline
                // );
                request_interrupt(INT_LCD, memory);
            }
        } else {
            reset_stat(memory, 2);
        }
    }

    fn draw_lcd(&mut self, memory: &mut Memory) -> [u8; 160] {
        if !get_lcdc(memory, LCDC_PRIORITY) {
            return [0u8; 160];
        }

        let y = memory.read_byte(LY);

        let scx = memory.read_byte(SCX);
        let scy = memory.read_byte(SCY);

        let wx = memory.read_byte(WX);
        let wy = memory.read_byte(WY);

        let is_8000 = get_lcdc(memory, LCDC_BG_WIN_TILE);

        let palette = memory.read_byte(BGP);

        let mut window_visible = false;

        let mut color_index = [0u8; 160];

        for x in 0..160 {
            let is_window = get_lcdc(memory, LCDC_WIN_ENABLE) && (x + 7 >= wx) && (y >= wy);

            if is_window && !window_visible {
                window_visible = true;
            }

            let (tile_index, line_offset, shift) = if is_window {
                self.fetch_window_tile(x, y, wx, wy, memory)
            } else {
                self.fetch_background_tile(x, y, scx, scy, memory)
            };

            let tile_address = if is_8000 {
                TILEDATA_START_ADDR + tile_index * 16 + line_offset * 2
            } else {
                let index = tile_index as i8 as i16;
                0x9000u16.wrapping_add_signed(index * 16) + line_offset * 2
            };

            let lo = memory.read_byte(tile_address);
            let hi = memory.read_byte(tile_address + 1);

            //HACK: naive render
            let lo_bit = (lo >> shift) & 1;
            let hi_bit = (hi >> shift) & 1;

            let pixel = (hi_bit << 1) | lo_bit;

            let color = (palette >> (pixel * 2)) & 0x03;

            color_index[x as usize] = pixel;
            self.frame_buffer[(x as usize) + (y as usize) * 160] = color;
        }

        if window_visible {
            self.window_line += 1;
        }

        color_index
    }

    fn fetch_window_tile(&self, x: u8, y: u8, wx: u8, wy: u8, memory: &Memory) -> (u16, u16, u8) {
        let relative_x = x + 7 - wx;
        let relative_y = y - wy;

        let line_offset = (relative_y % 8) as u16;

        let window_tilemap_addr = if get_lcdc(memory, LCDC_WIN_TILEMAP) {
            0x9C00
        } else {
            0x9800
        };

        let shift = 7 - (relative_x % 8);

        let tile_x = ((relative_x / 8) & 0x1F) as u16;
        let tile_y = (self.window_line / 8) as u16;
        (
            memory.read_byte(window_tilemap_addr + tile_x + tile_y * 32) as u16,
            line_offset,
            shift,
        )
    }

    fn fetch_background_tile(
        &self,
        x: u8,
        y: u8,
        scx: u8,
        scy: u8,
        memory: &Memory,
    ) -> (u16, u16, u8) {
        let tile_x = ((scx.wrapping_add(x) / 8) & 0x1F) as u16;
        let tile_y = (scy.wrapping_add(y) / 8) as u16;

        let line_offset = (scy.wrapping_add(y) % 8) as u16;

        let bg_tilemap_addr = if get_lcdc(memory, LCDC_BG_TILEMAP) {
            0x9C00
        } else {
            0x9800
        };

        (
            memory.read_byte(bg_tilemap_addr + tile_y * 32 + tile_x) as u16,
            line_offset,
            7 - (scx.wrapping_add(x) % 8),
        )
    }

    fn draw_lcd_sprite(&mut self, memory: &mut Memory, color_index: [u8; 160]) {
        if !get_lcdc(memory, LCDC_OBJ_ENABLE) {
            return;
        }

        let sprite_height = if get_lcdc(memory, LCDC_OBJ_SIZE) {
            16u8
        } else {
            8u8
        };
        let y = memory.read_byte(LY);

        let scx = memory.read_byte(SCX);
        let scy = memory.read_byte(SCY);

        for &oam_entry_addr in self.oam_buffer.iter() {
            let sprite_y = memory.read_byte(oam_entry_addr);
            let sprite_x = memory.read_byte(oam_entry_addr + 1);

            let mut tile_index = memory.read_byte(oam_entry_addr + 2) as u16;
            let obj_flags = memory.read_byte(oam_entry_addr + 3);

            let priority = (obj_flags >> 7) & 1 == 0;
            let flip_x = (obj_flags >> 5) & 1 == 1;
            let flip_y = (obj_flags >> 6) & 1 == 1;

            let mut pixel_y = y.wrapping_sub(sprite_y).wrapping_add(16);

            if flip_y {
                pixel_y = sprite_height.wrapping_sub(1u8).wrapping_sub(pixel_y);
            }

            tile_index &= if get_lcdc(memory, LCDC_OBJ_SIZE) {
                0xFE
            } else {
                0xFF
            };

            tile_index += if pixel_y >= 8 { 1 } else { 0 };

            let line_offset = (pixel_y % 8) as u16;

            let line_address = TILEDATA_START_ADDR + tile_index * 16 + line_offset * 2;

            let lo = memory.read_byte(line_address);
            let hi = memory.read_byte(line_address + 1);

            let pallete = if (obj_flags >> 4) & 1 == 0 {
                memory.read_byte(OBP0)
            } else {
                memory.read_byte(OBP1)
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

                    self.frame_buffer[(screen_x as usize) + (y as usize) * 160] = color;
                }
            }
        }
    }

    pub fn update(&mut self, cycles: i32, memory: &mut Memory) {
        if !get_lcdc(memory, LCDC_PPU_ENABLE) {
            return;
        }

        self.current_cycle += cycles;

        match self.mode {
            PpuMode::OAM_SCAN => {
                memory.lock_oam = true;

                if self.current_cycle >= 80 {
                    self.current_cycle -= 80;
                    self.oam_scan(memory);
                    self.mode = PpuMode::DRAW;
                }
            }
            PpuMode::DRAW => {
                memory.lock_vram = true;

                // let mut penalties = 0u8;
                //
                // penalties += memory.read_byte(SCX) % 8;
                // penalties += if get_lcdc(memory, LCDC_WIN_ENABLE) { 6 } else { 0 };

                if self.current_cycle >= 172 {
                    self.current_cycle -= 172;
                    let color_index = self.draw_lcd(memory);
                    self.draw_lcd_sprite(memory, color_index);
                    self.mode = PpuMode::HBLANK;

                    if test_stat(memory, STAT_MODE0_INT) {
                        request_interrupt(INT_LCD, memory);
                    }
                }
            }
            PpuMode::HBLANK => {
                memory.lock_vram = false;
                memory.lock_oam = false;

                if self.current_cycle >= 204 {
                    self.current_cycle -= 204;
                    self.next_scanline(memory);
                }
            }
            PpuMode::VBLANK => {
                memory.lock_vram = false;
                memory.lock_oam = false;
                // memory.dump_vram_to_file("vram_dump.bin");
                //Do nothing until next scanline (wait 456-T cycles)
                if self.current_cycle >= 456 {
                    self.current_cycle -= 456;
                    self.next_scanline(memory);
                }
            }
        }

        let mut set_ppu_mode: u8 = 0;
        let ppu_mode = match self.mode {
            PpuMode::VBLANK => 1,
            PpuMode::HBLANK => 0,
            PpuMode::OAM_SCAN => 2,
            PpuMode::DRAW => 3,
        };

        memory.write_byte(STAT, memory.read_byte(STAT) & !3);
        memory.write_byte(STAT, memory.read_byte(STAT) | set_ppu_mode);
    }
}

impl Default for Ppu {
    fn default() -> Ppu {
        Ppu {
            frame_buffer: [0; 160 * 144],
            mode: PpuMode::OAM_SCAN,
            current_cycle: 0,
            oam_buffer: Vec::new(),

            mode_change: false,
            finish_frame: false,
            window_line: 0,
        }
    }
}
