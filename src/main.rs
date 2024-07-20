use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

mod bmp;

fn initialize_pattern(framebuffer: &mut Framebuffer) {
    let pattern = vec![
        (1.0, 0.0), (2.0, 1.0), (0.0, 2.0), (1.0, 2.0), (2.0, 2.0), // Glider pattern
    ];

    framebuffer.set_current_color(0xFFFFFF); // White color for alive cells
    for &(x, y) in &pattern {
        framebuffer.point(x, y);
    }
}

fn count_alive_neighbors(framebuffer: &Framebuffer, x: isize, y: isize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some((r, g, b)) = framebuffer.get_pixel(x + dx, y + dy) {
                if r == 255 && g == 255 && b == 255 { // Alive cell
                    count += 1;
                }
            }
        }
    }
    count
}

fn step(framebuffer: &Framebuffer) -> Vec<(f32, f32, bool)> {
    let mut changes = Vec::new();
    for y in 0..framebuffer.height() as isize {
        for x in 0..framebuffer.width() as isize {
            let alive_neighbors = count_alive_neighbors(framebuffer, x, y);
            if let Some((r, g, b)) = framebuffer.get_pixel(x, y) {
                let is_alive = r == 255 && g == 255 && b == 255;
                if is_alive && (alive_neighbors < 2 || alive_neighbors > 3) {
                    changes.push((x as f32, y as f32, false)); // Cell dies
                } else if !is_alive && alive_neighbors == 3 {
                    changes.push((x as f32, y as f32, true)); // Cell becomes alive
                }
            }
        }
    }
    changes
}

fn apply_changes(framebuffer: &mut Framebuffer, changes: Vec<(f32, f32, bool)>) {
    for (x, y, alive) in changes {
        if alive {
            framebuffer.set_current_color(0xFFFFFF); // White for alive cells
        } else {
            framebuffer.set_current_color(0x000000); // Black for dead cells
        }
        framebuffer.point(x, y);
    }
}

fn main() {
    let window_width = 600;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    initialize_pattern(&mut framebuffer);

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Apply Game of Life rules
        let changes = step(&framebuffer);
        apply_changes(&mut framebuffer, changes);

        // Convert framebuffer data to u32 buffer
        let buffer = framebuffer.to_u32_buffer();

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
