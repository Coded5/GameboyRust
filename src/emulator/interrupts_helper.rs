use super::memory::Memory;

pub const ADDRESS_IE: u16 = 0xFFFF;
pub const ADDRESS_IF: u16 = 0xFF0F;

pub const INT_JOYPAD: u8 = 4;
pub const INT_SERIAL: u8 = 3;
pub const INT_TIMER: u8 = 2;
pub const INT_LCD: u8 = 1;
pub const INT_VBLANK: u8 = 0;

pub fn request_interrupt(interrupt: u8, memory: &mut Memory) {
    *memory.get_mut_byte(ADDRESS_IF) |= (1 << interrupt);
}
