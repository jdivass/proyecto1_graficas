use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::YELLOW);

    loop {
        let x = (player.pos.x + d * player.a.cos()) as usize;
        let y = (player.pos.y + d * player.a.sin()) as usize;
        let i = x / block_size;
        let j = y / block_size;

        if j >= maze.len() || i >= maze[j].len() || maze[j][i] == '#' {
            break;
        }

        framebuffer.set_pixel(x as u32, y as u32);
        d += 1.0;
    }
}
