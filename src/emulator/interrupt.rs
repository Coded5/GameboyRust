pub const ADDRESS_IE: u16 = 0xFFFF;
pub const ADDRESS_IF: u16 = 0xFF0F;

pub const INT_JOYPAD: u8 = 4;
pub const INT_SERIAL: u8 = 3;
pub const INT_TIMER: u8 = 2;
pub const INT_LCD: u8 = 1;
pub const INT_VBLANK: u8 = 0;

#[derive(Debug, Default)]
pub struct InterruptState {
    pub ime: bool,
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
}

impl InterruptState {
    pub fn is_requested(&self, interrupt: u8) -> bool {
        (self.interrupt_flag >> interrupt) & (self.interrupt_enable >> interrupt) & 1 == 1
    }

    pub fn have_pending(&self) -> bool {
        self.interrupt_flag & self.interrupt_enable != 0
    }
}
