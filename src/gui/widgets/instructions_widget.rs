use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, widgets::{Block, Paragraph, Widget}};

#[derive(Debug)]
pub struct InstructionWidget;

impl Widget for InstructionWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Instruction".bold())
            .border_set(border::THICK)
            .blue();

        Paragraph::new("Instruction goes here!")
            .centered()
            .block(block)
            .render(area, buf);
    }

}

impl Default for InstructionWidget {

    fn default() -> InstructionWidget {
        InstructionWidget
    }

}
