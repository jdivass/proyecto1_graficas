use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut distance = 0.0;
    let max_distance = (maze.len().max(maze[0].len()) * block_size) as f32 * 2.0;

    loop {
        let x = player.pos.x + distance * a.cos();
        let y = player.pos.y + distance * a.sin();

        if x < 0.0 || y < 0.0 {
            return Intersect {
                distance,
                impact: '#',
            };
        }

        let i = x as usize / block_size;
        let j = y as usize / block_size;

        if j >= maze.len() || i >= maze[j].len() {
            return Intersect {
                distance,
                impact: '#',
            };
        }

        let cell = maze[j][i];
        if cell == '#' {
            return Intersect {
                distance,
                impact: cell,
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        distance += 1.0;
        if distance >= max_distance {
            return Intersect {
                distance,
                impact: ' ',
            };
        }
    }
}
