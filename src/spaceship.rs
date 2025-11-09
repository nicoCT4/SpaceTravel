use nalgebra_glm::Vec3;
use crate::vertex::Vertex;
use crate::obj_loader::Model;
use std::f32::consts::PI;

pub struct Spaceship {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub velocity: Vec3,
    pub vertices: Vec<Vertex>,
    pub is_loaded: bool,
}

impl Spaceship {
    pub fn new() -> Self {
        println!("🚀 Loading spaceship model...");
        
        // Try to load the spaceship model
        let (vertices, is_loaded) = match Model::load_obj("assets/models/NavePrototipo2.obj") {
            Ok(model) => {
                println!("✅ Spaceship model loaded successfully!");
                println!("   - Vertices: {}", model.vertices.len());
                println!("   - Faces: {}", model.faces.len());
                (Self::convert_model_to_vertices(model), true)
            }
            Err(e) => {
                println!("⚠️  Could not load spaceship model: {}", e);
                println!("   Using fallback cube model");
                (Self::create_fallback_model(), false)
            }
        };

        Spaceship {
            position: Vec3::new(2.0, 0.0, 2.0), // Start near the scene
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.3,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            vertices,
            is_loaded,
        }
    }

    // Create a simple spaceship-like shape as fallback
    fn create_fallback_model() -> Vec<Vertex> {
        let mut vertices = Vec::new();

        // Simple spaceship shape - pointed front, wider back
        let ship_vertices = [
            // Front point
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(-0.2, -0.1, -0.3),
            Vec3::new(0.2, -0.1, -0.3),
            
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.2, -0.1, -0.3),
            Vec3::new(0.0, 0.1, -0.3),
            
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.0, 0.1, -0.3),
            Vec3::new(-0.2, -0.1, -0.3),
            
            // Back triangles (wings)
            Vec3::new(-0.2, -0.1, -0.3),
            Vec3::new(-0.4, 0.0, -0.5),
            Vec3::new(0.0, 0.1, -0.3),
            
            Vec3::new(0.2, -0.1, -0.3),
            Vec3::new(0.0, 0.1, -0.3),
            Vec3::new(0.4, 0.0, -0.5),
        ];

        for pos in &ship_vertices {
            vertices.push(Vertex::new(
                *pos,
                Vec3::new(0.0, 1.0, 0.0), // Normal pointing up
                nalgebra_glm::Vec2::new(0.0, 0.0),
            ));
        }

        vertices
    }

    // Convert Model to Vec<Vertex>
    fn convert_model_to_vertices(model: Model) -> Vec<Vertex> {
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

    pub fn update(&mut self, delta_time: f32) {
        // Update position based on velocity
        self.position += self.velocity * delta_time;
        
        // Apply some drag
        self.velocity *= 0.95;
        
        // Keep within bounds (simple boundary check)
        let boundary = 10.0;
        if self.position.x.abs() > boundary || self.position.z.abs() > boundary {
            self.velocity *= -0.5; // Bounce back
        }
    }
    
    pub fn check_collision(&self, body_position: Vec3, body_radius: f32) -> bool {
        let distance = (self.position - body_position).magnitude();
        distance < (body_radius + 0.3) // 0.3 es el radio aproximado de la nave
    }
    
    pub fn handle_collision(&mut self, body_position: Vec3) {
        // Empujar la nave lejos del cuerpo
        let direction = (self.position - body_position).normalize();
        self.velocity = direction * 2.0; // Rebote
        self.position += direction * 0.5; // Separar inmediatamente
    }

    pub fn apply_thrust(&mut self, thrust: f32) {
        let forward = Vec3::new(
            self.rotation.y.sin(),
            0.0,
            self.rotation.y.cos(),
        );
        self.velocity += forward * thrust;
    }

    pub fn rotate(&mut self, delta_y: f32) {
        self.rotation.y += delta_y;
        // Keep rotation in [0, 2π] range
        if self.rotation.y > 2.0 * PI {
            self.rotation.y -= 2.0 * PI;
        } else if self.rotation.y < 0.0 {
            self.rotation.y += 2.0 * PI;
        }
    }

    pub fn get_model_name(&self) -> &str {
        if self.is_loaded {
            "NavePrototipo2"
        } else {
            "Fallback Model"
        }
    }
}
