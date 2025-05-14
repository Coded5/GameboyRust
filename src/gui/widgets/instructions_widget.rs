use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, Stylize},
    symbols::border,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::{emulator::memory::Memory, gui::disassembler::disassemble_rom};

#[derive(Debug)]
pub struct InstructionWidget {
    instruction_list: Vec<(u16, String)>,
    pub state: ListState,
}

impl InstructionWidget {
    pub fn new(memory: &Memory) -> InstructionWidget {
        let mut state = ListState::default();
        state.select_first();

        let instruction_list = disassemble_rom(memory);

        InstructionWidget {
            instruction_list,
            state,
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Instruction".bold())
            .border_set(border::THICK);

        let inst_list: Vec<ListItem> = self
            .instruction_list
            .iter()
            .map(|x| ListItem::from(format!("{:04X} {}", x.0, x.1)))
            .collect();

        let list = List::new(inst_list).block(block).highlight_style(
            Style::default()
                .bg(ratatui::style::Color::White)
                .add_modifier(Modifier::BOLD)
                .black(),
        );

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    pub fn update_selected_instruction(&mut self, pc: u16) {
        //TODO: Use binary search instead
        //NOTE: Linear search will be slow but it good for now.
        let mut current_index: usize = 0;

        if (pc >= self.instruction_list.len() as u16) {
            return;
        }

        while (self.instruction_list[current_index].0 < pc) {
            current_index += 1;
        }

        self.state.select(Some(current_index));
    }
}

impl Widget for &mut InstructionWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_list(area, buf);
    }
}
