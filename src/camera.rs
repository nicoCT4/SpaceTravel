use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
   Orbital,      // Modo orbital alrededor de un punto
   FirstPerson,  // Modo primera persona desde la nave
   Free,         // Modo libre (vuelo libre)
}

pub struct Camera {
   pub eye: Vec3,
   pub center: Vec3,
   pub up: Vec3,
   pub has_changed: bool,
   pub mode: CameraMode,
   pub yaw: f32,
   pub pitch: f32,
}

impl Camera {
   pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
      Camera {
         eye,
         center,
         up,
         has_changed: true,
         mode: CameraMode::Orbital,
         yaw: 0.0,
         pitch: 0.0,
      }
   }

   pub fn set_mode(&mut self, mode: CameraMode) {
      self.mode = mode;
      self.has_changed = true;
   }

   pub fn update_first_person(&mut self, ship_position: Vec3, ship_rotation: Vec3) {
      // Posicionar cámara ligeramente detrás y arriba de la nave
      let offset = Vec3::new(0.0, 0.5, -2.0);
      
      // Rotar el offset según la rotación de la nave
      let cos_y = ship_rotation.y.cos();
      let sin_y = ship_rotation.y.sin();
      
      let rotated_offset = Vec3::new(
         offset.x * cos_y - offset.z * sin_y,
         offset.y,
         offset.x * sin_y + offset.z * cos_y,
      );
      
      self.eye = ship_position + rotated_offset;
      
      // Mirar hacia adelante de la nave
      let forward = Vec3::new(sin_y, 0.0, cos_y);
      self.center = ship_position + forward * 5.0;
      
      self.has_changed = true;
   }

   pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
      let forward = (self.center - self.eye).normalize();
      let right = forward.cross(&self.up).normalize();
      let up = right.cross(&forward).normalize();

      let rotated = 
         vector.x * right +
         vector.y * up -
         vector.z * forward;

      rotated.normalize()
   }

   pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
      let radius_vector = self.eye - self.center;
      let radius = radius_vector.magnitude();

      let current_yaw = radius_vector.z.atan2(radius_vector.x);
      let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
      let current_pitch = (-radius_vector.y).atan2(radius_xz);

      let new_yaw = current_yaw + delta_yaw;
      // Permitir movimiento 3D completo - más rango en pitch
      let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

      let new_eye = self.center + Vec3::new(
         radius * new_pitch.cos() * new_yaw.cos(),
         -radius * new_pitch.sin(),
         radius * new_pitch.cos() * new_yaw.sin()
      );

      self.eye = new_eye;
      self.has_changed = true;
   }
   
   pub fn move_up_down(&mut self, delta: f32) {
      // Movimiento vertical libre (arriba/abajo del plano eclíptico)
      self.eye.y += delta;
      self.center.y += delta;
      self.has_changed = true;
   }
   
   pub fn move_forward_back(&mut self, delta: f32) {
      let direction = (self.center - self.eye).normalize();
      self.eye += direction * delta;
      self.center += direction * delta;
      self.has_changed = true;
   }
   
   pub fn move_left_right(&mut self, delta: f32) {
      let forward = (self.center - self.eye).normalize();
      let right = forward.cross(&self.up).normalize();
      self.eye += right * delta;
      self.center += right * delta;
      self.has_changed = true;
   }

   pub fn zoom(&mut self, delta: f32) {
      let direction = (self.center - self.eye).normalize();
      self.eye += direction * delta;
      self.has_changed = true;
   }
}

pub fn create_view_matrix(camera: &Camera) -> Mat4 {
   look_at(&camera.eye, &camera.center, &camera.up)
}

pub fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
   let fov = 45.0 * PI / 180.0;
   let aspect_ratio = window_width / window_height;
   let near = 0.1;
   let far = 1000.0;

   perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
   Mat4::new(
      width / 2.0, 0.0, 0.0, width / 2.0,
      0.0, -height / 2.0, 0.0, height / 2.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0
   )
}