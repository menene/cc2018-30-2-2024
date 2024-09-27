
use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
// use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod color;
mod fragment;
mod shader;
mod obj;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shader::vertex_shader;
use obj::Obj;

pub struct Uniforms {
    model_matrix: Mat4
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // vertex shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // primitive assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // rasterization
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // fragment processing
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment.color.to_hex();

            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn create_model_matrix(translation: Vec3, scale: f32) -> Mat4 {
    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    transform_matrix
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "3D Objects",
        window_width,
        window_height,
        WindowOptions::default()
    )
    .unwrap();

    framebuffer.set_background_color(0x333355);

    let mut translation = Vec3::new(300.0, 200.0, 0.0);
    let mut scale = 100.0f32;

    let obj = Obj::load("assets/models/model.obj").expect("Error al cargar modelo");
    let vertex_array = obj.get_vertex_array();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::Right) {
            translation.x += 10.0;
        }

        if window.is_key_down(Key::Left) {
            translation.x -= 10.0;
        }

        if window.is_key_down(Key::Up) {
            translation.y -= 10.0;
        }

        if window.is_key_down(Key::Down) {
            translation.y += 10.0;
        }

        if window.is_key_down(Key::S) {
            scale += 0.9;
        }

        if window.is_key_down(Key::A) {
            scale -= 0.9;
        }

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale);
        let uniforms = Uniforms { model_matrix };

        framebuffer.set_current_color(0xFFDDDD);

        render(&mut framebuffer, &uniforms, &vertex_array);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}