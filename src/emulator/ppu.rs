use super::memory::Memory;

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

#[allow(non_camel_case_types)]
pub enum PpuMode {
    OAM_SCAN, //Mode 2
    DRAW,     //Mode 3
    HBLANK,   //Mode 0
    VBLANK,   //Mode 1
}

pub struct Ppu {
    screen_buffer: [u8; 160 * 144],
    current_line_cycle: i32,
    current_mode_cycle: i32,
    oam_buffer: Vec<u16>,
    mode: PpuMode,
}

impl Ppu {
    fn oam_scan(&mut self, memory: &mut Memory) -> Vec<u16> {
        //OAM $FE00 - $FE9F

        let memory_iter = (OAM_START_ADDR..OAM_END_ADDR).step_by(4);
        let is_tall_sprite = (memory.get_byte(LCDC) >> 2) & 0x1 == 1;
        let ly = memory.get_byte(LY);

        let mut oam_buffer: Vec<u16> = Vec::new();

        for addr in memory_iter {
            let y_pos = memory.get_byte(addr);
            let x_pos = memory.get_byte(addr + 1);
            let tile_index = memory.get_byte(addr + 2);
            let attributes = memory.get_byte(addr + 3);

            //Sprite X-Position must be greater than 0
            //LY + 16 must be greater than or equal to Sprite Y-Position
            //LY + 16 must be less than Sprite Y-Position + Srite Height (8 in Normal Mode, 16 in Tall-Sprite-Mode)
            //The amount of sprites already stored in the OAM Buffer must be less than 10

            let sprite_height = if is_tall_sprite { 16 } else { 8 };

            if (x_pos > 0
                && ly + 16 >= y_pos
                && ly + 16 < y_pos + sprite_height
                && oam_buffer.len() < 10)
            {
                oam_buffer.push(addr);
            }
        }

        oam_buffer
    }

    fn next_scanline(&mut self, memory: &mut Memory) {
        let mut current_scanline: &mut u8 = memory.get_mut_byte(LY);
        (*current_scanline).wrapping_add(1);

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

    fn draw_pixel_fifo(&mut self, memory: &mut Memory) {
        let x_position = 0;
        let y_position = memory.get_byte(LY); //current scanline

        let scx = memory.get_byte(SCX);
        let scy = memory.get_byte(SCY);

        let tile_x = ((scx + x_position) / 8) as u16;
        let tile_y = ((scy + y_position) / 8) as u16;

        //Background Fetch
        let tile_index = memory.get_byte(TILEMAP_START_ADDR + tile_y * 32 + tile_x) as u16;

        let line_offset = (((scy + y_position) % 8) * 2) as u16;
        let tile_address = TILEDATA_START_ADDR + tile_index * 16 + line_offset;

        let hi = memory.get_byte(tile_address);
        let lo = memory.get_byte(tile_address + 1);
    }

    fn background_fetch_fifo(&mut self, memory: &mut Memory) {}

    pub fn update(&mut self, cycles: i32, memory: &mut Memory) {
        self.current_line_cycle += cycles;
        self.current_mode_cycle += cycles;

        match (self.mode) {
            PpuMode::OAM_SCAN => {
                if (self.current_mode_cycle >= 80) {
                    self.current_mode_cycle -= 80;
                    self.oam_buffer = self.oam_scan(memory);
                    self.next_scanline(memory);
                }
            }
            PpuMode::DRAW => {
                //TODO: Write to screen buffer
                todo!();
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
