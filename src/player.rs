
use nalgebra_glm::{Vec2};

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // angulo de vista
    pub fov: f32, // campo de vista
}