use std::io;

use gameboy::gui::debug_term_gui::EmuDebugger;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = EmuDebugger::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
