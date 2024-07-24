use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;
mod bmp;

fn initialize_pattern(framebuffer: &mut Framebuffer) {
    // Definir patrones de osciladores
    let blinker = vec![(1, 0), (1, 1), (1, 2)];
    let toad = vec![(1, 2), (1, 3), (1, 4), (2, 1), (2, 2), (2, 3)];
    let beacon = vec![(1, 1), (1, 2), (2, 1), (2, 2), (3, 3), (3, 4), (4, 3), (4, 4)];
    let pulsar = vec![
        // Primera línea horizontal superior
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        // Segunda línea horizontal superior
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        // Tercera línea horizontal superior
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        // Primera línea horizontal inferior
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        // Segunda línea horizontal inferior
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        // Tercera línea horizontal inferior
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
    ];

    // Definir patrones de spaceships
    let glider = vec![(1, 2), (2, 1), (0, 0), (1, 0), (2, 0)];
    let lwss = vec![
        (1, 3), (4, 3),
        (0, 2), (0, 1), (4, 1),
        (0, 0), (1, 0), (2, 0), (3, 0)
    ];
    let mwss = vec![
        (1, 3), (3, 4), (5, 3),
        (0, 2), (0, 1), (5, 1),
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0)
    ];
    let hwss = vec![
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
        (0, 2), (0, 3), (6, 3),
        (1, 1), (3, 0), (4, 0), (6, 1)
    ];

    // Still lifes
    let tub = vec![(1, 0), (0, 1), (2, 1), (1, 2)];

    // Posicionar y añadir los patrones al framebuffer
    let patterns = vec![
        pulsar, 
        glider.clone(), mwss.clone(), hwss.clone(), lwss.clone(),  blinker.clone(),
        toad.clone(), blinker.clone(), beacon.clone(), tub.clone(), blinker.clone(),
        toad.clone(), glider.clone(), mwss.clone(), toad.clone(), blinker.clone(),
        toad.clone(), blinker.clone(), tub.clone(), blinker.clone(),
        toad.clone(), blinker.clone(), beacon.clone(), tub.clone(), blinker.clone(),
        glider.clone(), beacon.clone(), hwss.clone(), lwss.clone(),  blinker.clone(),
        toad.clone(), blinker.clone(), beacon.clone(), tub.clone(),
        glider.clone(), mwss.clone(), hwss.clone(), lwss.clone(), 
        glider, mwss, hwss, lwss
    ];

    let offsets = vec![
        (5, 80), 
        (0, 0), (20, 5), (40, 0), (60, 5), (80, 0), (95, 5),
        (5, 25), (25, 20), (45, 25), (65, 20), (85, 25), 
        (0, 40), (20, 45), (40, 40), (60, 45), (80, 40), (95, 45),
        (3, 65), (23, 60), (43, 65), (63, 60), (83, 65), 
        (0, 80), (20, 85), (40, 80), (60, 85), (80, 80), (95, 85),
        (6, 95), (26, 95), (46, 95), (66, 95), (86, 95), 
        (74, 35), (53, 50), (33, 77), (8, 15),
        (55, 15), (18, 36), (76, 32), (81, 91)
    ];

    framebuffer.set_current_color(0xebdc7f); // Color de las células vivas
    for (pattern, &(offset_x, offset_y)) in patterns.iter().zip(offsets.iter()) {
        for &(x, y) in pattern {
            if x + offset_x < framebuffer.width() as isize && y + offset_y < framebuffer.height() as isize {
                framebuffer.point((x + offset_x) as f32, (y + offset_y) as f32);
            }
        }
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
                if r == 235 && g == 220 && b == 127 { // Color de las células vivas
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
                let is_alive = r == 235 && g == 220 && b == 127;
                if is_alive && (alive_neighbors < 2 || alive_neighbors > 3) {
                    changes.push((x as f32, y as f32, false)); // La célula muere
                } else if !is_alive && alive_neighbors == 3 {
                    changes.push((x as f32, y as f32, true)); // La célula revive
                }
            }
        }
    }
    changes
}

fn apply_changes(framebuffer: &mut Framebuffer, changes: Vec<(f32, f32, bool)>) {
    for (x, y, alive) in changes {
        if alive {
            framebuffer.set_current_color(0xebdc7f); // Color de las células vivas
        } else {
            framebuffer.set_current_color(0x0c0b38); // Color de las células muertas
        }
        framebuffer.point(x, y);
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(0x0c0b38);
    let mut window = Window::new(
        "Rust Graphics - Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    initialize_pattern(&mut framebuffer);

    while window.is_open() {
        // Escuchar inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Aplicar las reglas de Game of Life
        let changes = step(&framebuffer);
        apply_changes(&mut framebuffer, changes);

        // Convertir los datos del framebuffer a un buffer de u32
        let buffer = framebuffer.to_u32_buffer();

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
