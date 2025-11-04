use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use std::f32::consts::PI;

pub fn create_sphere(radius: f32, segments: u32, rings: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    for ring in 0..=rings {
        let theta = PI * ring as f32 / rings as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        
        for segment in 0..=segments {
            let phi = 2.0 * PI * segment as f32 / segments as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            
            let x = sin_theta * cos_phi;
            let y = cos_theta;
            let z = sin_theta * sin_phi;
            
            let position = Vec3::new(x * radius, y * radius, z * radius);
            let normal = Vec3::new(x, y, z);
            let tex_coords = Vec2::new(
                segment as f32 / segments as f32,
                ring as f32 / rings as f32,
            );
            
            vertices.push(Vertex::new(position, normal, tex_coords));
        }
    }
    
    // Convert to triangles
    let mut triangles = Vec::new();
    
    for ring in 0..rings {
        for segment in 0..segments {
            let current = ring * (segments + 1) + segment;
            let next = current + segments + 1;
            
            // First triangle
            triangles.push(vertices[current as usize].clone());
            triangles.push(vertices[(next + 1) as usize].clone());
            triangles.push(vertices[(current + 1) as usize].clone());
            
            // Second triangle
            triangles.push(vertices[current as usize].clone());
            triangles.push(vertices[next as usize].clone());
            triangles.push(vertices[(next + 1) as usize].clone());
        }
    }
    
    triangles
}
