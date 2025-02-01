use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, text::Text, widgets::{Cell, List, ListItem, ListState, Row, StatefulWidget, Table, TableState}};

use crate::emulator::memory::Memory;

#[derive(Debug)]
pub struct MemoryWidget {
    cstate: ListState,
    state: TableState,
}

impl MemoryWidget {

    pub fn new() -> MemoryWidget {
        let mut state = TableState::default();
        let mut cstate = ListState::default();
        state.select_first();
        cstate.select_first();
        
        MemoryWidget {
            cstate,
            state,
        }
    }

    fn render_table(&mut self, area: Rect, buf: &mut Buffer, state: &mut MemoryWidgetState) {

        let rows = (0..0xFFF0_i32).step_by(0x10)
            .map(|addr| {
                (addr..addr+0x10)
                    .map(|offset| {
                        let style = if state.marked_address.iter().any(|&i| i == offset as u16) {
                            Style::default().bg(Color::Red).bold()
                        }
                        else {
                            Style::default()
                        };

                        Cell::from(
                            Text::from(format!("{:02X}", state.memory.get_byte(offset as u16)))
                            .style(style)
                        )
                    })
                    .collect::<Row>()
                    .height(1)
            });

        let col_items = (0..0xFFF0).step_by(0x10).map(|x| {
            ListItem::from(format!("{:04X}", x))
        });

        let headers = (0..16).map(|x| {
            Cell::from(Text::from(format!("{:02X}", x)))
        }).collect::<Row>().height(1);

        let list = List::new(col_items)
            .highlight_style(Style::default().bg(Color::Red));

        let table = Table::new(rows, [Constraint::Max(2); 16])
            .header(headers)
            .row_highlight_style(Style::default().bg(Color::Red))
            .column_highlight_style(Style::default().bg(Color::Red));

        let [list_layout, table_layout] = Layout::horizontal([
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [_, compensated_list_layout] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1)
        ])
        .areas(list_layout);

        StatefulWidget::render(list, compensated_list_layout, buf, &mut self.cstate);
        StatefulWidget::render(table, table_layout, buf, &mut self.state);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k') => {
                self.state.select_previous();
                self.cstate.select_previous();
            },
            KeyCode::Char('j') => {
                self.state.select_next();
                self.cstate.select_next();
            },
            KeyCode::Char('l') => {
                self.state.select_next_column();
            },
            KeyCode::Char('h') => {
                self.state.select_previous_column();
            }
            _ => (),
        }
    }

}

impl Default for MemoryWidget {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MemoryWidgetState<'a> {
    memory: &'a Memory,
    marked_address: Vec<u16>
}

impl MemoryWidgetState<'_> {

    pub fn new(memory: &Memory, marked_address: Vec<u16>) -> MemoryWidgetState<'_> {
        MemoryWidgetState {
            memory,
            marked_address
        }
    }

}

impl<'a> StatefulWidget for &'a mut MemoryWidget {
    type State = MemoryWidgetState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut MemoryWidgetState) {
        self.render_table(area, buf, state);
    }

}
