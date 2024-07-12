mod framebuffer;
mod bmp;

use crate::bmp::WriteBmp;
use crate::framebuffer::Framebuffer;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFF0000);
    framebuffer.point(400, 300);
    framebuffer.point(401, 300);
    framebuffer.point(400, 301);
    framebuffer.point(401, 301);

    framebuffer.set_current_color(0x00FF00);
    framebuffer.point(200, 150);
    framebuffer.point(201, 150);
    framebuffer.point(200, 151);
    framebuffer.point(201, 151);

    framebuffer.set_current_color(0x0000FF);
    framebuffer.point(600, 450);
    framebuffer.point(601, 450);
    framebuffer.point(600, 451);
    framebuffer.point(601, 451);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}