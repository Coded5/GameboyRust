use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::Stylize, symbols::border, widgets::{Block, Paragraph, Widget}};

use crate::emulator::cpu::Cpu;

#[derive(Debug)]
pub struct RegistersWidget<'a> {
    cpu: &'a Cpu,
}

impl RegistersWidget<'_> {

    pub fn new(cpu: &Cpu) -> RegistersWidget<'_> {
        RegistersWidget {
            cpu
        }
    }

    pub fn format_registers(&self) -> String {
        format!("
 A  : {:02X}    PC : {:04X}
 B C: {:02X} {:02X} SP : {:04X}
 D E: {:02X} {:02X}
 H L: {:02X} {:02X}
 F  : {:04b}
      ZNHC
            ", self.cpu.a, self.cpu.pc, self.cpu.b, self.cpu.c, self.cpu.sp, self.cpu.d, self.cpu.e, self.cpu.h, self.cpu.l, self.cpu.f >> 4)

    }

}

impl Widget for RegistersWidget<'_> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block_r8 = Block::bordered()
            .title("Register8".bold())
            .border_set(border::THICK)
            .blue();

        Paragraph::new(
            self.format_registers()
        )
            .block(block_r8)
            .render(area, buf);
 
    }

}
