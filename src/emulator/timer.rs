use super::{bus::Bus, interrupt::INT_TIMER};

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

#[derive(Default, Debug)]
pub struct Timer {
    last_tick: bool,
    div: u16,

    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
}

impl Timer {
    pub fn update(&mut self, cycle: i32, bus: &mut Bus) {
        for _ in 0..cycle {
            self.div = self.div.wrapping_add(cycle as u16);
            self.increment_tima(bus);
        }
    }

    pub fn increment_tima(&mut self, bus: &mut Bus) {
        let tac_enable = (self.tac >> 2) & 1 == 1;
        let bit = match self.tac & 3 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => panic!("Invalid time control"),
        };

        let div_bit = (self.div >> bit) & 1 == 1;
        let tick = tac_enable && div_bit;

        if self.last_tick && !tick {
            let (res, overflow) = self.tima.overflowing_add(1);
            self.tima = res;

            if overflow {
                self.tima = self.tma;
                bus.request_interrupt(INT_TIMER);
            }
        }
        self.last_tick = tick;
    }

    pub fn div_reset(&mut self) {
        self.div = 0;
    }

    pub fn div(&self) -> u8 {
        ((self.div >> 8) & 0xFF) as u8
    }
}
