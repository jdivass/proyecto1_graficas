use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;

pub struct RayHit {
    pub distance: f32,
    pub position: Vector2,
}

pub fn cast_ray(maze: &Maze, player: &Player, a: f32, block_size: usize) -> RayHit {
    let mut distance = 0.0;
    let max_distance = (maze.len().max(maze[0].len()) * block_size) as f32 * 2.0;

    loop {
        let x = player.pos.x + distance * a.cos();
        let y = player.pos.y + distance * a.sin();

        if x < 0.0 || y < 0.0 {
            return RayHit {
                distance,
                position: Vector2::new(x.max(0.0), y.max(0.0)),
            };
        }

        let i = x as usize / block_size;
        let j = y as usize / block_size;

        if j >= maze.len() || i >= maze[j].len() || maze[j][i] == '#' {
            return RayHit {
                distance,
                position: Vector2::new(x, y),
            };
        }

        distance += 1.0;

        if distance >= max_distance {
            return RayHit {
                distance,
                position: Vector2::new(x, y),
            };
        }
    }
}
