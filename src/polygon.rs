use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, points: &[(usize, usize)]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            self.line(x0, y0, x1, y1);
        }
    }
}