use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{render_maze, Maze};
use crate::player::Player;
use crate::textures::TextureManager;
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
    textures: &TextureManager,
    mode: RenderMode,
) {
    match mode {
        RenderMode::TwoD => render_2d(framebuffer, maze, player, block_size),
        RenderMode::ThreeD => render_3d(framebuffer, maze, player, block_size, textures),
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

fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let projection_plane = (width as f32 / 2.0) / (player.fov / 2.0).tan();

    render_surfaces(
        framebuffer,
        textures,
        maze,
        player,
        block_size,
        projection_plane,
    );

    for column in 0..width {
        let ray_progress = column as f32 / (width - 1) as f32;
        let a = player.a - player.fov / 2.0 + player.fov * ray_progress;
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);

        // Corregir ojo de pescado
        let corrected_distance = (intersect.distance * (a - player.a).cos()).max(0.1);
        let wall_height = block_size as f32 * projection_plane / corrected_distance;
        let projected_top = (height as f32 - wall_height) / 2.0;
        let wall_top = projected_top.max(0.0) as usize;
        let wall_bottom = (projected_top + wall_height).min(height as f32) as usize;
        let brightness = (220.0 / (1.0 + corrected_distance * 0.01)).clamp(45.0, 200.0) as u8;
        draw_stake(
            framebuffer,
            textures,
            column,
            wall_top,
            wall_bottom,
            projected_top,
            wall_height,
            intersect.map_x,
            intersect.map_y,
            intersect.wall_side,
            intersect.texture_x,
            brightness,
            intersect.impact,
        );
    }
}

fn render_surfaces(
    framebuffer: &mut Framebuffer,
    textures: &TextureManager,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    projection_plane: f32,
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let horizon = height / 2;
    let maze_width = (maze[0].len() * block_size) as f32;
    let maze_height = (maze.len() * block_size) as f32;
    let camera_height = block_size as f32 / 2.0;

    for column in 0..width {
        let ray_progress = column as f32 / (width - 1) as f32;
        let a = player.a - player.fov / 2.0 + player.fov * ray_progress;

        // One panoramic sky image wraps around the complete viewing angle.
        for y in 0..horizon {
            let v = y as f32 / horizon as f32;
            framebuffer.set_pixel_color(column as u32, y as u32, textures.sky_pixel(a, v));
        }

        // Perspective floor casting, with one image stretched across the whole maze.
        for y in horizon..height {
            let distance_from_horizon = (y - horizon) as f32 + 0.5;
            let perpendicular_distance = camera_height * projection_plane / distance_from_horizon;
            let ray_distance = perpendicular_distance / (a - player.a).cos().max(0.001);
            let world_x = player.pos.x + ray_distance * a.cos();
            let world_y = player.pos.y + ray_distance * a.sin();
            let color = textures.floor_pixel(world_x, world_y, maze_width, maze_height);
            let brightness =
                (200.0 / (1.0 + perpendicular_distance * 0.002)).clamp(75.0, 200.0) as u8;
            framebuffer.set_pixel_color(column as u32, y as u32, shade(color, brightness));
        }
    }
}

fn draw_stake(
    framebuffer: &mut Framebuffer,
    textures: &TextureManager,
    column: usize,
    wall_top: usize,
    wall_bottom: usize,
    projected_top: f32,
    wall_height: f32,
    map_x: usize,
    map_y: usize,
    wall_side: usize,
    texture_u: f32,
    brightness: u8,
    impact: char,
) {
    let wall_texture = textures.wall_texture(map_x, map_y, wall_side);
    for y in wall_top..wall_bottom {
        let texture_v = (y as f32 - projected_top) / wall_height;
        let color = if impact == '#' {
            wall_texture.sample(texture_u, texture_v)
        } else {
            Color::WHITE
        };
        framebuffer.set_pixel_color(column as u32, y as u32, shade(color, brightness));
    }
}

fn shade(color: Color, brightness: u8) -> Color {
    let factor = brightness as f32 / 200.0;
    Color::new(
        (color.r as f32 * factor).min(255.0) as u8,
        (color.g as f32 * factor).min(255.0) as u8,
        (color.b as f32 * factor).min(255.0) as u8,
        color.a,
    )
}
