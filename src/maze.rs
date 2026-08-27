#![allow(dead_code)]
use crate::framebuffer::Framebuffer;
use rand::rng;
use rand::seq::SliceRandom;
use raylib::prelude::*;
pub type Maze = Vec<Vec<char>>;

pub fn create_maze(width: usize, height: usize) -> Maze {
    let width = width.max(3);
    let height = height.max(3);

    // The maze algorithm requires odd dimensions.
    let width = if width % 2 == 0 { width - 1 } else { width };

    let height = if height % 2 == 0 { height - 1 } else { height };

    let mut maze = vec![vec!['#'; width]; height];

    maze[1][1] = '.';

    fn carve(maze: &mut Maze, x: usize, y: usize) {
        let mut directions = [(2isize, 0isize), (-2, 0), (0, 2), (0, -2)];

        directions.shuffle(&mut rng());

        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx > 0
                && nx < (maze[0].len() - 1) as isize
                && ny > 0
                && ny < (maze.len() - 1) as isize
            {
                let nx = nx as usize;
                let ny = ny as usize;

                if maze[ny][nx] == '#' {
                    let wall_x = (x + nx) / 2;
                    let wall_y = (y + ny) / 2;

                    maze[wall_y][wall_x] = '.';
                    maze[ny][nx] = '.';

                    carve(maze, nx, ny);
                }
            }
        }
    }

    carve(&mut maze, 1, 1);

    // Mark start and finish.
    maze[1][1] = 's';
    maze[height - 2][width - 2] = 'f';

    maze
}

pub fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    match cell {
        '#' => {
            framebuffer.set_current_color(Color::BLUE);
        }
        's' => {
            framebuffer.set_current_color(Color::GREEN);
        }
        'f' => {
            framebuffer.set_current_color(Color::RED);
        }
        _ => {
            framebuffer.set_current_color(Color::WHITE);
        }
    }

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}
