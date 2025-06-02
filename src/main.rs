use std::env;
use std::io;
use std::time::SystemTime;

use gameboy::devices::screen::Screen;
use gameboy::emulator::gameboy::Gameboy;
use piston::EventSettings;
use piston::Events;
use piston::RenderEvent;
// use gameboy::{emulator::gameboy::Gameboy, gui::debug_term_gui::EmuDebugger};

// fn main() -> io::Result<()> {
//     env::set_var("RUST_BACKTRACE", "1");
//
//     let mut gb: Gameboy = Gameboy::new();
//     let _ = gb.memory.load_rom("./roms/mgb_boot.bin");
//
//     let mut terminal = ratatui::init();
//     let app_result = EmuDebugger::new(&mut gb).run(&mut terminal);
//     ratatui::restore();
//     app_result
// }

fn main() {
    let mut screen = Screen::start(2);
    let mut events = Events::new(EventSettings::new());

    let mut gb: Gameboy = Gameboy::new();
    gb.set_gb_initial_state();
    gb.load_test_ram();
    // let mut last_time = SystemTime::now();

    while let Some(e) = events.next(&mut screen.window) {
        // let current_time = SystemTime::now();
        // let deltaTime = current_time.duration_since(last_time).unwrap().as_millis();

        gb.tick(&mut screen);

        if let Some(args) = e.render_args() {
            screen.render(&args);
        }
    }
}

//TODO: Debug TUI
//  Stack list
//  Memory marker
//  Actually good stepping debug
//  Unit test
//
//TODO: Fix all instruction issue
//  KNOWN INSTRUCTION WITH ISSUE
//  CALL
//  PUSH
//  POP
//
//TODO: Screen & PPU
//  Use OpenGL(?)
//TODO: Input handling
//TODO: Audio
