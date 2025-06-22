use std::fs::File;
use std::thread;
use std::time::{Duration, Instant};

use gameboy::{devices::screen::Screen, emulator::gameboy::Gameboy};
use log::LevelFilter;
use log::{debug, info};
use num_format::{Locale, ToFormattedString};
use simplelog::Config;
use simplelog::TermLogger;
use simplelog::{CombinedLogger, WriteLogger};

fn main() {
    CombinedLogger::init(vec![
        // WriteLogger::new(
        //     LevelFilter::Info,
        //     Config::default(),
        //     File::create("gb_log_2.log").unwrap(),
        // ),
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            simplelog::TerminalMode::Stdout,
            simplelog::ColorChoice::Auto,
        ),
        // TermLogger::new(
        //     LevelFilter::Info,
        //     Config::default(),
        //     simplelog::TerminalMode::Stdout,
        //     simplelog::ColorChoice::Auto,
        // ),
        // TermLogger::new(
        //     LevelFilter::Warn,
        //     Config::default(),
        //     simplelog::TerminalMode::Stdout,
        //     simplelog::ColorChoice::Auto,
        // ),
    ])
    .unwrap();
    let mut screen = Screen::new(4);

    let mut gameboy: Gameboy =
        // Gameboy::new("./roms/gb-test-roms/cpu_instrs/cpu_instrs.gb").unwrap();
    Gameboy::new("./roms/bgbtest.gb").unwrap();
    gameboy.set_gb_initial_state();

    // let _ = gameboy.memory.load_rom("./roms/mgb_boot.bin");

    let mut fps = 0;

    screen.window.set_target_fps(60);

    // let cycle_cap: u128 = 4194304;
    let cycle_cap: u128 = 69905;

    let mut current_time = Instant::now();

    let mut d = false;

    let mut track_cycle: u128 = 0;

    while screen.window.is_open() && !screen.window.is_key_down(minifb::Key::Escape) {
        while gameboy.accum_cycle < cycle_cap {
            //A = START
            //S = select
            //A = Z
            //B = X
            gameboy.joypad.start = !screen.window.is_key_down(minifb::Key::A);
            gameboy.joypad.select = !screen.window.is_key_down(minifb::Key::S);
            gameboy.joypad.btn_a = !screen.window.is_key_down(minifb::Key::Z);
            gameboy.joypad.btn_b = !screen.window.is_key_down(minifb::Key::X);
            gameboy.joypad.up = !screen.window.is_key_down(minifb::Key::Up);
            gameboy.joypad.down = !screen.window.is_key_down(minifb::Key::Down);
            gameboy.joypad.left = !screen.window.is_key_down(minifb::Key::Left);
            gameboy.joypad.right = !screen.window.is_key_down(minifb::Key::Right);
            gameboy.tick();
        }

        screen.render(gameboy.get_frame_buffer());
        fps += 1;
        track_cycle += gameboy.accum_cycle;
        gameboy.accum_cycle -= cycle_cap;

        if current_time.elapsed() >= Duration::from_secs(1) {
            let cycle = track_cycle.to_formatted_string(&Locale::en);
            info!("{cycle} T-cycle, {fps} FPS");
            track_cycle = 0;
            fps = 0;
            gameboy.accum_cycle = 0;
            current_time = Instant::now();
        }

        if screen.window.is_key_down(minifb::Key::O) && !d {
            let _ = gameboy.memory.dump_oam_to_file("oam.bin");
            d = true;
        }

        if screen.window.is_key_down(minifb::Key::O) {
            d = false;
        }
    }
}
