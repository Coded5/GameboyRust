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

    pub fn render(&mut self, args: &RenderArgs) {
        let mut settings = TextureSettings::new();
        settings.set_filter(Filter::Nearest);

        let mut data: Vec<u8> = vec![0u8; 160 * 144 * 4];

        for i in (0..data.len()).step_by(4) {
            data[i] = (i % 255) as u8;
            data[i + 1] = ((i + 100) % 255) as u8;
            data[i + 2] = ((i + 200) % 255) as u8;
            data[i + 3] = 0xFF;
        }

        let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(160, 144, self.frame.clone())
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

    //TODO: Improve this
    pub fn set_frame_from_buffer(&mut self, buffer: Vec<u8>) {
        assert_eq!(buffer.len(), 160 * 144 * 4);

        self.frame = buffer.clone();
    }
}
