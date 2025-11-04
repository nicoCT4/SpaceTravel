use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

// Estructura para manejar planetas
struct Planet {
    orbit_radius: f32,
    orbit_speed: f32,
    size: f32,
    color: u32,
    angle: f32,
}

fn start_warp(
    planet_index: usize, 
    planets: &[Planet], 
    target_x: &mut f32, 
    target_y: &mut f32, 
    target_zoom: &mut f32, 
    warping: &mut bool, 
    warp_progress: &mut f32
) {
    if let Some(planet) = planets.get(planet_index) {
        let px = planet.orbit_radius * planet.angle.cos();
        let py = planet.orbit_radius * planet.angle.sin();
        
        *target_x = px;
        *target_y = py;
        *target_zoom = 2.5;
        *warping = true;
        *warp_progress = 0.0;
        
        println!("🚀 Warping to planet {}!", planet_index + 1);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

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
    let mut time = 0.0f32;
    
    // Definir planetas con órbitas
    let mut planets = vec![
        Planet { orbit_radius: 120.0, orbit_speed: 0.02, size: 15.0, color: 0x0080FF, angle: 0.0 }, // Azul
        Planet { orbit_radius: 160.0, orbit_speed: 0.015, size: 12.0, color: 0xFF4000, angle: 1.57 }, // Rojo
        Planet { orbit_radius: 200.0, orbit_speed: 0.01, size: 18.0, color: 0x40FF40, angle: 3.14 }, // Verde (más grande)
        Planet { orbit_radius: 240.0, orbit_speed: 0.008, size: 10.0, color: 0xFFFF80, angle: 4.71 }, // Amarillo claro
    ];
    
    println!("🎮 Controles:");
    println!("  ESC - Salir");
    println!("  SPACE - Pausar/reanudar órbitas");
    println!("  W/S - Zoom in/out");
    println!("  1-4 - Warp a planeta (animado)");
    println!("  Flechas - Rotar cámara");
    
    let mut paused = false;
    let mut camera_zoom = 1.0f32;
    let mut camera_x = 0.0f32;
    let mut camera_y = 0.0f32;
    let mut target_zoom = 1.0f32;
    let mut target_x = 0.0f32;
    let mut target_y = 0.0f32;
    let mut warp_progress = 0.0f32;
    let mut warping = false;
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Manejar controles
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
            println!("Órbitas: {}", if paused { "PAUSADAS" } else { "ACTIVAS" });
        }
        
        // Control de zoom
        if window.is_key_down(Key::W) {
            target_zoom = (target_zoom * 1.02).min(5.0);
        }
        if window.is_key_down(Key::S) {
            target_zoom = (target_zoom / 1.02).max(0.3);
        }
        
        // Control de cámara con flechas
        let cam_speed = 2.0 / camera_zoom;
        if window.is_key_down(Key::Up) {
            target_y -= cam_speed;
        }
        if window.is_key_down(Key::Down) {
            target_y += cam_speed;
        }
        if window.is_key_down(Key::Left) {
            target_x -= cam_speed;
        }
        if window.is_key_down(Key::Right) {
            target_x += cam_speed;
        }
        
        // Sistema de warping a planetas
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) && !warping {
            start_warp(0, &planets, &mut target_x, &mut target_y, &mut target_zoom, &mut warping, &mut warp_progress);
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) && !warping {
            start_warp(1, &planets, &mut target_x, &mut target_y, &mut target_zoom, &mut warping, &mut warp_progress);
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) && !warping {
            start_warp(2, &planets, &mut target_x, &mut target_y, &mut target_zoom, &mut warping, &mut warp_progress);
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) && !warping {
            start_warp(3, &planets, &mut target_x, &mut target_y, &mut target_zoom, &mut warping, &mut warp_progress);
        }
        
        // Actualizar tiempo y órbitas
        if !paused {
            time += 0.016; // ~60 FPS
            for planet in &mut planets {
                planet.angle += planet.orbit_speed;
            }
        }
        
        // Actualizar warp y cámara suave
        if warping {
            warp_progress += 0.05;
            if warp_progress >= 1.0 {
                warping = false;
                warp_progress = 0.0;
                println!("✅ Warp completed!");
            }
        }
        
        // Interpolación suave de cámara
        let smooth_speed = 0.1;
        camera_zoom = lerp(camera_zoom, target_zoom, smooth_speed);
        camera_x = lerp(camera_x, target_x, smooth_speed);
        camera_y = lerp(camera_y, target_y, smooth_speed);
        
        // Limpiar buffer con estrellas
        for (i, pixel) in buffer.iter_mut().enumerate() {
            // Crear estrellas aleatorias
            if (i * 123456789) % 8000 == 0 {
                *pixel = 0xFFFFFF; // Estrella blanca
            } else if (i * 987654321) % 12000 == 0 {
                *pixel = 0x8080FF; // Estrella azul
            } else {
                *pixel = 0x000011; // Fondo azul muy oscuro (espacio)
            }
        }
        
        // Calcular posición del sol con cámara
        let center_x = (width as f32 / 2.0 - camera_x * camera_zoom) as usize;
        let center_y = (height as f32 / 2.0 - camera_y * camera_zoom) as usize;
        let radius = (50.0 * camera_zoom) as f32;
        
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
        
        // Dibujar órbitas (líneas débiles) con cámara
        for planet in &planets {
            let orbit_r = planet.orbit_radius * camera_zoom;
            for angle in (0..360).step_by(3) {
                let rad = (angle as f32).to_radians();
                let ox = center_x as f32 + orbit_r * rad.cos();
                let oy = center_y as f32 + orbit_r * rad.sin();
                
                if ox >= 0.0 && ox < width as f32 && oy >= 0.0 && oy < height as f32 {
                    buffer[oy as usize * width + ox as usize] = 0x404040; // Gris oscuro
                }
            }
        }
        
        // Dibujar planetas en órbita con cámara
        for planet in &planets {
            let world_px = planet.orbit_radius * planet.angle.cos();
            let world_py = planet.orbit_radius * planet.angle.sin();
            let px = center_x as f32 + (world_px - camera_x) * camera_zoom;
            let py = center_y as f32 + (world_py - camera_y) * camera_zoom;
            let planet_size = planet.size * camera_zoom;
            
            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - px;
                    let dy = y as f32 - py;
                    let dist = (dx * dx + dy * dy).sqrt();
                    
                    if dist < planet_size {
                        buffer[y * width + x] = planet.color;
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
