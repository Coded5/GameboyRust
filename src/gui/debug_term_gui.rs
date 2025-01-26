use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect
    }, 
    style::Stylize, 
    symbols::border,
    text::Line,
    widgets::{
        Block,
        Paragraph,
        Widget
    },
    DefaultTerminal,
    Frame
};

use crate::emulator::gameboy::Gameboy;

use super::widgets::{instructions_widget::InstructionWidget, memory_widget::MemoryWidget, registers_widget::RegistersWidget, stacks_widget::StackWidget};

#[derive(Debug)]
pub struct EmuDebugger<'a> {
    gb: &'a mut Gameboy, 
    exit: bool, 
}

impl EmuDebugger<'_> {

    pub fn new(gb: &mut Gameboy) -> EmuDebugger<'_> {
        EmuDebugger {
            gb,
            exit: false
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let top_bottom_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ])
            .split(frame.area());

        let top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ])
            .split(top_bottom_layout[0]);
 
        let bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(top_bottom_layout[1]);

        frame.render_widget(MemoryWidget::new(&self.gb.memory), top_layout[0]);
        frame.render_widget(InstructionWidget, top_layout[1]);

        frame.render_widget(StackWidget, bottom_layout[0]);
        frame.render_widget(RegistersWidget::new(&self.gb.cpu), bottom_layout[1]);

    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc {
            self.exit = true;
        }
    }

}
