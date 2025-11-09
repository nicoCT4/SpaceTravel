use nalgebra_glm::Vec3;
use crate::vertex::Vertex;
use std::f32::consts::PI;

pub struct OrbitRing {
    pub center: Vec3,
    pub radius: f32,
    pub segments: usize,
    pub color: u32,
    pub line_width: f32,
    pub dashed: bool,
}

impl OrbitRing {
    pub fn new(center: Vec3, radius: f32, color: u32) -> Self {
        OrbitRing {
            center,
            radius,
            segments: 200, // Más segmentos para líneas más suaves
            color,
            line_width: 0.015, // Grosor reducido para look minimalista
            dashed: true, // Efecto de línea punteada
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        
        // Crear línea con grosor usando quad (dos triángulos por segmento)
        for i in 0..self.segments {
            let angle1 = (i as f32 / self.segments as f32) * 2.0 * PI;
            let angle2 = ((i + 1) as f32 / self.segments as f32) * 2.0 * PI;
            
            // Calcular si este segmento debe ser visible (para efecto punteado)
            let dash_pattern = (i / 8) % 3; // Patrón: 8 visible, 8 gap, 8 visible, 8 gap más largo
            if self.dashed && dash_pattern == 2 {
                continue; // Saltar algunos segmentos para crear efecto punteado
            }
            
            // Puntos en el círculo
            let x1 = self.center.x + self.radius * angle1.cos();
            let z1 = self.center.z + self.radius * angle1.sin();
            let x2 = self.center.x + self.radius * angle2.cos();
            let z2 = self.center.z + self.radius * angle2.sin();
            let y = self.center.y;
            
            // Calcular perpendicular para dar grosor a la línea
            let dx = x2 - x1;
            let dz = z2 - z1;
            let length = (dx * dx + dz * dz).sqrt();
            let perpx = -dz / length * self.line_width;
            let perpz = dx / length * self.line_width;
            
            // Crear un quad (rectángulo) con dos triángulos
            let v1 = Vec3::new(x1 + perpx, y, z1 + perpz);
            let v2 = Vec3::new(x1 - perpx, y, z1 - perpz);
            let v3 = Vec3::new(x2 - perpx, y, z2 - perpz);
            let v4 = Vec3::new(x2 + perpx, y, z2 + perpz);
            
            // Primer triángulo del quad
            vertices.push(Vertex::new(v1, normal, nalgebra_glm::Vec2::new(0.0, 0.0)));
            vertices.push(Vertex::new(v2, normal, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(v3, normal, nalgebra_glm::Vec2::new(0.5, 1.0)));
            
            // Segundo triángulo del quad
            vertices.push(Vertex::new(v1, normal, nalgebra_glm::Vec2::new(0.0, 0.0)));
            vertices.push(Vertex::new(v3, normal, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(v4, normal, nalgebra_glm::Vec2::new(0.5, 1.0)));
        }
        
        vertices
    }
}
