use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, widgets::{Block, Paragraph, Widget}};

#[derive(Debug)]
pub struct MemoryWidget;

impl Widget for MemoryWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Memory ! !".bold())
            .border_set(border::THICK)
            .blue();

        Paragraph::new("Memoey goes here!")
            .centered()
            .block(block)
            .render(area, buf);
    }

}

impl Default for MemoryWidget {

    fn default() -> MemoryWidget {
        MemoryWidget
    }

}
