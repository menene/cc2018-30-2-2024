extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    let v1 = Vec3::new(100.0, 100.0, 0.0);
    let v2 = Vec3::new(700.0, 500.0, 0.0);
    framebuffer.line(v1, v2);


    // let points = vec![
    //     Vec3::new(10.0, 10.0, 0.0),
    //     Vec3::new(80.0, 10.0, 0.0),
    //     Vec3::new(50.0, 90.0, 0.0),
    // ];
    let points = vec![
        Vec3::new(165.0, 380.0, 0.0), 
        Vec3::new(185.0, 360.0, 0.0), 
        Vec3::new(180.0, 330.0, 0.0), 
        Vec3::new(207.0, 345.0, 0.0), 
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0), 
        Vec3::new(250.0, 380.0, 0.0), 
        Vec3::new(220.0, 385.0, 0.0), 
        Vec3::new(205.0, 410.0, 0.0), 
        Vec3::new(193.0, 383.0, 0.0)
    ];
    framebuffer.filled_polygon(&points);

    let _ = framebuffer.render_buffer("output.bmp");
}   
