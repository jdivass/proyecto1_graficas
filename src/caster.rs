use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub map_x: usize,
    pub map_y: usize,
    pub wall_side: usize,
    pub texture_x: f32,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let size = block_size as f32;
    let position_x = player.pos.x / size;
    let position_y = player.pos.y / size;
    let ray_x = a.cos();
    let ray_y = a.sin();
    let mut map_x = position_x.floor() as i32;
    let mut map_y = position_y.floor() as i32;

    let delta_x = if ray_x.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        (1.0 / ray_x).abs()
    };
    let delta_y = if ray_y.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        (1.0 / ray_y).abs()
    };

    let (step_x, mut side_x) = if ray_x < 0.0 {
        (-1, (position_x - map_x as f32) * delta_x)
    } else {
        (1, (map_x as f32 + 1.0 - position_x) * delta_x)
    };
    let (step_y, mut side_y) = if ray_y < 0.0 {
        (-1, (position_y - map_y as f32) * delta_y)
    } else {
        (1, (map_y as f32 + 1.0 - position_y) * delta_y)
    };

    loop {
        let (distance_in_cells, wall_side) = if side_x < side_y {
            let distance = side_x;
            side_x += delta_x;
            map_x += step_x;
            (distance, 0)
        } else {
            let distance = side_y;
            side_y += delta_y;
            map_y += step_y;
            (distance, 1)
        };

        if map_x < 0
            || map_y < 0
            || map_y as usize >= maze.len()
            || map_x as usize >= maze[map_y as usize].len()
        {
            return make_intersect(
                framebuffer,
                player,
                ray_x,
                ray_y,
                distance_in_cells * size,
                '#',
                map_x.max(0) as usize,
                map_y.max(0) as usize,
                wall_side,
                size,
                draw_line,
            );
        }

        let impact = maze[map_y as usize][map_x as usize];
        if impact == '#' {
            return make_intersect(
                framebuffer,
                player,
                ray_x,
                ray_y,
                distance_in_cells * size,
                impact,
                map_x as usize,
                map_y as usize,
                wall_side,
                size,
                draw_line,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn make_intersect(
    framebuffer: &mut Framebuffer,
    player: &Player,
    ray_x: f32,
    ray_y: f32,
    distance: f32,
    impact: char,
    map_x: usize,
    map_y: usize,
    wall_side: usize,
    block_size: f32,
    draw_line: bool,
) -> Intersect {
    let hit = Vector2::new(
        player.pos.x + ray_x * distance,
        player.pos.y + ray_y * distance,
    );
    if draw_line {
        line(framebuffer, player.pos, hit);
    }

    let wall_hit = if wall_side == 0 {
        hit.y / block_size
    } else {
        hit.x / block_size
    };
    let mut texture_x = wall_hit.rem_euclid(1.0);
    if (wall_side == 0 && ray_x > 0.0) || (wall_side == 1 && ray_y < 0.0) {
        texture_x = 1.0 - texture_x;
    }

    Intersect {
        distance,
        impact,
        map_x,
        map_y,
        wall_side,
        texture_x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dda_finds_the_next_wall_and_its_texture_coordinate() {
        let maze = vec![
            vec!['#', '#', '#'],
            vec!['#', '.', '#'],
            vec!['#', '#', '#'],
        ];
        let player = Player {
            pos: Vector2::new(60.0, 60.0),
            a: 0.0,
            fov: std::f32::consts::PI / 3.0,
        };
        let mut framebuffer = Framebuffer::new(120, 120, Color::BLACK);

        let hit = cast_ray(&mut framebuffer, &maze, &player, 0.0, 40, false);

        assert!((hit.distance - 20.0).abs() < 0.001);
        assert_eq!((hit.map_x, hit.map_y), (2, 1));
        assert_eq!(hit.wall_side, 0);
        assert!((hit.texture_x - 0.5).abs() < 0.001);
    }
}
