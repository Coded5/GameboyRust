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
    current_cycle: i32,
    oam_buffer: Vec<u16>,
    mode: PpuMode,

    mode_change: bool,
}

pub fn get_lcdc(memory: &Memory, control: u8) -> bool {
    (memory.get_byte(LCDC) >> control) & 1 == 1
}

pub fn reset_stat(memory: &mut Memory, stat: u8) {
    *memory.get_mut_byte(STAT) &= !(1 << stat);
}

pub fn set_stat(memory: &mut Memory, stat: u8) {
    *memory.get_mut_byte(STAT) |= (1 << stat);
}

pub fn test_stat(memory: &Memory, stat: u8) -> bool {
    (memory.get_byte(STAT) >> stat) & 1 == 1
}

impl Ppu {
    fn oam_scan(&mut self, memory: &mut Memory) {
        //OAM $FE00 - $FE9F

        let memory_iter = (OAM_START_ADDR..OAM_END_ADDR).step_by(4);
        let is_tall_sprite = (memory.get_byte(LCDC) >> 2) & 0x1 == 1;
        let ly = memory.get_byte(LY);

        self.oam_buffer = Vec::new();

        for addr in memory_iter {
            let y_pos = memory.get_byte(addr);
            let x_pos = memory.get_byte(addr + 1);
            let tile_index = memory.get_byte(addr + 2);
            let attributes = memory.get_byte(addr + 3);

            // Sprite X-Position must be greater than 0
            //LY + 16 must be greater than or equal to Sprite Y-Position
            //LY + 16 must be less than Sprite Y-Position + Srite Height (8 in Normal Mode, 16 in Tall-Sprite-Mode)
            //The amount of sprites already stored in the OAM Buffer must be less than 10

            let sprite_height = if is_tall_sprite { 16 } else { 8 };

            if (x_pos > 0
                && ly + 16 >= y_pos
                && ly + 16 < y_pos + sprite_height
                && self.oam_buffer.len() < 10)
            {
                self.oam_buffer.push(addr);
            }
        }
    }

    fn next_scanline(&mut self, memory: &mut Memory) {
        let mut current_scanline: &mut u8 = memory.get_mut_byte(LY);
        *current_scanline = (*current_scanline).wrapping_add(1);

        if (*current_scanline > 154) {
            *current_scanline = 0;
        } else if *current_scanline >= 144 {
            self.mode = PpuMode::VBLANK;
            self.mode_change = true;
        } else {
            self.mode_change = true;
            self.mode = PpuMode::OAM_SCAN;
        }

        let ly = *current_scanline;
        let lyc = memory.get_byte(LYC);

        if lyc == ly {
            //SET LYC == LY
            set_stat(memory, 2);

            if test_stat(memory, STAT_LYC_INT) {
                // println!("LYC == LY REQUESTED; ly={} lyc={}", ly, lyc);
                request_interrupt(INT_LCD, memory);
            }
        }
    }

    fn draw_lcd(&mut self, memory: &mut Memory) {
        let y = memory.get_byte(LY);

        let scx = memory.get_byte(SCX);
        let scy = memory.get_byte(SCY);

        let wx = memory.get_byte(WX);
        let wy = memory.get_byte(WY);

        let is_8000 = get_lcdc(memory, LCDC_BG_WIN_TILE);

        let palette = memory.get_byte(BGP);

        for x in 0..160 {
            let is_window = get_lcdc(memory, LCDC_WIN_ENABLE) && (x + 7 >= wx) && (y >= wy);

            let (tile_index, line_offset, shift) = if is_window {
                self.fetch_window_tile(x, y, wx, wy, memory)
            } else {
                self.fetch_background_tile(x, y, scx, scy, memory)
            };

            let tile_address = if is_8000 {
                TILEDATA_START_ADDR + tile_index * 16 + line_offset * 2
            } else {
                let index = tile_index as i8 as i16;
                0x9000u16.wrapping_add_signed(index * 16)
            };

            let lo = memory.get_byte(tile_address);
            let hi = memory.get_byte(tile_address + 1);

            //HACK: naive render
            let lo_bit = (lo >> shift) & 1;
            let hi_bit = (hi >> shift) & 1;

            let pixel = (hi_bit << 1) | lo_bit;

            let color = (palette >> (pixel * 2)) & 0x03;

            self.frame_buffer[(x as usize) + (y as usize) * 160] = color;
        }
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
        let tile_y = (relative_y / 8) as u16;
        (
            memory.get_byte(window_tilemap_addr + tile_x + tile_y * 32) as u16,
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

        let bg_tilemap_addr = if (get_lcdc(memory, LCDC_BG_TILEMAP)) {
            0x9C00
        } else {
            0x9800
        };

        (
            memory.get_byte(bg_tilemap_addr + tile_y * 32 + tile_x) as u16,
            line_offset,
            scx.wrapping_add(x) % 8,
        )
    }

    fn draw_lcd_sprite(&mut self, memory: &mut Memory) {
        let sprite_height = if get_lcdc(memory, LCDC_OBJ_SIZE) {
            16u8
        } else {
            8u8
        };
        let y = memory.get_byte(LY);

        let scx = memory.get_byte(SCX);
        let scy = memory.get_byte(SCY);

        for &oam_entry_addr in self.oam_buffer.iter() {
            let sprite_y = memory.get_byte(oam_entry_addr);
            let sprite_x = memory.get_byte(oam_entry_addr + 1);

            let mut tile_index = memory.get_byte(oam_entry_addr + 2) as u16;
            let obj_flags = memory.get_byte(oam_entry_addr + 3);

            let priority = (obj_flags >> 7) & 1 == 0;
            let flip_x = (obj_flags >> 5) & 1 == 1;
            let flip_y = (obj_flags >> 6) & 1 == 1;

            let mut pixel_y = y.wrapping_sub(sprite_y).wrapping_add(16);

            if (flip_y) {
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

            let lo = memory.get_byte(line_address);
            let hi = memory.get_byte(line_address + 1);

            let pallete = if (obj_flags >> 4) & 1 == 0 {
                memory.get_byte(OBP0)
            } else {
                memory.get_byte(OBP1)
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
                    if (!priority
                        && (self.frame_buffer[(screen_x as usize) + (y as usize) * 160] != 0))
                    {
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

        self.mode_change = false;

        match self.mode {
            PpuMode::OAM_SCAN => {
                memory.lock_oam = true;

                if self.current_cycle >= 80 {
                    self.current_cycle -= 80;
                    self.oam_scan(memory);
                    self.mode = PpuMode::DRAW;
                    self.mode_change = true;
                }
            }
            PpuMode::DRAW => {
                memory.lock_vram = true;
                //
                // let mut penalties = 0u8;
                //
                // penalties += memory.get_byte(SCX) % 8;
                // penalties += if get_lcdc(memory, LCDC_WIN_ENABLE) { 6 } else { 0 };

                //TODO: Pixel FIFO

                if self.current_cycle >= 172 {
                    self.current_cycle -= 172;
                    self.draw_lcd(memory);
                    self.draw_lcd_sprite(memory);
                    self.mode = PpuMode::HBLANK;
                    self.mode_change = true;
                }
            }
            PpuMode::HBLANK => {
                memory.lock_vram = false;
                memory.lock_oam = false;

                if self.current_cycle >= 456 {
                    self.current_cycle -= 456;
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

        if !self.mode_change {
            return;
        }

        let mut set_ppu_mode: u8 = 0;
        match self.mode {
            PpuMode::VBLANK => {
                set_ppu_mode = 1;
                request_interrupt(INT_VBLANK, memory);
            }
            PpuMode::HBLANK => {
                set_ppu_mode = 0;
                if test_stat(memory, STAT_MODE0_INT) {
                    request_interrupt(INT_LCD, memory);
                }
            }
            PpuMode::OAM_SCAN => {
                set_ppu_mode = 2;
                if test_stat(memory, STAT_MODE2_INT) {
                    request_interrupt(INT_LCD, memory);
                }
            }
            PpuMode::DRAW => {
                set_ppu_mode = 3;
            }
        }

        *memory.get_mut_byte(STAT) &= !3;
        *memory.get_mut_byte(STAT) |= set_ppu_mode;
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
        }
    }
}
