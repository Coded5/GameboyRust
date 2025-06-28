use super::{bus::Bus, interrupt::INT_JOYPAD};

pub const JOYPAD: u16 = 0xFF00;

#[derive(Default, Debug)]
pub struct Joypad {
    pub start: bool,
    pub select: bool,
    pub btn_b: bool,
    pub btn_a: bool,
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,

    old_joypad_byte: u8,
    joypad_byte: u8,
}

impl Joypad {
    pub fn update(&mut self, bus: &mut Bus) {
        let is_button = (self.joypad_byte >> 5) & 1;
        let is_direction = (self.joypad_byte >> 4) & 1;

        self.joypad_byte =
            (((!is_button & (self.start as u8)) | (!is_direction & (self.down as u8))) << 3)
                | (((!is_button & (self.select as u8)) | (!is_direction & (self.up as u8))) << 2)
                | (((!is_button & (self.btn_b as u8)) | (!is_direction & (self.left as u8))) << 1)
                | ((!is_button & (self.btn_a as u8)) | (!is_direction & (self.right as u8)))
                | (is_direction << 4)
                | (is_button << 5);

        if self.old_joypad_byte & !self.joypad_byte != 0 {
            bus.request_interrupt(INT_JOYPAD);
            self.old_joypad_byte = self.joypad_byte;
        }
    }

    pub fn read_joypad(&self) -> u8 {
        self.joypad_byte
    }

    pub fn write_joypad(&mut self, value: u8) {
        let mask = 0b110000;
        self.joypad_byte &= !mask;
        self.joypad_byte |= value & mask;
    }
}
