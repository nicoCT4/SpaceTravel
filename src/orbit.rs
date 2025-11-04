use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use std::f32::consts::PI;

pub struct Orbit {
    pub center: Vec3,
    pub radius: f32,
    pub segments: usize,
    pub vertices: Vec<Vertex>,
}

impl Orbit {
    pub fn new(center: Vec3, radius: f32, segments: usize) -> Self {
        let mut vertices = Vec::new();
        
        // Create a circle of vertices
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * PI;
            let x = center.x + radius * angle.cos();
            let z = center.z + radius * angle.sin();
            let y = center.y; // Keep orbit on the ecliptic plane
            
            vertices.push(Vertex::new(
                Vec3::new(x, y, z),
                Vec3::new(0.0, 1.0, 0.0), // Normal pointing up
                Vec2::new(0.0, 0.0) // Tex coords
            ));
        }
        
        Orbit {
            center,
            radius,
            segments,
            vertices,
        }
    }
    
    pub fn get_line_vertices(&self) -> Vec<(Vertex, Vertex)> {
        let mut lines = Vec::new();
        
        for i in 0..self.segments {
            let current = self.vertices[i].clone();
            let next = self.vertices[(i + 1) % self.segments].clone();
            lines.push((current, next));
        }
        
        lines
    }
}
