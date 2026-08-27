#![allow(dead_code)]
use raylib::prelude::*;
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
    screen_texture: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            background_color,
        );
        assert_eq!(
            color_buffer.format(),
            PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
            "Framebuffer requires an RGBA8 image"
        );
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
            screen_texture: None,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        self.set_pixel_color(x, y, self.current_color);
    }

    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            // Images created by gen_image_color use four-byte RGBA pixels.
            let offset = ((y * self.width + x) * 4) as usize;
            unsafe {
                let pixel = (self.color_buffer.data as *mut u8).add(offset);
                *pixel = color.r;
                *pixel.add(1) = color.g;
                *pixel.add(2) = color.b;
                *pixel.add(3) = color.a;
            }
        }
    }

    pub fn blend_pixel(&mut self, x: u32, y: u32, foreground: Color) {
        if x >= self.width || y >= self.height || foreground.a == 0 {
            return;
        }
        if foreground.a == 255 {
            self.set_pixel_color(x, y, foreground);
            return;
        }

        let offset = ((y * self.width + x) * 4) as usize;
        unsafe {
            let pixel = (self.color_buffer.data as *mut u8).add(offset);
            let alpha = foreground.a as f32 / 255.0;
            *pixel = blend_channel(*pixel, foreground.r, alpha);
            *pixel.add(1) = blend_channel(*pixel.add(1), foreground.g, alpha);
            *pixel.add(2) = blend_channel(*pixel.add(2), foreground.b, alpha);
            *pixel.add(3) = 255;
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if self.screen_texture.is_none() {
            self.screen_texture = window
                .load_texture_from_image(raylib_thread, &self.color_buffer)
                .ok();
        } else if let Some(texture) = self.screen_texture.as_mut() {
            let data_length = self.color_buffer.get_pixel_data_size();
            let pixels = unsafe {
                std::slice::from_raw_parts(self.color_buffer.data as *const u8, data_length)
            };
            let _ = texture.update_texture(pixels);
        }

        if let Some(texture) = self.screen_texture.as_ref() {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(texture, 0, 0, Color::WHITE);
        }
    }
}

fn blend_channel(background: u8, foreground: u8, alpha: f32) -> u8 {
    (foreground as f32 * alpha + background as f32 * (1.0 - alpha)) as u8
}
