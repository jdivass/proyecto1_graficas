use crate::maze::Maze;
use rand::rng;
use rand::seq::SliceRandom;
use raylib::prelude::Vector2;

pub struct SkullKid {
    pub position: Vector2,
    pub animation_offset: f32,
}

pub fn spawn_skull_kids(maze: &Maze, block_size: usize, count: usize) -> Vec<SkullKid> {
    let mut path_cells = maze
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| (*cell == '.').then_some((x, y)))
        })
        .collect::<Vec<_>>();

    path_cells.shuffle(&mut rng());
    path_cells
        .into_iter()
        .take(count)
        .enumerate()
        .map(|(index, (x, y))| SkullKid {
            position: Vector2::new(
                (x as f32 + 0.5) * block_size as f32,
                (y as f32 + 0.5) * block_size as f32,
            ),
            animation_offset: index as f32 * 0.73 + (x * 7 + y * 11) as f32 * 0.09,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skull_kids_only_spawn_on_open_paths() {
        let maze = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', 's', '.', 'f', '#'],
            vec!['#', '.', '#', '.', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        let skull_kids = spawn_skull_kids(&maze, 40, 10);
        assert_eq!(skull_kids.len(), 3);
        for skull_kid in skull_kids {
            let x = skull_kid.position.x as usize / 40;
            let y = skull_kid.position.y as usize / 40;
            assert_eq!(maze[y][x], '.');
        }
    }
}
