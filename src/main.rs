mod caster;
mod framebuffer;
mod line;
mod maze;
mod movement;
mod player;
mod renderer;

use crate::framebuffer::Framebuffer;
use crate::maze::create_maze;
use crate::movement::handle_movement;
use crate::player::Player;
use crate::renderer::{render, RenderMode};
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        window_width as u32,
        window_height as u32,
        Color::new(50, 50, 100, 255),
    );

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let maze = create_maze(15, 11);
    let block_size = 40;
    let mut player = Player {
        // The start is at maze cell (1, 1), so put the player in its center.
        pos: Vector2::new(1.5 * block_size as f32, 1.5 * block_size as f32),
        a: std::f32::consts::PI / 3.0,
        fov: std::f32::consts::PI / 3.0,
    };
    let mut render_mode = RenderMode::ThreeD;

    while !window.window_should_close() {
        framebuffer.clear();
        handle_movement(&window, &mut player);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            render_mode = match render_mode {
                RenderMode::TwoD => RenderMode::ThreeD,
                RenderMode::ThreeD => RenderMode::TwoD,
            };
        }

        render(&mut framebuffer, &maze, &player, block_size, render_mode);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
