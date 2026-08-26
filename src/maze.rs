#![allow(dead_code)]
use rand::seq::SliceRandom;
use rand::{rng};

type Maze = Vec<Vec<char>>;

fn create_maze(width: usize, height: usize) -> Maze {
    let width = width.max(3);
    let height = height.max(3);

    // The maze algorithm requires odd dimensions.
    let width = if width % 2 == 0 {
        width - 1
    } else {
        width
    };

    let height = if height % 2 == 0 {
        height - 1
    } else {
        height
    };

    let mut maze = vec![vec!['#'; width]; height];

    maze[1][1] = '.';

    fn carve(maze: &mut Maze, x: usize, y: usize) {
        let mut directions = [
            (2isize, 0isize),
            (-2, 0),
            (0, 2),
            (0, -2),
        ];

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

