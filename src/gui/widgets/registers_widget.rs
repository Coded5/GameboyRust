use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, widgets::{Block, Paragraph, Widget}};

#[derive(Debug)]
pub struct RegistersWidget;

impl Widget for RegistersWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Register! !".bold())
            .border_set(border::THICK)
            .blue();

        Paragraph::new("Register goes here!")
            .centered()
            .block(block)
            .render(area, buf);
    }

}

impl Default for RegistersWidget {

    fn default() -> RegistersWidget {
        RegistersWidget
    }

}
