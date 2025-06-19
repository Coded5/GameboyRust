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
    accum_cycle: i32,
    timer_accum_cycle: i32,
}

impl Timer {
    pub fn update(&mut self, cycle: i32, memory: &mut Memory) {
        self.accum_cycle += cycle;

        if self.accum_cycle >= 256 {
            *memory.get_mut_byte(DIV) = memory.get_byte(DIV).wrapping_add(1);
            self.accum_cycle -= 256;
        }

        self.update_timer(cycle, memory);
    }

    pub fn update_timer(&mut self, cycle: i32, memory: &mut Memory) {
        let tac_enable = (memory.get_byte(TAC) >> 2) & 1 == 1;
        let increment_time = match (memory.get_byte(TAC) & 3) {
            0 => 256 * 4,
            1 => 4 * 4,
            2 => 16 * 4,
            3 => 64 * 4,
            _ => panic!("Invalid time control"),
        };

        if !tac_enable {
            return;
        }

        self.timer_accum_cycle += cycle;

        if self.timer_accum_cycle >= increment_time {
            let (res, overflow) = memory.get_byte(TIMA).overflowing_add(1);

            *memory.get_mut_byte(TIMA) = res;

            if overflow {
                *memory.get_mut_byte(TIMA) = memory.get_byte(TMA);
                request_interrupt(INT_TIMER, memory);
            }

            self.timer_accum_cycle -= increment_time;
        }
    }

    pub fn reset_div(&mut self, memory: &mut Memory) {
        *memory.get_mut_byte(DIV) = 0;
    }
}
