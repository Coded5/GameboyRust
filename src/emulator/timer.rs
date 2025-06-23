use super::{
    cpu::{request_interrupt, INT_TIMER},
    memory::Memory,
};

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

#[derive(Default, Debug)]
pub struct Timer {
    last_tick: bool,
    div: u16,
}

impl Timer {
    pub fn update(&mut self, cycle: i32, memory: &mut Memory) {
        for _ in 0..cycle {
            self.increment_div(1, memory);
            self.increment_tima(memory);
        }
    }

    fn increment_div(&mut self, cycle: i32, memory: &mut Memory) {
        self.div = self.div.wrapping_add(cycle as u16);

        if memory.div_reset {
            self.div = 0;
            memory.div_reset = false;
        }

        let hi = (self.div >> 8) & 0xFF;
        memory.write_byte_uncheck(DIV, hi as u8);
    }

    pub fn increment_tima(&mut self, memory: &mut Memory) {
        let tac_enable = (memory.read_byte(TAC) >> 2) & 1 == 1;
        let bit = match memory.read_byte(TAC) & 3 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => panic!("Invalid time control"),
        };

        let div_bit = (self.div >> bit) & 1 == 1;
        let tick = tac_enable && div_bit;

        if self.last_tick && !tick {
            let (res, overflow) = memory.read_byte(TIMA).overflowing_add(1);

            memory.write_byte(TIMA, res);

            if overflow {
                memory.write_byte(TIMA, memory.read_byte(TMA));
                request_interrupt(INT_TIMER, memory);
            }
        }
        self.last_tick = tick;
    }
}
