use minifb::{Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct Screen {
    pub window: Window,
    scale: usize,
}

impl Screen {
    pub fn new(scale: usize) -> Self {
        Self {
            window: Window::new(
                "Gameboy Emulator",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    scale: minifb::Scale::X4,
                    ..WindowOptions::default()
                },
            )
            .unwrap(),
            scale,
        }
    }

    pub fn render(&mut self, buffer: &[u8; 160 * 144]) {
        let mut frame_buffer: Vec<u32> = Vec::new();

        for pixel in buffer {
            let mut pixel = match pixel {
                3 => 0xFF000000,
                2 => 0xFF3C3C3C,
                1 => 0xFF787878,
                0 => 0xFFF0F0F0,
                _ => panic!("Invalid pixel"),
            };

            frame_buffer.push(pixel);
        }

        self.window
            .update_with_buffer(&frame_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
