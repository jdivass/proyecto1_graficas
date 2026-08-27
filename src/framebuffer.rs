#![allow(dead_code)]
use raylib::prelude::*;
use crate::line::line;
use crate::maze::Maze;
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width.try_into().unwrap(), height.try_into().unwrap(), background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self){
        self.color_buffer = Image::gen_image_color(self.width.try_into().unwrap(), self.height.try_into().unwrap(), self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32){

        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color){
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color){
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str){
        self.color_buffer.export_image(file_path);
    }
    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ){
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
    }
    }

}

