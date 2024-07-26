use crate::framebuffer::Framebuffer;
use crate::line::Line;
use nalgebra_glm::Vec3;

pub trait Polygon {
    fn polygon(&mut self, points: &Vec<Vec3>);
    fn filled_polygon(&mut self, points: &Vec<Vec3>);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &Vec<Vec3>) {
        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];
            self.line(start, end);
        }
    }

    fn filled_polygon(&mut self, points: &Vec<Vec3>) {
        // si no hay puntos regresamos
        if points.is_empty() {
            return;
        }

        // definir los limites del poligono en el eje y
        let min_y = points.iter()
            .map(|p| p.y)
            .fold(f32::INFINITY, f32::min) as usize; // encientra el mas pequenio

        let max_y = points.iter()
            .map(|p| p.y)
            .fold(f32::NEG_INFINITY, f32::max) as usize; // encuentra el mas grande

        // iterar por cada scanline
        //  .. = significa que es inclusivo
        for y in min_y..=max_y {
            let mut intersections = vec![];

            // encontrar intersecciones en el eje x
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                // solo se determina la coordenada x
                if (p1.y as usize) <= y && (p2.y as usize) > y || (p2.y as usize) <= y && (p1.y as usize) > y {
                    let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x);
                }
            }

            // ordenar las intersecciones (x)
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // llenado de los puntos del lado correcto de 
            // las intersecciones
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i] as usize;
                    let x2 = intersections[i + 1] as usize;
                    for x in x1..=x2 {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}