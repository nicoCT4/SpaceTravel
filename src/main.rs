use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod fragment;
mod color;
mod shaders;
mod camera;
mod celestial_body;
mod obj_loader;
mod spaceship;
mod orbit;
mod sphere;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use camera::{Camera, CameraMode};
use obj_loader::Model;
use shaders::{vertex_shader, fragment_shader, Uniforms};
use celestial_body::{CelestialBody, ShaderType};
use spaceship::Spaceship;
use orbit::OrbitRing;
use sphere::create_sphere;


pub struct RenderContext {
    framebuffer: Framebuffer,
    camera: Camera,
    bodies: Vec<CelestialBody>,
    orbits: Vec<OrbitRing>,
    spaceship: Spaceship,
    current_body_index: usize,
    time: f32,
    warp_animation: Option<WarpAnimation>,
    skybox: CelestialBody,
}

struct WarpAnimation {
    from: Vec3,
    to: Vec3,
    progress: f32,
    duration: f32,
}

impl RenderContext {
    fn new(width: usize, height: usize) -> Self {
        let mut bodies = Vec::new();
        let mut orbits = Vec::new();
        
        // Sol en el centro
        bodies.push(
            CelestialBody::new(
                Vec3::new(0.0, 0.0, 0.0),
                1.5,
                ShaderType::Sun,
            )
            .with_rotation_speed(Vec3::new(0.0, 0.1, 0.0))
        );
        
        // Planeta rocoso (tipo Tierra/Marte)
        bodies.push(
            CelestialBody::new(
                Vec3::new(3.0, 0.0, 0.0),
                0.5,
                ShaderType::RockyPlanet,
            )
            .with_orbit(3.0, 0.5)
            .with_rotation_speed(Vec3::new(0.0, 0.5, 0.0))
        );
        // Órbita del planeta rocoso - Blanco brillante
        orbits.push(OrbitRing::new(Vec3::new(0.0, 0.0, 0.0), 3.0, 0xFFFFFF));
        
        // Luna del planeta rocoso
        bodies.push(
            CelestialBody::new(
                Vec3::new(3.8, 0.0, 0.0),
                0.15,
                ShaderType::Moon,
            )
            .with_orbit(0.8, 1.2)
            .with_rotation_speed(Vec3::new(0.0, 0.3, 0.0))
        );
        
        // Gigante gaseoso (tipo Júpiter)
        bodies.push(
            CelestialBody::new(
                Vec3::new(6.0, 0.0, 0.0),
                0.8, 
                ShaderType::GasGiant,
            )
            .with_orbit(6.0, 0.25)
            .with_rotation_speed(Vec3::new(0.0, 0.8, 0.0))
        );
        // Órbita del gigante gaseoso - Blanco brillante
        orbits.push(OrbitRing::new(Vec3::new(0.0, 0.0, 0.0), 6.0, 0xFFFFFF));
        
        // Skybox - DESHABILITADO temporalmente para mejor performance
        let skybox = CelestialBody::new(
            Vec3::new(0.0, 0.0, 0.0),
            50.0,
            ShaderType::Starfield,
        );

        RenderContext {
            framebuffer: Framebuffer::new(width, height),
            camera: Camera::new(
                Vec3::new(0.0, 3.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            bodies,
            orbits,
            spaceship: Spaceship::new(),
            current_body_index: 0,
            time: 0.0,
            warp_animation: None,
            skybox,
        }
    }
    
    fn start_warp(&mut self, target_position: Vec3) {
        self.warp_animation = Some(WarpAnimation {
            from: self.camera.center,
            to: target_position,
            progress: 0.0,
            duration: 2.0, // 2 segundos de animación
        });
    }
    
    fn update_warp(&mut self, delta_time: f32) {
        if let Some(ref mut warp) = self.warp_animation {
            warp.progress += delta_time / warp.duration;
            
            if warp.progress >= 1.0 {
                self.camera.center = warp.to;
                self.warp_animation = None;
            } else {
                // Interpolación suave (ease in-out)
                let t = warp.progress;
                let smooth_t = t * t * (3.0 - 2.0 * t);
                
                self.camera.center = warp.from + (warp.to - warp.from) * smooth_t;
                
                // Zoom out durante el warp
                let zoom_factor = 1.0 + (t * (1.0 - t) * 4.0) * 5.0;
                let direction = (self.camera.center - self.camera.eye).normalize();
                let base_distance = 8.0;
                self.camera.eye = self.camera.center - direction * base_distance * zoom_factor;
            }
        }
    }
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,   1.0, 0.0,
        0.0,    0.0,   0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(camera: &Camera) -> Mat4 {
    look_at(&camera.eye, &camera.center, &camera.up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_type: &ShaderType,
) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
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

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            let color = shaded_color.to_hex();
            
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_orbit_lines(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    color: u32,
) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
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

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage - color con efecto de brillo sutil
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        
        if x < framebuffer.width && y < framebuffer.height {
            // Extraer componentes RGB del color base
            let r = ((color >> 16) & 0xFF) as f32;
            let g = ((color >> 8) & 0xFF) as f32;
            let b = (color & 0xFF) as f32;
            
            // Reducir brillo para efecto más minimalista y sutil
            let brightness = 0.5; // Factor de brillo reducido para look minimalista
            let final_r = (r * brightness).min(255.0) as u8;
            let final_g = (g * brightness).min(255.0) as u8;
            let final_b = (b * brightness).min(255.0) as u8;
            
            let final_color = ((final_r as u32) << 16) | ((final_g as u32) << 8) | (final_b as u32);
            
            framebuffer.set_current_color(final_color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

// Convert Model to Vec<Vertex>
fn convert_model_to_vertices(model: &Model) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    // Calculate normals for each face
    for face in &model.faces {
        let v0 = model.vertices[face[0]];
        let v1 = model.vertices[face[1]];
        let v2 = model.vertices[face[2]];
        
        // Calculate face normal
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = nalgebra_glm::normalize(&nalgebra_glm::cross(&edge1, &edge2));
        
        // Add vertices for this face
        vertices.push(Vertex::new(v0, normal, nalgebra_glm::Vec2::new(0.0, 0.0)));
        vertices.push(Vertex::new(v1, normal, nalgebra_glm::Vec2::new(1.0, 0.0)));
        vertices.push(Vertex::new(v2, normal, nalgebra_glm::Vec2::new(0.5, 1.0)));
    }
    
    vertices
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 600;  // Aumentado para mejor calidad visual
    let framebuffer_height = 450;  // Aumentado para mejor calidad visual
    let frame_delay = Duration::from_millis(16);

    let mut window = Window::new(
        "Space Renderer - Solar System",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    let mut context = RenderContext::new(framebuffer_width, framebuffer_height);
    context.framebuffer.set_background_color(0x000011);

    // Use optimized procedural sphere instead of loading from file
    // 20 segments x 15 rings = much better performance than the huge .obj file
    let vertex_arrays = create_sphere(1.0, 20, 15);
    
    println!("✅ Using optimized sphere: {} vertices", vertex_arrays.len());

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    let mut last_frame_time = std::time::Instant::now();

    println!("Controls:");
    println!("🎮 Camera:");
    println!("  Arrow Keys: Orbit camera");
    println!("  W/S: Zoom in/out");
    println!("  Q/E: Move up/down (3D movement)");
    // println!("  C: Toggle camera mode (Orbital/First Person)"); // DESHABILITADO
    println!("🚀 Spaceship:");
    println!("  A/D: Rotate spaceship left/right");
    println!("  Shift: Thrust forward");
    println!("🎯 Focus (with warp animation):");
    println!("  1: Focus on Sun");
    println!("  2: Focus on Rocky Planet");
    println!("  3: Focus on Moon");
    println!("  4: Focus on Gas Giant");
    println!("  5: Focus on Spaceship");
    println!("⚙️  Controls:");
    println!("  Space: Toggle orbit animation");
    println!("  O: Toggle orbit lines visibility");
    println!("  ESC: Exit");

    let mut orbit_enabled = true;
    let mut show_orbits = true; // Habilitadas por defecto para mejor visualización

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;

        // Handle input
        handle_input(&window, &mut context, &mut orbit_enabled, &mut show_orbits);
        
        // Update warp animation
        context.update_warp(delta_time);
        
        // DESHABILITADO: Modo primera persona causa lag
        // if matches!(context.camera.mode, CameraMode::FirstPerson) {
        //     context.camera.update_first_person(context.spaceship.position, context.spaceship.rotation);
        // }

        // Update bodies
        if orbit_enabled {
            context.time += delta_time;
            
            let planet_pos = if context.bodies.len() > 1 {
                context.bodies[1].position
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };
            
            for i in 0..context.bodies.len() {
                match i {
                    0 => {
                        let rotation_speed = context.bodies[i].rotation_speed;
                        context.bodies[i].rotation += rotation_speed * delta_time;
                        context.bodies[i].time += delta_time;
                    },
                    1 => {
                        context.bodies[i].update(delta_time);
                    },
                    2 => {
                        let orbit_speed = context.bodies[i].orbit_speed;
                        let orbit_radius = context.bodies[i].orbit_radius;
                        let rotation_speed = context.bodies[i].rotation_speed;
                        
                        context.bodies[i].orbit_angle += orbit_speed * delta_time;
                        context.bodies[i].position.x = planet_pos.x + context.bodies[i].orbit_angle.cos() * orbit_radius;
                        context.bodies[i].position.z = planet_pos.z + context.bodies[i].orbit_angle.sin() * orbit_radius;
                        context.bodies[i].rotation += rotation_speed * delta_time;
                        context.bodies[i].time += delta_time;
                    },
                    _ => {
                        context.bodies[i].update(delta_time);
                    }
                }
            }
        }
        
        // Update spaceship
        context.spaceship.update(delta_time);
        
        // Check collisions
        for body in &context.bodies {
            if context.spaceship.check_collision(body.position, body.scale) {
                context.spaceship.handle_collision(body.position);
                println!("⚠️  Collision detected!");
            }
        }

        context.framebuffer.clear();

        let view_matrix = create_view_matrix(&context.camera);

        // Render all bodies with LOD (Level of Detail)
        for body in &context.bodies {
            // Calcular distancia a la cámara para LOD
            let distance = (body.position - context.camera.eye).magnitude();
            
            // Solo renderizar si está relativamente cerca (culling simple)
            if distance > 50.0 {
                continue; // Skip si está muy lejos
            }
            
            let model_matrix = create_model_matrix(
                body.position,
                body.scale,
                body.rotation,
            );

            let uniforms = Uniforms::new(
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                body.time,
            );

            render(
                &mut context.framebuffer,
                &uniforms,
                &vertex_arrays,
                &body.shader_type,
            );
        }

        // Render spaceship
        let spaceship_model_matrix = create_model_matrix(
            context.spaceship.position,
            context.spaceship.scale,
            context.spaceship.rotation,
        );

        let spaceship_uniforms = Uniforms::new(
            spaceship_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            context.time,
        );

        render(
            &mut context.framebuffer,
            &spaceship_uniforms,
            &context.spaceship.vertices,
            &ShaderType::Ship,
        );
        
        // Render orbit rings if enabled (render last so they're on top)
        if show_orbits {
            // Solo renderizar órbitas si no estamos en primera persona
            if !matches!(context.camera.mode, CameraMode::FirstPerson) {
                for orbit_ring in &context.orbits {
                    let orbit_vertices = orbit_ring.get_vertices();
                    let orbit_model_matrix = create_model_matrix(
                        Vec3::new(0.0, 0.0, 0.0),
                        1.0,
                        Vec3::new(0.0, 0.0, 0.0),
                    );
                    
                    let orbit_uniforms = Uniforms::new(
                        orbit_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        context.time,
                    );
                    
                    // Render órbitas con un shader personalizado (líneas semi-transparentes)
                    render_orbit_lines(
                        &mut context.framebuffer,
                        &orbit_uniforms,
                        &orbit_vertices,
                        orbit_ring.color,
                    );
                }
            }
        }

        window
            .update_with_buffer(
                &context.framebuffer.buffer,
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, context: &mut RenderContext, orbit_enabled: &mut bool, show_orbits: &mut bool) {
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.3; // Reducido para zoom más suave
    let move_speed = 0.2; // Reducido para movimiento más suave

    // Camera controls - solo si no estamos en modo primera persona
    // NOTA: Modo primera persona deshabilitado por performance
    if !matches!(context.camera.mode, CameraMode::FirstPerson) {
        // Camera orbit
        if window.is_key_down(Key::Left) {
            context.camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            context.camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            context.camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            context.camera.orbit(0.0, rotation_speed);
        }

        // Camera zoom (más suave y con throttling)
        if window.is_key_down(Key::W) {
            context.camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::S) {
            context.camera.zoom(-zoom_speed);
        }
        
        // 3D Movement - Up/Down (más suave)
        if window.is_key_down(Key::Q) {
            context.camera.move_up_down(move_speed);
        }
        if window.is_key_down(Key::E) {
            context.camera.move_up_down(-move_speed);
        }
    }
    
    // DESHABILITADO: Toggle camera mode por performance
    // if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
    //     match context.camera.mode {
    //         CameraMode::Orbital => {
    //             context.camera.set_mode(CameraMode::FirstPerson);
    //             println!("📷 Camera Mode: First Person (Following Spaceship)");
    //         }
    //         CameraMode::FirstPerson => {
    //             context.camera.set_mode(CameraMode::Orbital);
    //             println!("📷 Camera Mode: Orbital");
    //         }
    //         _ => {}
    //     }
    // }

    // Focus with warp animation
    if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
        context.current_body_index = 0;
        context.start_warp(context.bodies[0].position);
        context.camera.set_mode(CameraMode::Orbital);
        println!("🎯 Warping to: Sun");
    }
    if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
        context.current_body_index = 1;
        context.start_warp(context.bodies[1].position);
        context.camera.set_mode(CameraMode::Orbital);
        println!("🎯 Warping to: Rocky Planet");
    }
    if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
        context.current_body_index = 2;
        context.start_warp(context.bodies[2].position);
        context.camera.set_mode(CameraMode::Orbital);
        println!("🎯 Warping to: Moon");
    }
    if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
        context.current_body_index = 3;
        context.start_warp(context.bodies[3].position);
        context.camera.set_mode(CameraMode::Orbital);
        println!("🎯 Warping to: Gas Giant");
    }
    if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
        context.start_warp(context.spaceship.position);
        // Modo primera persona deshabilitado por performance
        // context.camera.set_mode(CameraMode::FirstPerson);
        println!("🎯 Warping to: Spaceship");
    }

    // Toggle orbit animation
    if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
        *orbit_enabled = !*orbit_enabled;
        println!("🔄 Orbit animation: {}", if *orbit_enabled { "ON" } else { "OFF" });
    }
    
    // Toggle orbit lines visibility
    if window.is_key_pressed(Key::O, minifb::KeyRepeat::No) {
        *show_orbits = !*show_orbits;
        println!("⭕ Orbit lines: {}", if *show_orbits { "VISIBLE" } else { "HIDDEN" });
    }

    // Spaceship controls
    let delta_time = 0.016;
    
    if window.is_key_down(Key::A) {
        context.spaceship.rotate(-2.0 * delta_time);
    }
    if window.is_key_down(Key::D) {
        context.spaceship.rotate(2.0 * delta_time);
    }
    if window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
        context.spaceship.apply_thrust(5.0 * delta_time);
    }
}