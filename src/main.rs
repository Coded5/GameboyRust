use std::env;
use std::io;

use gameboy::{emulator::gameboy::Gameboy, gui::debug_term_gui::EmuDebugger};

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let mut gb: Gameboy = Gameboy::new();
    let _ = gb.memory.load_rom("./roms/mgb_boot.bin");

    let mut terminal = ratatui::init();
    let app_result = EmuDebugger::new(&mut gb).run(&mut terminal);
    ratatui::restore();
    app_result
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
