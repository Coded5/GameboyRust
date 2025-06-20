use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::emulator::{cpu::Cpu, memory::Memory};

#[derive(Debug)]
pub struct StackWidget;

const SP_START: u16 = 0xFFFE;

impl StackWidget {
    fn render_stack_list(&mut self, area: Rect, buf: &mut Buffer, state: &mut StackWidgetState) {
        let mut stack_list: Vec<ListItem> = Vec::new();
        let mut current_stack = state.sp;

        let block = Block::bordered()
            .title("Stack".bold())
            .border_set(border::THICK);

        while (current_stack < SP_START) {
            let lo = state.memory.read_byte(current_stack) as u16;
            current_stack += 1;
            let hi = state.memory.read_byte(current_stack) as u16;
            current_stack += 1;

            let addr = (hi << 8) | lo;

            stack_list.push(ListItem::from(format!("{:04X}", addr)))
        }

        let list = List::new(stack_list).block(block);

        StatefulWidget::render(list, area, buf, &mut ListState::default());
    }
}

pub struct StackWidgetState<'a> {
    sp: u16,
    memory: &'a Memory,
}

impl<'a> StatefulWidget for &'a mut StackWidget {
    type State = StackWidgetState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_stack_list(area, buf, state);
    }
}

impl StackWidgetState<'_> {
    pub fn new(sp: u16, memory: &Memory) -> StackWidgetState<'_> {
        StackWidgetState { sp, memory }
    }
}

impl Default for StackWidget {
    fn default() -> StackWidget {
        StackWidget
    }
}
