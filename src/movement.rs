use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;
use std::f32::consts::{PI, TAU};

pub const MOVE_SPEED: f32 = 8.0;
pub const ROTATION_SPEED: f32 = PI / 15.0;
pub const MOUSE_SENSITIVITY: f32 = 0.0035;
const PLAYER_RADIUS: f32 = 8.0;

pub struct MovementResult {
    x_wall: Option<(usize, usize)>,
    y_wall: Option<(usize, usize)>,
}

impl MovementResult {
    pub fn hit_wall(&self, wall: (usize, usize)) -> bool {
        self.x_wall == Some(wall) || self.y_wall == Some(wall)
    }
}

pub fn handle_movement(
    window: &RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
) -> MovementResult {
    let mouse_movement = window.get_mouse_delta();
    player.a = (player.a + mouse_movement.x * MOUSE_SENSITIVITY).rem_euclid(TAU);

    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED;
    }

    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED;
    }

    let direction = Vector2::new(player.a.cos(), player.a.sin());
    let mut movement = 0.0;

    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        movement += MOVE_SPEED;
    }

    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        movement -= MOVE_SPEED;
    }

    let movement = direction * movement;

    let next_x = Vector2::new(player.pos.x + movement.x, player.pos.y);
    let x_wall = match collision_at(next_x, maze, block_size) {
        Collision::None => {
            player.pos.x = next_x.x;
            None
        }
        Collision::Wall(wall) => Some(wall),
        Collision::Boundary => None,
    };

    let next_y = Vector2::new(player.pos.x, player.pos.y + movement.y);
    let y_wall = match collision_at(next_y, maze, block_size) {
        Collision::None => {
            player.pos.y = next_y.y;
            None
        }
        Collision::Wall(wall) => Some(wall),
        Collision::Boundary => None,
    };

    MovementResult { x_wall, y_wall }
}

enum Collision {
    None,
    Wall((usize, usize)),
    Boundary,
}

fn collision_at(position: Vector2, maze: &Maze, block_size: usize) -> Collision {
    for offset_x in [-PLAYER_RADIUS, PLAYER_RADIUS] {
        for offset_y in [-PLAYER_RADIUS, PLAYER_RADIUS] {
            let x = position.x + offset_x;
            let y = position.y + offset_y;

            if x < 0.0 || y < 0.0 {
                return Collision::Boundary;
            }

            let map_x = x as usize / block_size;
            let map_y = y as usize / block_size;
            if map_y >= maze.len() || map_x >= maze[map_y].len() {
                return Collision::Boundary;
            }
            if maze[map_y][map_x] == '#' {
                return Collision::Wall((map_x, map_y));
            }
        }
    }

    Collision::None
}
