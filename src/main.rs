mod caster;
mod framebuffer;
mod line;
mod maze;
mod movement;
mod player;
mod renderer;
mod textures;
use crate::framebuffer::Framebuffer;
use crate::maze::create_maze;
use crate::movement::handle_movement;
use crate::player::Player;
use crate::renderer::{render, RenderMode};
use crate::textures::TextureManager;
use raylib::prelude::*;

fn main() {
    const WINDOW_SCALE: f32 = 0.8;

    let (mut window, raylib_thread) = raylib::init()
        .size(800, 600)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    window.set_target_fps(60);

    let monitor = get_current_monitor();
    let monitor_width = get_monitor_width(monitor);
    let monitor_height = get_monitor_height(monitor);
    let monitor_position = get_monitor_position(monitor);
    let window_width = (monitor_width as f32 * WINDOW_SCALE) as i32;
    let window_height = (monitor_height as f32 * WINDOW_SCALE) as i32;

    window.set_window_size(window_width, window_height);
    window.set_window_position(
        monitor_position.x as i32 + (monitor_width - window_width) / 2,
        monitor_position.y as i32 + (monitor_height - window_height) / 2,
    );

    let Some((maze_width, maze_height)) = select_level(&mut window, &raylib_thread) else {
        return;
    };

    let mut framebuffer = Framebuffer::new(
        window_width as u32,
        window_height as u32,
        Color::new(50, 50, 100, 255),
    );

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let maze = create_maze(maze_width, maze_height);
    let textures = TextureManager::new(&maze).expect("Failed to load museum textures");
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
        handle_movement(&window, &mut player, &maze, block_size);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            render_mode = match render_mode {
                RenderMode::TwoD => RenderMode::ThreeD,
                RenderMode::ThreeD => RenderMode::TwoD,
            };
        }

        render(
            &mut framebuffer,
            &maze,
            &player,
            block_size,
            &textures,
            render_mode,
        );

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}

fn select_level(window: &mut RaylibHandle, raylib_thread: &RaylibThread) -> Option<(usize, usize)> {
    loop {
        if window.window_should_close() {
            return None;
        }

        let selected_level = if window.is_key_pressed(KeyboardKey::KEY_ONE)
            || window.is_key_pressed(KeyboardKey::KEY_KP_1)
        {
            Some((15, 11))
        } else if window.is_key_pressed(KeyboardKey::KEY_TWO)
            || window.is_key_pressed(KeyboardKey::KEY_KP_2)
        {
            Some((21, 15))
        } else if window.is_key_pressed(KeyboardKey::KEY_THREE)
            || window.is_key_pressed(KeyboardKey::KEY_KP_3)
        {
            Some((31, 21))
        } else {
            None
        };

        if selected_level.is_some() {
            return selected_level;
        }

        let screen_width = window.get_screen_width();
        let screen_height = window.get_screen_height();
        let mut drawing = window.begin_drawing(raylib_thread);
        drawing.clear_background(Color::new(18, 20, 32, 255));

        draw_centered_text(
            &mut drawing,
            "MUSEUM MAZE",
            screen_width,
            screen_height / 4,
            52,
            Color::GOLD,
        );
        draw_centered_text(
            &mut drawing,
            "Select a level",
            screen_width,
            screen_height / 4 + 75,
            28,
            Color::RAYWHITE,
        );
        draw_centered_text(
            &mut drawing,
            "1  SMALL   -   15 x 11",
            screen_width,
            screen_height / 2 - 35,
            26,
            Color::LIME,
        );
        draw_centered_text(
            &mut drawing,
            "2  MEDIUM  -   21 x 15",
            screen_width,
            screen_height / 2 + 15,
            26,
            Color::SKYBLUE,
        );
        draw_centered_text(
            &mut drawing,
            "3  LARGE   -   31 x 21",
            screen_width,
            screen_height / 2 + 65,
            26,
            Color::ORANGE,
        );
        draw_centered_text(
            &mut drawing,
            "Press 1, 2, or 3 to begin",
            screen_width,
            screen_height - 90,
            20,
            Color::GRAY,
        );
    }
}

fn draw_centered_text(
    drawing: &mut RaylibDrawHandle,
    text: &str,
    screen_width: i32,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let text_width = drawing.measure_text(text, font_size);
    drawing.draw_text(text, (screen_width - text_width) / 2, y, font_size, color);
}
