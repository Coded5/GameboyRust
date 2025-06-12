use std::env;
use std::io;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::u64;

use gameboy::devices::screen::Screen;
use gameboy::emulator::gameboy::Gameboy;
use gameboy::gui::debugger::EmuDebugger;
use gameboy::logger;
use piston::Event;
use piston::EventLoop;
use piston::EventSettings;
use piston::Events;
use piston::RenderEvent;

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let event_settings = EventSettings::new().ups(u64::MAX);

    let mut events = Events::new(event_settings);
    let mut screen = Screen::start(4);

    let mut gb: Gameboy = Gameboy::new();
    let _ = gb.memory.load_rom("./roms/dmg-acid2.gb");
    gb.set_gb_initial_state();
    // gb.load_test_ram();
    // let mut last_time = SystemTime::now();

    // let mut terminal = ratatui::init();

    let _ = logger::init();

    // let mut debugger_tui = EmuDebugger::new(&gb);

    let mut start = Instant::now();

    while let Some(e) = events.next(&mut screen.window) {
        // let current_time = SystemTime::now();
        // let deltaTime = current_time.duration_since(last_time).unwrap().as_millis();

        // println!("{}", deltaTime);

        // last_time = current_time;

        gb.tick();

        if let Some(args) = e.render_args() {
            screen.render(&args, gb.get_frame_buffer());
        }

        let elapsed_time = start.elapsed();

        // if elapsed_time >= Duration::from_secs(1) {
        //     gb.accum_cycle = 0;
        //     start = Instant::now();
        // }

        // terminal.draw(|frame| debugger_tui.draw(frame))?;
    }

    // ratatui::restore();

    Ok(())
}

//TODO: Debug TUI
//  Stack list
//  Memory marker
//  Actually good stepping debug
//
//TODO: Input handling
//TODO: Audio
