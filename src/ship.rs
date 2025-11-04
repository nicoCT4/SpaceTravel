use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use crate::obj::Obj;

pub struct Ship {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub velocity: Vec3,
    pub vertices: Vec<Vertex>,
}

impl Ship {
    pub fn new() -> Self {
        // Try to load the ship model, fall back to a simple cube if not found
        let vertices = match Obj::load("assets/models/NavePrototipo2.obj") {
            Ok(obj) => {
                println!("Ship model loaded successfully!");
                obj.get_vertex_array()
            }
            Err(_) => {
                println!("Ship model not found, using default cube");
                create_cube_vertices()
            }
        };

        Ship {
            position: Vec3::new(0.0, 0.0, 5.0), // Start behind camera
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.3,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            vertices,
        }
    }

    pub fn update(&mut self, camera_position: Vec3, camera_forward: Vec3, delta_time: f32) {
        // Ship follows camera at a fixed distance
        let follow_distance = 2.0;
        let target_position = camera_position - camera_forward * follow_distance;
        
        // Smooth movement towards target position
        let move_speed = 5.0;
        let direction = target_position - self.position;
        self.velocity = direction * move_speed;
        self.position += self.velocity * delta_time;
        
        // Make ship face movement direction
        if self.velocity.magnitude() > 0.1 {
            self.rotation.y = self.velocity.z.atan2(self.velocity.x);
        }
    }

    pub fn check_collision(&self, planet_position: Vec3, planet_radius: f32) -> bool {
        let distance = (self.position - planet_position).magnitude();
        let ship_radius = 0.2; // Approximate ship radius
        distance < (planet_radius + ship_radius)
    }
}

fn create_cube_vertices() -> Vec<Vertex> {
    // Simple cube vertices as fallback
    let positions = [
        // Front face
        Vec3::new(-0.5, -0.5,  0.5),
        Vec3::new( 0.5, -0.5,  0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new(-0.5, -0.5,  0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new(-0.5,  0.5,  0.5),
        
        // Back face
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5,  0.5, -0.5),
        Vec3::new( 0.5,  0.5, -0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new( 0.5,  0.5, -0.5),
        Vec3::new( 0.5, -0.5, -0.5),
        
        // Left face
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, -0.5,  0.5),
        Vec3::new(-0.5,  0.5,  0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5,  0.5,  0.5),
        Vec3::new(-0.5,  0.5, -0.5),
        
        // Right face
        Vec3::new( 0.5, -0.5, -0.5),
        Vec3::new( 0.5,  0.5, -0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new( 0.5, -0.5, -0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new( 0.5, -0.5,  0.5),
        
        // Top face
        Vec3::new(-0.5,  0.5, -0.5),
        Vec3::new(-0.5,  0.5,  0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new(-0.5,  0.5, -0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new( 0.5,  0.5, -0.5),
        
        // Bottom face
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new( 0.5, -0.5, -0.5),
        Vec3::new( 0.5, -0.5,  0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new( 0.5, -0.5,  0.5),
        Vec3::new(-0.5, -0.5,  0.5),
    ];

    positions
        .iter()
        .map(|&pos| Vertex::new(pos, Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0)))
        .collect()
}
