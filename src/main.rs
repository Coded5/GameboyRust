use std::{env, io};

use gameboy::{emulator::gameboy::Gameboy, gui::{debug_term_gui::EmuDebugger, disassembler::disassemble_rom}};

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let mut gb: Gameboy = Gameboy::new();
    let _ = gb.memory.load_rom("./roms/mgb_boot.bin");

    let mut terminal = ratatui::init();
    let app_result = EmuDebugger::new(&mut gb).run(&mut terminal);
    ratatui::restore();
    app_result
}
