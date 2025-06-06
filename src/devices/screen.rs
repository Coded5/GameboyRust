use glutin_window::OpenGL;
use graphics::{clear, Transformed};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{Filter, GlGraphics, Texture, TextureSettings};
use piston::{RenderArgs, WindowSettings};

use glutin_window::GlutinWindow as Window;

pub struct Screen {
    gl: GlGraphics,
    pub window: Window,
    scale: u32,
    frame: Vec<u8>,
}

impl Screen {
    pub fn start(scale: u32) -> Screen {
        let window: Window = WindowSettings::new("Gameboy Emulator", [160 * scale, 144 * scale])
            .graphics_api(OpenGL::V3_2)
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Screen {
            gl: GlGraphics::new(OpenGL::V3_2),
            window,
            scale,
            frame: vec![0u8; 160 * 144 * 4],
        }
    }

    pub fn render(&mut self, args: &RenderArgs, screen_buffer: &[u8; 160 * 144]) {
        let mut settings = TextureSettings::new();
        settings.set_filter(Filter::Nearest);

        let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(160, 144, self.get_video_buffer_rgba(screen_buffer))
                .expect("Failed to create image buffer");

        let texture = Texture::from_image(&img_buffer, &settings);

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            graphics::image(
                &texture,
                c.transform.scale(self.scale as f64, self.scale as f64),
                gl,
            );
        })
    }

    pub fn get_video_buffer_rgba(&self, buffer: &[u8; 160 * 144]) -> Vec<u8> {
        let mut frame_buffer: Vec<u8> = Vec::new();

        for pixel in buffer {
            let mut pixel_data: Vec<u8> = match pixel {
                3 => vec![0, 0, 0, 255],
                2 => vec![60, 60, 60, 255],
                1 => vec![120, 120, 120, 255],
                0 => vec![240, 240, 240, 255],
                _ => panic!("Invalid pixel"),
            };

            frame_buffer.append(&mut pixel_data);
        }

        frame_buffer
    }
}
