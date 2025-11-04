use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

fn main() {
    println!("🚀 Iniciando simulador del sistema solar...");
    
    let width = 800;
    let height = 600;
    
    let mut window = match Window::new(
        "🌟 Sistema Solar - Space Renderer",
        width,
        height,
        WindowOptions::default(),
    ) {
        Ok(win) => {
            println!("✅ Ventana creada exitosamente!");
            win
        },
        Err(e) => {
            println!("❌ Error creando ventana: {:?}", e);
            return;
        }
    };
    
    // Buffer de píxeles
    let mut buffer: Vec<u32> = vec![0; width * height];
    
    // Variables para animación
    let mut frame = 0;
    
    println!("🎮 Controles:");
    println!("  ESC - Salir");
    println!("  Cualquier tecla - Cambiar color");
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Limpiar buffer
        for pixel in buffer.iter_mut() {
            *pixel = 0x000011; // Fondo azul muy oscuro (espacio)
        }
        
        // Dibujar un "sol" simple en el centro
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = 50;
        
        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                
                if dist < radius as f32 {
                    let color = if frame % 60 < 30 {
                        0xFFFF00 // Amarillo
                    } else {
                        0xFF8000 // Naranja
                    };
                    buffer[y * width + x] = color;
                }
            }
        }
        
        // Dibujar algunos "planetas"
        let planets = [
            (center_x + 150, center_y, 0x0080FF, 15), // Azul
            (center_x - 120, center_y + 50, 0xFF4000, 12), // Rojo
            (center_x + 80, center_y - 100, 0x40FF40, 10), // Verde
        ];
        
        for (px, py, color, size) in planets.iter() {
            for y in 0..height {
                for x in 0..width {
                    let dx = x as i32 - *px as i32;
                    let dy = y as i32 - *py as i32;
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                    
                    if dist < *size as f32 {
                        buffer[y * width + x] = *color;
                    }
                }
            }
        }
        
        // Actualizar ventana
        match window.update_with_buffer(&buffer, width, height) {
            Ok(_) => {},
            Err(e) => {
                println!("❌ Error actualizando ventana: {:?}", e);
                break;
            }
        }
        
        frame += 1;
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
    
    println!("👋 Simulador cerrado");
}
