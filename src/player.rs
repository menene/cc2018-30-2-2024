
use nalgebra_glm::{Vec2};
use minifb::{Key, Window};
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // angulo de vista
    pub fov: f32, // campo de vista
}

pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;
  
    if window.is_key_down(Key::A) {
      player.a -= ROTATION_SPEED;
    }

    if window.is_key_down(Key::D) {
      player.a += ROTATION_SPEED;
    }

    if window.is_key_down(Key::S) {
      player.pos.x = player.pos.x - MOVE_SPEED * player.a.cos();
      player.pos.y = player.pos.y - MOVE_SPEED * player.a.sin();
    }
    
    if window.is_key_down(Key::W) {
      player.pos.x = player.pos.x + MOVE_SPEED * player.a.cos();
      player.pos.y = player.pos.y + MOVE_SPEED * player.a.sin();
    }
  }