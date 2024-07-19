use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let dx = (end.x as i32 - start.x as i32).abs();
        let dy = -(end.y as i32 - start.y as i32).abs();

        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };
        
        let mut err = dx + dy;

        let mut current = start.map(|x| x as i32);

        loop {
            self.point(current.x as usize, current.y as usize);

            if current == end.map(|x| x as i32) {
                break;
            }

            let e2 = 2 * err;
            
            if e2 >= dy {
                err += dy;
                current.x += sx;
            }
            
            if e2 <= dx {
                err += dx;
                current.y += sy;
            }
        }
    }
}