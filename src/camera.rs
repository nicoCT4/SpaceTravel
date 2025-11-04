use nalgebra_glm::{Vec3, Mat4, look_at, perspective, cross, normalize};
use std::f32::consts::PI;

pub struct Camera {
   pub eye: Vec3,
   pub center: Vec3,
   pub up: Vec3,
   pub has_changed: bool,
   pub free_look: bool,
   pub pitch: f32,
   pub yaw: f32,
   pub movement_speed: f32,
}

impl Camera {
   pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
      Camera {
         eye,
         center,
         up,
         has_changed: true,
         free_look: false,
         pitch: 0.0,
         yaw: 0.0,
         movement_speed: 0.5,
      }
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
      if self.free_look {
         self.free_look_rotate(delta_yaw, delta_pitch);
      } else {
         self.orbit_around_target(delta_yaw, delta_pitch);
      }
   }

   fn orbit_around_target(&mut self, delta_yaw: f32, delta_pitch: f32) {
      let radius_vector = self.eye - self.center;
      let radius = radius_vector.magnitude();

      let current_yaw = radius_vector.z.atan2(radius_vector.x);
      let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
      let current_pitch = (-radius_vector.y).atan2(radius_xz);

      let new_yaw = current_yaw + delta_yaw;
      let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

      let new_eye = self.center + Vec3::new(
         radius * new_pitch.cos() * new_yaw.cos(),
         -radius * new_pitch.sin(),
         radius * new_pitch.cos() * new_yaw.sin()
      );

      self.eye = new_eye;
      self.has_changed = true;
   }

   fn free_look_rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
      self.yaw += delta_yaw;
      self.pitch += delta_pitch;
      
      self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
      
      self.update_free_look_center();
      self.has_changed = true;
   }

   pub fn zoom(&mut self, delta: f32) {
      if self.free_look {
         self.move_forward(delta);
      } else {
         let direction = (self.center - self.eye).normalize();
         self.eye += direction * delta;
         self.has_changed = true;
      }
   }

   pub fn toggle_free_look(&mut self) {
      self.free_look = !self.free_look;
      
      if self.free_look {
         let forward = normalize(&(self.center - self.eye));
         self.yaw = forward.z.atan2(forward.x);
         self.pitch = forward.y.asin();
         self.center = self.eye + forward;
      }
      
      self.has_changed = true;
   }

   pub fn move_forward(&mut self, delta: f32) {
      let forward = self.get_forward_vector();
      self.eye += forward * delta * self.movement_speed;
      if self.free_look {
         self.center = self.eye + forward;
      }
      self.has_changed = true;
   }

   pub fn move_right(&mut self, delta: f32) {
      let right = self.get_right_vector();
      self.eye += right * delta * self.movement_speed;
      if self.free_look {
         let forward = self.get_forward_vector();
         self.center = self.eye + forward;
      }
      self.has_changed = true;
   }

   pub fn move_up(&mut self, delta: f32) {
      let up = Vec3::new(0.0, 1.0, 0.0); // World up
      self.eye += up * delta * self.movement_speed;
      if self.free_look {
         let forward = self.get_forward_vector();
         self.center = self.eye + forward;
      }
      self.has_changed = true;
   }

   pub fn get_forward_vector(&self) -> Vec3 {
      if self.free_look {
         let x = self.pitch.cos() * self.yaw.cos();
         let y = self.pitch.sin();
         let z = self.pitch.cos() * self.yaw.sin();
         normalize(&Vec3::new(x, y, z))
      } else {
         normalize(&(self.center - self.eye))
      }
   }

   pub fn get_right_vector(&self) -> Vec3 {
      let forward = self.get_forward_vector();
      normalize(&cross(&forward, &self.up))
   }

   fn update_free_look_center(&mut self) {
      let forward = self.get_forward_vector();
      self.center = self.eye + forward;
   }

   pub fn set_target(&mut self, target: Vec3) {
      self.center = target;
      if !self.free_look {
         // Recalculate position to maintain distance
         let direction = normalize(&(self.eye - self.center));
         let distance = (self.eye - self.center).magnitude();
         self.eye = self.center + direction * distance;
      }
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