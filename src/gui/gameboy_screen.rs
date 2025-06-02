use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct ScreenWidget {}

impl Widget for ScreenWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Screen")
            .border_set(border::THICK)
            .white();

        Paragraph::new("Screen").block(block).render(area, buf);
    }
}
