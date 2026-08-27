use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

pub const MOVE_SPEED: f32 = 8.0;
pub const ROTATION_SPEED: f32 = PI / 15.0;
const PLAYER_RADIUS: f32 = 8.0;

pub fn handle_movement(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
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

    // Resolve each axis separately so the player slides along walls at corners.
    let next_x = Vector2::new(player.pos.x + movement.x, player.pos.y);
    if can_stand_at(next_x, maze, block_size) {
        player.pos.x = next_x.x;
    }

    let next_y = Vector2::new(player.pos.x, player.pos.y + movement.y);
    if can_stand_at(next_y, maze, block_size) {
        player.pos.y = next_y.y;
    }
}

fn can_stand_at(position: Vector2, maze: &Maze, block_size: usize) -> bool {
    for offset_x in [-PLAYER_RADIUS, PLAYER_RADIUS] {
        for offset_y in [-PLAYER_RADIUS, PLAYER_RADIUS] {
            let x = position.x + offset_x;
            let y = position.y + offset_y;

            if x < 0.0 || y < 0.0 {
                return false;
            }

            let map_x = x as usize / block_size;
            let map_y = y as usize / block_size;
            if map_y >= maze.len() || map_x >= maze[map_y].len() || maze[map_y][map_x] == '#' {
                return false;
            }
        }
    }

    true
}
