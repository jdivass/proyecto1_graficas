use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{render_maze, Maze};
use crate::player::Player;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RenderMode {
    TwoD,
    ThreeD,
}

pub fn render(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    mode: RenderMode,
) {
    match mode {
        RenderMode::TwoD => render_2d(framebuffer, maze, player, block_size),
        RenderMode::ThreeD => render_3d(framebuffer, maze, player, block_size),
    }
}

fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    render_maze(framebuffer, maze, block_size);

    framebuffer.set_current_color(Color::YELLOW);
    let num_rays = 60;

    for i in 0..num_rays {
        let ray_progress = i as f32 / (num_rays - 1) as f32;
        let a = player.a - player.fov / 2.0 + player.fov * ray_progress;
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }

    framebuffer.set_current_color(Color::MAGENTA);
    let player_radius = 4;
    for x in player.pos.x as i32 - player_radius..=player.pos.x as i32 + player_radius {
        for y in player.pos.y as i32 - player_radius..=player.pos.y as i32 + player_radius {
            if x >= 0 && y >= 0 {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

fn render_3d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let projection_plane = (width as f32 / 2.0) / (player.fov / 2.0).tan();

    for column in 0..width {
        let ray_progress = column as f32 / (width - 1) as f32;
        let a = player.a - player.fov / 2.0 + player.fov * ray_progress;
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);

        // Corregir ojo de pescado
        let corrected_distance = (intersect.distance * (a - player.a).cos()).max(0.1);
        let wall_height = (block_size as f32 * projection_plane / corrected_distance) as usize;
        let wall_top = height.saturating_sub(wall_height) / 2;
        let wall_bottom = (wall_top + wall_height).min(height);

        let brightness = (220.0 / (1.0 + corrected_distance * 0.01)).clamp(45.0, 200.0) as u8;
        let color = impact_color(intersect.impact, brightness);
        draw_stake(framebuffer, column, wall_top, wall_bottom, height, color);
    }
}

fn impact_color(impact: char, brightness: u8) -> Color {
    match impact {
        '#' => Color::new(brightness / 2, brightness / 2, brightness, 255),
        'f' => Color::new(brightness, brightness / 4, brightness / 4, 255),
        's' => Color::new(brightness / 4, brightness, brightness / 4, 255),
        _ => Color::new(brightness, brightness, brightness, 255),
    }
}

fn draw_stake(
    framebuffer: &mut Framebuffer,
    column: usize,
    wall_top: usize,
    wall_bottom: usize,
    screen_height: usize,
    wall_color: Color,
) {
    framebuffer.set_current_color(Color::new(30, 30, 55, 255));
    for y in 0..wall_top {
        framebuffer.set_pixel(column as u32, y as u32);
    }

    framebuffer.set_current_color(wall_color);
    for y in wall_top..wall_bottom {
        framebuffer.set_pixel(column as u32, y as u32);
    }

    framebuffer.set_current_color(Color::new(55, 45, 40, 255));
    for y in wall_bottom..screen_height {
        framebuffer.set_pixel(column as u32, y as u32);
    }
}
