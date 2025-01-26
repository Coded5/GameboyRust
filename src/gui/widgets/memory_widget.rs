use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Modifier, Style, Stylize}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Padding, Paragraph, Widget}};

use crate::emulator::memory::Memory;

#[derive(Debug)]
pub struct MemoryWidget<'a> {
    memory: &'a Memory 
}

impl MemoryWidget<'_> {

    pub fn new(memory: &Memory) -> MemoryWidget<'_> {
        MemoryWidget {
            memory
        }
    }

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

    pub fn get_formatted_memory(&self) -> Text<'_> {
        let mut lines = Vec::new();

        let head = Span::styled("ADDR 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F",
            Style::default()
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
        );

        lines.push(Line::from(head));

        for addr in (0..=0xFF).step_by(0x10) {
            let mut spans = vec![
                Span::styled(format!("{:04X}", addr), Style::default().bg(Color::Red)).add_modifier(Modifier::BOLD),
            ];

            for i in (0..0x10) {
                spans.push(
                    Span::from(format!(" {:02X}", self.memory.get_byte(addr + i)))
                );
            }

            lines.push(Line::from(spans));
        }

        Text::from(lines)
    }

}

impl Widget for MemoryWidget<'_> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Memory".bold())
            .border_set(border::THICK)
            .padding(Padding::horizontal(1));

        Paragraph::new(self.get_formatted_memory())
            .block(block)
            .render(area, buf);
    }

}
