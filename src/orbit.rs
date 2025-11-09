use nalgebra_glm::Vec3;
use crate::vertex::Vertex;
use std::f32::consts::PI;

pub struct OrbitRing {
    pub center: Vec3,
    pub radius: f32,
    pub segments: usize,
    pub color: u32,
}

impl OrbitRing {
    pub fn new(center: Vec3, radius: f32, color: u32) -> Self {
        OrbitRing {
            center,
            radius,
            segments: 128, // Aumentado para órbitas más suaves
            color,
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        
        // Crear triángulos muy pequeños para cada punto de la órbita
        for i in 0..self.segments {
            let angle = (i as f32 / self.segments as f32) * 2.0 * PI;
            
            let x = self.center.x + self.radius * angle.cos();
            let z = self.center.z + self.radius * angle.sin();
            let y = self.center.y;
            
            // Crear un triángulo muy pequeño (casi un punto)
            let size = 0.01;
            let v1 = Vec3::new(x - size, y, z);
            let v2 = Vec3::new(x + size, y, z);
            let v3 = Vec3::new(x, y, z + size);
            
            vertices.push(Vertex::new(v1, normal, nalgebra_glm::Vec2::new(0.0, 0.0)));
            vertices.push(Vertex::new(v2, normal, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(v3, normal, nalgebra_glm::Vec2::new(0.5, 1.0)));
        }
        
        vertices
    }
}
