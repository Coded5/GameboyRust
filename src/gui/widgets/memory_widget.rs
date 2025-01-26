use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Modifier, Style, Stylize}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Padding, Paragraph, Widget}};

#[derive(Debug)]
pub struct MemoryWidget {
    pub memory: [u8; 0x20000],
}

impl MemoryWidget {

    pub fn get_memory_str(&self) -> Text<'_> {
        let mut lines = Vec::new();

        let head = Span::styled("ADDR 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F",
            Style::default()
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
        );

        lines.push(Line::from(head));

        //TODO: Make it highlight PC and CURSOR
        for i in (0..0xFF).step_by(0x10) {
            let line = Line::from(vec![
                Span::styled(format!("{:04X}", i), Style::default().bg(Color::Red)),
                Span::styled(" 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00", Style::default())
            ]);

            lines.push(line);
        }


        Text::from(lines)
    }

}

impl Widget for MemoryWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Memory".bold())
            .border_set(border::THICK)
            .padding(Padding::horizontal(1))
            .blue();

        Paragraph::new(self.get_memory_str())
            .block(block)
            .render(area, buf);
    }

}

impl Default for MemoryWidget {

    fn default() -> MemoryWidget {
        MemoryWidget {
            memory: [0; 0x20000],
        }
    }

}
