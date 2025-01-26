use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::Stylize, symbols::border, widgets::{Block, Paragraph, Widget}};

#[derive(Debug, Default)]
pub struct RegistersWidget {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl RegistersWidget {

    pub fn format_registers(&self) -> String {
        format!("
 A  : {:02X} 
 B C: {:02X} {:02X}
 D E: {:02X} {:02X}
 H L: {:02X} {:02X}
 F  : {:04b}
      ZNHC
            ", self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.f >> 4)

    }

}

impl Widget for RegistersWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(5),
                Constraint::Min(5),
            ])
            .split(area);



        let block_r8 = Block::bordered()
            .title("Register8".bold())
            .border_set(border::THICK)
            .blue();

        let block_r16 = Block::bordered()
            .title("Register16".bold())
            .border_set(border::THICK)
            .blue();

        Paragraph::new(
            self.format_registers()
        )
            .block(block_r8)
            .render(layout[0], buf);
 
        Paragraph::new("Register goes here!")
            .centered()
            .block(block_r16)
            .render(layout[1], buf);
    }

}

// impl Default for RegistersWidget {
//
//     fn default() -> RegistersWidget {
//         RegistersWidget {
//             a: 0,
//             f: 0,
//             b: 0,
//             c: 0,
//             d: 0,
//             e: 0,
//             h: 0,
//             l: 0,
//             sp: 0,
//             pc: 0,
//         }
//     }
//
// }
