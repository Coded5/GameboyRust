use super::memory::Memory;

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
    screen_buffer: [u8; 160 * 144],
    current_line_cycle: i32,
    current_mode_cycle: i32,
    oam_buffer: Vec<u16>,
    mode: PpuMode,
}

pub fn get_lcdc(memory: &Memory, control: u8) -> bool {
    (memory.get_byte(LCDC) >> control) & 1 == 1
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

        //Enter V-Blank
        if (*current_scanline >= 144) {
            self.mode = PpuMode::VBLANK;
            //TODO: Request interrupt here?
        } else if (*current_scanline > 155) {
            *current_scanline = 0;
        } else {
            self.mode = PpuMode::OAM_SCAN;
        }
    }

    fn draw_lcd(&mut self, memory: &mut Memory) {
        let y = memory.get_byte(LY);

        let scx = memory.get_byte(SCX);
        let scy = memory.get_byte(SCY);

        let wx = memory.get_byte(WX);
        let wy = memory.get_byte(WY);

        let is_8000 = get_lcdc(memory, LCDC_BG_WIN_TILE);

        for x in 0..160 {
            let is_window = get_lcdc(memory, LCDC_WIN_ENABLE) && (x >= wx - 7) && (y >= wy);

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
            self.screen_buffer[(x as usize) + (y as usize) * 160] = pixel;
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
            (scx + x) % 8,
        )
    }

    //NOTE: Ignore OBJ Priority for now.
    fn draw_lcd_sprite(&mut self, memory: &mut Memory) {
        let y = memory.get_byte(LY);

        let scx = memory.get_byte(SCX);
        let scy = memory.get_byte(SCY);

        for &oam_entry_addr in self.oam_buffer.iter() {
            let yy = memory.get_byte(oam_entry_addr);
            let xx = memory.get_byte(oam_entry_addr + 1);
            let tile_index = memory.get_byte(oam_entry_addr + 2) as u16;
            let _obj_flags = memory.get_byte(oam_entry_addr + 3);

            let line_offset = scy.wrapping_add(y).wrapping_add(16).wrapping_sub(yy) as u16;

            let line_address = TILEDATA_START_ADDR + tile_index * 16 + line_offset * 2;

            let lo = memory.get_byte(line_address);
            let hi = memory.get_byte(line_address + 1);

            //HACK: naive render!
            //8x16 sprite not suported!
            for bit in 0..8 {
                let lo_bit = (lo >> (7 - bit)) & 1;
                let hi_bit = (hi >> (7 - bit)) & 1;
                let pixel = (hi_bit << 1) | lo_bit;

                if pixel == 0 {
                    continue;
                }

                let screen_x = xx.wrapping_sub(8).wrapping_add(bit);
                if screen_x < 160 {
                    self.screen_buffer[(screen_x as usize) + (y as usize) * 160] = pixel;
                }
            }
        }
    }

    pub fn update(&mut self, cycles: i32, memory: &mut Memory) {
        self.current_line_cycle += cycles;
        self.current_mode_cycle += cycles;

        match (self.mode) {
            PpuMode::OAM_SCAN => {
                if (self.current_mode_cycle >= 80) {
                    self.current_mode_cycle -= 80;
                    self.oam_scan(memory);
                    self.mode = PpuMode::DRAW;
                }
            }
            PpuMode::DRAW => {
                //TODO: Pixel FIFO
                self.draw_lcd(memory);
                self.draw_lcd_sprite(memory);
                self.mode = PpuMode::HBLANK;
            }
            PpuMode::HBLANK => {
                if (self.current_line_cycle >= 456) {
                    self.current_line_cycle -= 456;
                    self.next_scanline(memory);
                }
            }
            PpuMode::VBLANK => {
                //Do nothing until next scanline (wait 456-T cycles)
                if (self.current_line_cycle >= 456) {
                    self.current_line_cycle -= 456;
                    self.next_scanline(memory);
                }
            }
        }
    }

    pub fn get_video_buffer_rgba(&self) -> Vec<u8> {
        let mut frame_buffer: Vec<u8> = Vec::new();

        //TODO: Obey GB Palette
        //TODO: Change temoporary color
        for pixel in self.screen_buffer {
            let mut pixel_data: Vec<u8> = match pixel {
                0 => vec![0, 0, 0, 255],
                1 => vec![60, 60, 60, 255],
                2 => vec![120, 120, 120, 255],
                3 => vec![240, 240, 240, 255],
                _ => panic!("Invalid pixel"),
            };

            frame_buffer.append(&mut pixel_data);
        }

        frame_buffer
    }

    //HACK: Temporary remove me
    pub fn is_vblank(&self) -> bool {
        matches!(self.mode, PpuMode::VBLANK)
    }
}

impl Default for Ppu {
    fn default() -> Ppu {
        Ppu {
            screen_buffer: [0; 160 * 144],
            mode: PpuMode::OAM_SCAN,
            current_line_cycle: 0,
            current_mode_cycle: 0,
            oam_buffer: Vec::new(),
        }
    }
}
