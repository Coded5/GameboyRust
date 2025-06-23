use super::{
    cpu::{request_interrupt, INT_JOYPAD},
    memory::Memory,
};

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
}

impl Joypad {
    pub fn update(&mut self, memory: &mut Memory) {
        let is_button = (memory.read_byte(JOYPAD) >> 5) & 1;
        let is_direction = (memory.read_byte(JOYPAD) >> 4) & 1;

        let joypad_byte: u8 =
            (((!is_button & (self.start as u8)) | (!is_direction & (self.down as u8))) << 3)
                | (((!is_button & (self.select as u8)) | (!is_direction & (self.up as u8))) << 2)
                | (((!is_button & (self.btn_b as u8)) | (!is_direction & (self.left as u8))) << 1)
                | ((!is_button & (self.btn_a as u8)) | (!is_direction & (self.right as u8)))
                | (is_direction << 4)
                | (is_button << 5);

        memory.write_byte(JOYPAD, joypad_byte);

        if self.old_joypad_byte & !joypad_byte != 0 {
            request_interrupt(INT_JOYPAD, memory);
            self.old_joypad_byte = joypad_byte;
        }
    }
}
