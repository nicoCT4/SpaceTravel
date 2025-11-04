use minifb::{Key, Window, WindowOptions};

fn main() {
    println!("Intentando crear ventana...");
    
    let mut window = Window::new(
        "Test Window",
        640,
        480,
        WindowOptions::default(),
    );
    
    match window {
        Ok(mut win) => {
            println!("Ventana creada exitosamente!");
            
            let mut buffer: Vec<u32> = vec![0; 640 * 480];
            
            // Llenar con un patrón de colores
            for i in 0..buffer.len() {
                buffer[i] = 0xFF0000; // Rojo
            }
            
            while win.is_open() && !win.is_key_down(Key::Escape) {
                win.update_with_buffer(&buffer, 640, 480).unwrap();
            }
            
            println!("Ventana cerrada normalmente");
        }
        Err(e) => {
            println!("Error creando ventana: {:?}", e);
        }
    }
}
