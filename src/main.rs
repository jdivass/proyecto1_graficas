mod framebuffer;
mod line;

use crate::framebuffer::Framebuffer;
use std::thread;
use std::time::Duration;
use raylib::prelude::*;
fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer =
        Framebuffer::new(window_width as u32, window_height as u32, Color::new(50,50,100,255));

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;

    while !window.window_should_close() {
        translate_x += 1.0;
        translate_y += 1.0;

        framebuffer.clear();

        Framebuffer::render(
            &mut framebuffer,
            translate_x,
            translate_y,
        );

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}

