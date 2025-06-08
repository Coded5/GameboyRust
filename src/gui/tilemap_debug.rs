use glutin_window::GlutinWindow as Window;
use glutin_window::OpenGL;
use graphics::{clear, Transformed};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{Filter, GlGraphics, Texture, TextureSettings};
use piston::{RenderArgs, WindowSettings};

use crate::emulator::memory::Memory;

pub struct TileMapView {
    gl: GlGraphics,
    pub window: Window,
    scale: u32,
}

impl TileMapView {
    pub fn new(scale: u32) -> TileMapView {
        let window: Window = WindowSettings::new("Gameboy Emulator", [160 * scale, 144 * scale])
            .graphics_api(OpenGL::V3_2)
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();

        TileMapView {
            gl: GlGraphics::new(OpenGL::V3_2),
            window,
            scale,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, memory: &Memory) {
        let mut settings = TextureSettings::new();
        settings.set_filter(Filter::Nearest);
    }
}
