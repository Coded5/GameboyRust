use std::env;
use std::io;

use gameboy::devices::screen::Screen;
use gameboy::emulator::gameboy::Gameboy;
use gameboy::gui::debugger::EmuDebugger;
use gameboy::logger;
use piston::Event;
use piston::EventSettings;
use piston::Events;
use piston::RenderEvent;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut events = Events::new(EventSettings::new());
    let mut screen = Screen::start(4);

    let mut gb: Gameboy = Gameboy::new();
    let _ = gb.memory.load_rom("./roms/mgb_boot.bin");
    // gb.set_gb_initial_state();
    // gb.load_test_ram();
    // let mut last_time = SystemTime::now();

    // let mut terminal = ratatui::init();
    // let app_result = EmuDebugger::new(&mut gb, &mut screen).run(&mut terminal);
    // ratatui::restore();

    let _ = logger::init();

    while let Some(e) = events.next(&mut screen.window) {
        // let current_time = SystemTime::now();
        // let deltaTime = current_time.duration_since(last_time).unwrap().as_millis();

        gb.tick();

        if let Some(args) = e.render_args() {
            screen.render(&args, gb.get_frame_buffer());
        }
    }
}

//TODO: Debug TUI
//  Stack list
//  Memory marker
//  Actually good stepping debug
//
//TODO: Input handling
//TODO: Audio
