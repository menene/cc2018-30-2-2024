
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // fragment.color * fragment.intensity
    combined_shader(fragment, uniforms)
    // combined_blend_shader(fragment, "normal")
    // combined_blend_shader(fragment, "multiply")
    // combined_blend_shader(fragment, "add")
    // combined_blend_shader(fragment, "subtract")
}

fn static_pattern_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();
  
    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;
  
    Color::new(r, g, b)
}

fn purple_shader(_fragment: &Fragment) -> Color {
    Color::new(128, 0, 128) // Purple color
}

fn circle_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let distance = (x * x + y * y).sqrt();
  
    if distance < 0.25 { // Circle radius
      Color::new(255, 255, 0) // Yellow circle
    } else {
      Color::new(0, 0, 0) // Black (transparent) background
    }
}

fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let time = uniforms.time as f32 * 0.05;
    let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
    let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;
  
    let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
    let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();
  
    let circle_size = 0.1;
    let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
    let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };
  
    let circle_intensity = (circle1 + circle2).min(1.0f32);
  
    Color::new(
      (circle_intensity * 255.0) as u8,
      (circle_intensity * 255.0) as u8,
      (circle_intensity * 255.0) as u8
    )
}

pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = static_pattern_shader(fragment);
    let circle_color = moving_circles_shader(fragment, uniforms);
  
    // Combine shaders: use circle color if it's not black, otherwise use base color
    if !circle_color.is_black() {
      circle_color * fragment.intensity
    } else {
      base_color * fragment.intensity
    }
}

pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
    let base_color = purple_shader(fragment);
    let circle_color = circle_shader(fragment);
  
    let combined_color = match blend_mode {
      "normal" => base_color.blend_normal(&circle_color),
      "multiply" => base_color.blend_multiply(&circle_color),
      "add" => base_color.blend_add(&circle_color),
      "subtract" => base_color.blend_subtract(&circle_color),
      _ => base_color
    };
  
    combined_color * fragment.intensity
}