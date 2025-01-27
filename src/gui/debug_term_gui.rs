use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{
        Constraint,
        Direction,
        Layout
    }, 
    DefaultTerminal,
    Frame
};

use crate::emulator::gameboy::Gameboy;

use super::widgets::{instructions_widget::InstructionWidget, memory_widget::{self, MemoryWidget}, registers_widget::RegistersWidget, stacks_widget::StackWidget};

#[derive(Debug)]
pub struct EmuDebugger<'a> {
    gb: &'a mut Gameboy,
    instructions_widget: InstructionWidget,
    memory_widget: MemoryWidget,
    exit: bool, 
}

impl EmuDebugger<'_> {

    pub fn new(gb: &mut Gameboy) -> EmuDebugger<'_> {
        let instructions_widget = InstructionWidget::new(&gb.memory);
        let memory_widget = MemoryWidget::new();

        EmuDebugger {
            gb,
            instructions_widget,
            memory_widget,
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

    fn draw(&mut self, frame: &mut Frame) {
        let [memory, inst, reg, stack] = Layout::horizontal([
            Constraint::Max(60),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(frame.area());


        frame.render_stateful_widget(&mut self.memory_widget, memory, &mut self.gb.memory);
        frame.render_widget(&mut self.instructions_widget, inst);

        frame.render_widget(StackWidget, stack);
        frame.render_widget(RegistersWidget::new(&self.gb.cpu), reg);

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

        self.memory_widget.handle_key_event(key_event);

        // match key_event.code {
        //     KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
        //     KeyCode::Char('j') => self.instructions_widget.state.select_next(),
        //     KeyCode::Char('k') => self.instructions_widget.state.select_previous(),
        //     _ => (),
        // }
    }

}
