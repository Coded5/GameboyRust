use std::time::{Duration, Instant};

use gameboy::{devices::screen::Screen, emulator::gameboy::Gameboy};
use log::debug;
use log::info;
use num_format::{Locale, ToFormattedString};
// use simplelog::CombinedLogger;
// use simplelog::Config;
// use simplelog::ConfigBuilder;
// use simplelog::TermLogger;

use clap::Parser;

///A Gameboy Emulator
#[derive(Parser, Debug)]
struct Args {
    /// Path to rom
    #[arg(short, long)]
    rom: String,

    /// Bootrom
    #[arg(short, long, default_value_t = String::new())]
    bootrom: String,

    /// Enable logging
    #[arg(short, long, default_value_t = false)]
    logging: bool,
}

fn main() {
    let args = Args::parse();

    if args.logging {
        env_logger::init();
    }

    let mut gameboy = Gameboy::new(&args.rom).unwrap();

    let mut screen = Screen::default();

    debug!("{}", args.bootrom);

    if args.bootrom.is_empty() {
        gameboy.no_bootrom_init();
    } else {
        // let _ = gameboy.memory.load_rom(&args.bootrom);
    }

    let mut fps = 0;
    screen.window.set_target_fps(60);
    let cycle_cap: u128 = 69905;

    let mut current_time = Instant::now();
    let mut track_cycle: u128 = 0;

    while screen.window.is_open() && !screen.window.is_key_down(minifb::Key::Escape) {
        while gameboy.accum_cycle < cycle_cap {
            //A = START
            //S = select
            //A = Z
            //B = X

            let mut joypad = gameboy.joypad.borrow_mut();
            joypad.start = !screen.window.is_key_down(minifb::Key::A);
            joypad.select = !screen.window.is_key_down(minifb::Key::S);
            joypad.btn_a = !screen.window.is_key_down(minifb::Key::Z);
            joypad.btn_b = !screen.window.is_key_down(minifb::Key::X);
            joypad.up = !screen.window.is_key_down(minifb::Key::Up);
            joypad.down = !screen.window.is_key_down(minifb::Key::Down);
            joypad.left = !screen.window.is_key_down(minifb::Key::Left);
            joypad.right = !screen.window.is_key_down(minifb::Key::Right);
            drop(joypad);

            gameboy.tick();
        }

        if gameboy.can_render {
            screen.render(gameboy.get_frame_buffer());
        }

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
    }
}
