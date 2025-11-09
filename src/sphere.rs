use nalgebra_glm::Vec3;
use crate::vertex::Vertex;
use std::f32::consts::PI;

pub fn create_sphere(radius: f32, segments: usize, rings: usize) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    for ring in 0..rings {
        let theta1 = (ring as f32 / rings as f32) * PI;
        let theta2 = ((ring + 1) as f32 / rings as f32) * PI;
        
        for segment in 0..segments {
            let phi1 = (segment as f32 / segments as f32) * 2.0 * PI;
            let phi2 = ((segment + 1) as f32 / segments as f32) * 2.0 * PI;
            
            // Calcular los 4 vértices del quad
            let v1 = spherical_to_cartesian(radius, theta1, phi1);
            let v2 = spherical_to_cartesian(radius, theta1, phi2);
            let v3 = spherical_to_cartesian(radius, theta2, phi1);
            let v4 = spherical_to_cartesian(radius, theta2, phi2);
            
            // Normales apuntan desde el centro
            let n1 = v1.normalize();
            let n2 = v2.normalize();
            let n3 = v3.normalize();
            let n4 = v4.normalize();
            
            // Primer triángulo del quad
            vertices.push(Vertex::new(v1, n1, nalgebra_glm::Vec2::new(0.0, 0.0)));
            vertices.push(Vertex::new(v2, n2, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(v3, n3, nalgebra_glm::Vec2::new(0.0, 1.0)));
            
            // Segundo triángulo del quad
            vertices.push(Vertex::new(v2, n2, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(v4, n4, nalgebra_glm::Vec2::new(1.0, 1.0)));
            vertices.push(Vertex::new(v3, n3, nalgebra_glm::Vec2::new(0.0, 1.0)));
        }
    }
    
    vertices
}

fn spherical_to_cartesian(radius: f32, theta: f32, phi: f32) -> Vec3 {
    let x = radius * theta.sin() * phi.cos();
    let y = radius * theta.cos();
    let z = radius * theta.sin() * phi.sin();
    Vec3::new(x, y, z)
}
