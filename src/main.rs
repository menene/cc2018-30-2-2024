
use minifb::{ Window, WindowOptions, Key };
use std::time::Duration;

mod framebuffer;

use crate::framebuffer::Framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "UVG Graphixs",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x000000);

    let mut x = 0 as i32;
    let mut x2 = 1 as i32;
    let mut x3 = 2 as i32;

    let mut speed = 1 as i32;
    let mut y = 40 as i32;
    let mut speedy = 1 as i32;
    let frame_delay = Duration::from_millis(16);

    framebuffer.set_current_color(0xFFFFFF);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if x3 as usize >= framebuffer_width {
            framebuffer.set_current_color(0xF17102);
            speed = -1;
        }

        if x as usize <= 0 {
            framebuffer.set_current_color(0x00FF00);
            speed = 1;
        }

        x += speed;
        x2 += speed;
        x3 += speed;

        if y as usize >= framebuffer_height {
            speedy = -1;
        }

        if y as usize <= 0 {
            speedy = 1;
        }

        y += speedy;

        framebuffer.clear();
        framebuffer.point(x as usize, y as usize);
        framebuffer.point(x2 as usize, y as usize);
        framebuffer.point(x3 as usize, y as usize);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}   
