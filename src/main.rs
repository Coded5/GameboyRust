use std::thread;
use std::time::{Duration, Instant};

use gameboy::{devices::screen::Screen, emulator::gameboy::Gameboy};
use log::info;
use log::LevelFilter;
use num_format::{Locale, ToFormattedString};
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::TermLogger;

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
        Gameboy::new("./roms/tetris.gb").unwrap();
    gameboy.set_gb_initial_state();

    let mut fps = 0;

    screen.window.set_target_fps(0);

    let cycle_cap: u128 = 4194304;

    let mut current_time = Instant::now();

    while screen.window.is_open() && !screen.window.is_key_down(minifb::Key::Escape) {
        gameboy.joypad.start = !screen.window.is_key_down(minifb::Key::A);
        gameboy.joypad.select = !screen.window.is_key_down(minifb::Key::S);
        gameboy.joypad.btn_a = !screen.window.is_key_down(minifb::Key::Z);
        gameboy.joypad.btn_b = !screen.window.is_key_down(minifb::Key::X);
        gameboy.joypad.up = !screen.window.is_key_down(minifb::Key::Up);
        gameboy.joypad.down = !screen.window.is_key_down(minifb::Key::Down);
        gameboy.joypad.left = !screen.window.is_key_down(minifb::Key::Left);
        gameboy.joypad.right = !screen.window.is_key_down(minifb::Key::Right);

        gameboy.tick();

        if gameboy.accum_cycle >= cycle_cap {
            let time_elapsed = current_time.elapsed();

            if time_elapsed < Duration::from_secs(1) {
                let sleep_time = Duration::from_secs(1).abs_diff(time_elapsed);
                thread::sleep(sleep_time);

                let cycle = gameboy.accum_cycle.to_formatted_string(&Locale::en);
                info!("{cycle} T-Cycle elapsed, {fps} FPS");
                fps = 0;
            }

            current_time = Instant::now();
            gameboy.accum_cycle -= cycle_cap;
        }

        if gameboy.can_render {
            screen.render(gameboy.get_frame_buffer());
            gameboy.can_render = false;
            fps += 1;
        }
    }
}
