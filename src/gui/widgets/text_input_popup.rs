use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

pub struct InputPopup {}

impl InputPopup {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for InputPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let block = Block::new().title("Text Input Popup").borders(Borders::ALL);

        Paragraph::new("hello world")
            .wrap(Wrap { trim: true })
            .block(block)
            .render(area, buf);
    }
}
