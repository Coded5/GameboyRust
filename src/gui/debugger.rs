use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use piston::{EventSettings, Events, RenderEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    DefaultTerminal, Frame,
};

use crate::{devices::screen::Screen, emulator::gameboy::Gameboy};

use super::{
    gameboy_screen::ScreenWidget,
    widgets::{
        instructions_widget::InstructionWidget,
        memory_widget::{MemoryWidget, MemoryWidgetState},
        registers_widget::RegistersWidget,
        stacks_widget::{StackWidget, StackWidgetState},
        text_input_popup::InputPopup,
    },
};

pub struct EmuDebugger<'a, 'b> {
    gb: &'a mut Gameboy,
    screen: &'b mut Screen,
    stacks_widget: StackWidget,
    instructions_widget: InstructionWidget,
    memory_widget: MemoryWidget,
    screen_widget: ScreenWidget,
    exit: bool,
}

impl<'a, 'b> EmuDebugger<'a, 'b> {
    pub fn new(gb: &'a mut Gameboy, screen: &'b mut Screen) -> EmuDebugger<'a, 'b> {
        let instructions_widget = InstructionWidget::new(&gb.memory);
        let memory_widget = MemoryWidget::new();
        let stacks_widget = StackWidget::default();
        let screen_widget = ScreenWidget::default();

        gb.cpu.run(&mut gb.memory, true);

        EmuDebugger {
            gb,
            stacks_widget,
            instructions_widget,
            memory_widget,
            screen_widget,
            screen,
            exit: false,
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
        let [memory, inst, half] = Layout::horizontal([
            Constraint::Max(60),
            Constraint::Fill(1),
            Constraint::Fill(2),
        ])
        .areas(frame.area());

        let area = frame.area();
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        let [screen, bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(half);

        let [reg, stack] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(bottom);

        let marked_address = vec![self.gb.cpu.pc];
        let mut memory_state = MemoryWidgetState::new(&self.gb.memory, marked_address);
        let mut stack_state = StackWidgetState::new(self.gb.cpu.sp, &self.gb.memory);

        frame.render_stateful_widget(&mut self.memory_widget, memory, &mut memory_state);
        frame.render_widget(&mut self.instructions_widget, inst);

        frame.render_widget(ScreenWidget::default(), screen);

        frame.render_stateful_widget(&mut self.stacks_widget, stack, &mut stack_state);
        frame.render_widget(RegistersWidget::new(&self.gb.cpu), reg);

        frame.render_widget(InputPopup::new(), popup_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc {
            self.exit = true;
        }

        self.memory_widget.handle_key_event(key_event);

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Char('n') => {
                self.gb.cpu.run(&mut self.gb.memory, true);
                self.instructions_widget.state.select_next();
            }
            KeyCode::Char('m') => {
                for _ in 0..400 {
                    self.gb.cpu.run(&mut self.gb.memory, true);
                }
            }
            KeyCode::Char('g') => {
                while (!self.gb.cpu.z()) {
                    self.gb.cpu.run(&mut self.gb.memory, true);
                }
            }
            _ => (),
        }

        self.instructions_widget
            .update_selected_instruction(self.gb.cpu.pc);
    }
}
