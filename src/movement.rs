use crate::player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

pub const MOVE_SPEED: f32 = 8.0;
pub const ROTATION_SPEED: f32 = PI / 15.0;

pub fn handle_movement(window: &RaylibHandle, player: &mut Player) {
    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED;
    }

    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED;
    }

    let direction = Vector2::new(player.a.cos(), player.a.sin());

    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        player.pos += direction * MOVE_SPEED;
    }

    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        player.pos -= direction * MOVE_SPEED;
    }
}
