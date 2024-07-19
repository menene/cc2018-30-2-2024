extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod line;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    let v1 = Vec3::new(100.0, 100.0, 0.0);
    let v2 = Vec3::new(700.0, 500.0, 0.0);

    framebuffer.line(v1, v2);

    let _ = framebuffer.render_buffer("output.bmp");
}   
