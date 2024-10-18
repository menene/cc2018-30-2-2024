
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
    fragment.color * fragment.intensity
    // stripe_shader(fragment, uniforms)
    // transformed_stripe_shader(fragment, uniforms)
    // lerp_stripe_shader(fragment, uniforms)
    // wave_color_shader(fragment, uniforms)
}

pub fn stripe_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let y = fragment.position.y as usize;
  
    // Define stripe colors
    let colors = [
      Color::new(255, 0, 0),   // Red
      Color::new(0, 255, 0),   // Green
      Color::new(0, 0, 255),   // Blue
      Color::new(255, 255, 0), // Yellow
    ];
  
    // Define stripe width
    let stripe_width = 20;
  
    // Calculate which stripe this fragment belongs to
    let stripe_index = (y / stripe_width) % colors.len();
  
    // Return the color for this stripe
    colors[stripe_index] * fragment.intensity
}
  
pub fn transformed_stripe_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let stripe_coord = fragment.vertex_position.y;
  
    // Define stripe colors
    let colors = [
      Color::new(255, 0, 0),   // Red
      Color::new(0, 255, 0),   // Green
      Color::new(0, 0, 255),   // Blue
      Color::new(255, 255, 0), // Yellow
    ];
  
    // Define stripe width
    let stripe_width = 0.1;
  
    // Calculate which stripe this fragment belongs to
    let stripe_index = ((stripe_coord / stripe_width).abs() as usize) % colors.len();
  
    // Return the color for this stripe
    colors[stripe_index] * fragment.intensity
}
  
pub fn lerp_stripe_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define stripe colors
    let colors = [
      Color::new(255, 0, 0),   // Red
      Color::new(0, 255, 0),   // Green
      Color::new(0, 0, 255),   // Blue
      Color::new(255, 255, 0), // Yellow
    ];
  
    // Define stripe width
    let stripe_width = 0.1;
  
    // Use the y-coordinate of the transformed position for stripe calculation
    let stripe_coord = fragment.vertex_position.y;
  
    // Calculate which stripe this fragment belongs to
    let stripe_float = (stripe_coord / stripe_width).abs();
    let stripe_index = (stripe_float as usize) % colors.len();
    let next_index = (stripe_index + 1) % colors.len();
  
    // Calculate interpolation factor
    let t = stripe_float.fract();
  
    // Interpolate between current and next color
    colors[stripe_index].lerp(&colors[next_index], t) * fragment.intensity
}
  
pub fn wave_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color1 = Color::new(255, 0, 0);   // Red
    let color2 = Color::new(0, 255, 0);   // Green
    let color3 = Color::new(0, 0, 255);   // Blue
  
    // Use both x and y coordinates for more interesting patterns
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let frequency = 10.0; // Adjust this value to control overall frequency
  
    // Create three overlapping sine waves
    let wave1 = (x * 7.0 * frequency + y * 5.0 * frequency).sin() * 0.5 + 0.5;
    let wave2 = (x * 5.0 * frequency - y * 8.0 * frequency + PI / 3.0).sin() * 0.5 + 0.5;
    let wave3 = (y * 6.0 * frequency + x * 4.0 * frequency + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;
  
    // Interpolate between colors based on the wave values
    let mut final_color = color1.lerp(&color2, wave1);
    final_color = final_color.lerp(&color3, wave2);
    final_color = final_color.lerp(&color1, wave3);
  
    final_color * fragment.intensity
}