use std::env;
use std::fs::File;
use std::io;
use std::time::Duration;
use std::time::Instant;

use gameboy::devices::screen::Screen;
use gameboy::emulator::gameboy::Gameboy;
use gameboy::logger;
use log::debug;
use log::info;
use log::LevelFilter;
use num_format::Locale;
use num_format::ToFormattedString;
use piston::Button;
use piston::EventLoop;
use piston::EventSettings;
use piston::Events;
use piston::Input;
use piston::Key;
use piston::PressEvent;
use piston::ReleaseEvent;
use piston::RenderEvent;

use piston::input::Event;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::TermLogger;
use simplelog::WriteLogger;

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    CombinedLogger::init(vec![
        // WriteLogger::new(
        //     LevelFilter::Info,
        //     Config::default(),
        //     File::create("gb_log_2.log").unwrap(),
        // ),
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            simplelog::TerminalMode::Stdout,
            simplelog::ColorChoice::Auto,
        ),
    ])
    .unwrap();

    let event_settings = EventSettings::new().ups(u64::MAX);

    let mut events = Events::new(event_settings);
    let mut screen = Screen::start(4);

    let mut gb: Gameboy = Gameboy::new("./roms/gb-test-roms/cpu_instrs/cpu_instrs.gb").unwrap();

    gb.set_gb_initial_state();
    // gb.load_test_ram();
    // let mut last_time = SystemTime::now();

    // let mut terminal = ratatui::init();

    // let _ = logger::init();
    // let mut debugger_tui = EmuDebugger::new(&gb);

    let mut start = Instant::now();

    let mut debounce: bool = false;

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

        if elapsed_time >= Duration::from_secs(1) {
            let cycle = gb.accum_cycle.to_formatted_string(&Locale::en);
            info!("{cycle} T-Cycle elapsed");
            gb.accum_cycle = 0;
            start = Instant::now();
        }

        // terminal.draw(|frame| debugger_tui.draw(frame))?;

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::M && !debounce {
                info!("Memory dump!");
                let _ = gb.memory.dump_memory_to_file("memdump.bin");
                debounce = true;
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::M {
                debounce = false;
            }
        }
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
