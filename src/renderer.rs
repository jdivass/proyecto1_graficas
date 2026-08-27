use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::line::line;
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
        RenderMode::ThreeD => {
            render_3d(framebuffer, maze, player, block_size, textures);
            render_minimap(framebuffer, maze, player, block_size);
        }
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

fn render_minimap(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    const MARGIN: usize = 16;
    const PADDING: usize = 4;

    let columns = maze[0].len();
    let rows = maze.len();
    let max_width = framebuffer.width as f32 * 0.24;
    let max_height = framebuffer.height as f32 * 0.28;
    let cell_size = (max_width / columns as f32)
        .min(max_height / rows as f32)
        .floor()
        .max(3.0) as usize;
    let map_width = columns * cell_size;
    let map_height = rows * cell_size;
    let origin_x = MARGIN + PADDING;
    let origin_y = MARGIN + PADDING;

    fill_rectangle(
        framebuffer,
        MARGIN,
        MARGIN,
        map_width + PADDING * 2,
        map_height + PADDING * 2,
        Color::new(8, 8, 12, 255),
    );

    for (map_y, row) in maze.iter().enumerate() {
        for (map_x, cell) in row.iter().enumerate() {
            let color = match cell {
                '#' => Color::new(125, 122, 122, 255),
                's' => Color::new(179, 191, 23, 255),
                'f' => Color::new(153, 0, 0, 255),
                _ => Color::new(215, 205, 175, 255),
            };
            fill_rectangle(
                framebuffer,
                origin_x + map_x * cell_size,
                origin_y + map_y * cell_size,
                cell_size,
                cell_size,
                color,
            );
        }
    }

    let scale = cell_size as f32 / block_size as f32;
    let player_position = Vector2::new(
        origin_x as f32 + player.pos.x * scale,
        origin_y as f32 + player.pos.y * scale,
    );

    framebuffer.set_current_color(Color::YELLOW);
    let minimap_rays = 9;
    for ray in 0..minimap_rays {
        let ray_progress = ray as f32 / (minimap_rays - 1) as f32;
        let a = player.a - player.fov / 2.0 + player.fov * ray_progress;
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);
        let ray_end = Vector2::new(
            origin_x as f32 + (player.pos.x + a.cos() * intersect.distance) * scale,
            origin_y as f32 + (player.pos.y + a.sin() * intersect.distance) * scale,
        );
        let ray_end = Vector2::new(
            ray_end
                .x
                .clamp(origin_x as f32, (origin_x + map_width - 1) as f32),
            ray_end
                .y
                .clamp(origin_y as f32, (origin_y + map_height - 1) as f32),
        );
        line(framebuffer, player_position, ray_end);
    }

    let marker_radius = (cell_size / 4).max(2) as i32;
    fill_rectangle(
        framebuffer,
        (player_position.x as i32 - marker_radius).max(0) as usize,
        (player_position.y as i32 - marker_radius).max(0) as usize,
        (marker_radius * 2 + 1) as usize,
        (marker_radius * 2 + 1) as usize,
        Color::MAGENTA,
    );
}

fn fill_rectangle(
    framebuffer: &mut Framebuffer,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: Color,
) {
    for pixel_y in y..y + height {
        for pixel_x in x..x + width {
            framebuffer.set_pixel_color(pixel_x as u32, pixel_y as u32, color);
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
        let wall_top = (projected_top.floor() as isize - 1).clamp(0, height as isize) as usize;
        let wall_bottom = ((projected_top + wall_height).ceil() as usize + 1).min(height);
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

        for y in 0..horizon {
            let v = y as f32 / horizon as f32;
            framebuffer.set_pixel_color(column as u32, y as u32, textures.sky_pixel(a, v));
        }

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
