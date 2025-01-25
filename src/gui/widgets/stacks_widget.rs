
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, widgets::{Block, Paragraph, Widget}};

#[derive(Debug)]
pub struct StackWidget;

impl Widget for StackWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Stack ! !".bold())
            .border_set(border::THICK)
            .blue();

        Paragraph::new("Stack goes here!")
            .centered()
            .block(block)
            .render(area, buf);
    }

}

impl Default for StackWidget {

    fn default() -> StackWidget {
        StackWidget
    }

}
